use std::env;
use log::{error};
use reqwest::Client;
use serde::Deserialize;
use teloxide::Bot;
use teloxide::prelude::Message;
use teloxide::requests::{Requester, ResponseResult};
#[derive(Debug, Deserialize)]
struct ExchangerateResponse {
    success: bool,
    result: Option<f64>,
    error: Option<ErrorData>,
}
#[derive(Debug, Deserialize)]
struct ErrorData {
    code: i32,
    info: String,
}

//
// ## API Provider
// [exchangerate.host](https://exchangerate.host)
//
// ## API Documentation
// [https://exchangerate.host/documentation](https://exchangerate.host/documentation)
//
// ## API Endpoint
// `/convert` – Converts an amount from one currency to another.
//
// ## Required Parameters
// - `access_key`: Your API key (required for authentication).
// - `from`: The base currency (e.g., "USD").
// - `to`: The target currency (e.g., "EUR").
// - `amount`: The amount to convert (e.g., 100.0).
//
// ## Terms of Use
// - An API key is required. You can obtain one by signing up at [exchangerate.host](https://exchangerate.host).
// - The free plan allows up to **100 requests per month**.
// - Higher tiers are available for increased usage and additional features.
// - Usage beyond the free tier may require upgrading to a paid plan.
// For more about pricing, visit: [https://exchangerate.host/pricing](https://exchangerate.host/pricing)
pub async fn handle_currency(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    let api_key = match env::var("EXCHANGERATE_TOKEN") {
        Ok(key) => key,
        Err(_) => {
            error!("EXCHANGERATE_TOKEN environment variable is not set");
            return Ok(());
        }
    };

    let parts = match parse_currency_args(text) {
        Ok(parts) => parts,
        Err(_) => {
            bot.send_message(
                msg.chat.id,
                "Usage: /currency <amount> <from> <to>\nExample: /currency 100 USD EUR",
            )
            .await?;
            return Ok(());
        }
    };

    let url = format!(
        "https://api.exchangerate.host/convert?access_key={}&from={}&to={}&amount={}",
        api_key, parts[1], parts[2], parts[0]
    );

    let client = Client::new();
    let response = client.get(&url).send().await;

    let reply = match response {
        Ok(resp) => match resp.json::<ExchangerateResponse>().await {
            Ok(data) => {
                if data.success {
                    format!(
                        "🔄 {} {} = {} {}",
                        parts[0],
                        parts[1],
                        data.result.unwrap_or(0.0),
                        parts[2]
                    )
                } else {
                    let err_msg = match data.error {
                        None => "⚠️ Failed to fetch currency data. Try again later.".to_string(),
                        Some(msg) => {
                            if msg.code >= 100 && msg.code < 200 {
                                error!("{}", msg.info);
                                "⚠️ Failed to fetch currency data. Try again later.".to_string()
                            } else {
                                msg.info
                            }
                        }
                    };
                    err_msg
                }
            }
            Err(_) => "❌ Couldn't parse currency data.".to_string(),
        },
        Err(_) => "⚠️ Failed to fetch currency data. Try again later.".to_string(),
    };

    bot.send_message(msg.chat.id, reply).await?;
    Ok(())
}

fn parse_currency_args(args: String) -> Result<Vec<String>, ()> {
    let parts: Vec<String> = args.split_whitespace().map(String::from).collect();
    if parts.len() != 3 {
        return Err(());
    }

    if parts[0].parse::<f64>().is_err() {
        return Err(());
    }

    Ok(parts)
}

#[cfg(test)]
mod tests {
    use crate::commands::currency::parse_currency_args;

    #[test]
    fn test_parse_currency_args() {
        let args = "100 USD EUR".to_string();
        assert_eq!(
            parse_currency_args(args),
            Ok(vec![
                "100".to_string(),
                "USD".to_string(),
                "EUR".to_string()
            ])
        );

        let args = "USD EUR".to_string();
        let result = parse_currency_args(args);
        assert!(result.is_err());

        let args = "Invalid USD EUR".to_string();
        let result = parse_currency_args(args);
        assert!(result.is_err());

        let args = "100 USD EUR Invalid".to_string();
        let result = parse_currency_args(args);
        assert!(result.is_err());
    }
}
