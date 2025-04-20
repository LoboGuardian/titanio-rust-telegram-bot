use crate::services::ApiService;
use log::error;
use std::sync::Arc;
use teloxide::{prelude::*, types::Message};

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
pub async fn handle_currency(
    bot: Bot,
    msg: Message,
    text: String,
    api: Arc<ApiService>,
) -> ResponseResult<()> {
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

    let amount: f64 = parts[0].parse().unwrap_or(0.0);
    let from = &parts[1];
    let to = &parts[2];

    let reply = match api.convert_currency(amount, from, to).await {
        Ok(result) => format!("🔄 {} {} = {} {}", amount, from, result, to),
        Err(err) => {
            error!("Currency conversion failed: {}", err);
            format!("❌ Currency conversion failed: {}", err)
        }
    };

    bot.send_message(msg.chat.id, reply).await?;
    Ok(())
}

fn parse_currency_args(args: String) -> Result<Vec<String>, ()> {
    let parts: Vec<String> = args.split_whitespace().map(String::from).collect();
    if parts.len() != 3 || parts[0].parse::<f64>().is_err() {
        return Err(());
    }
    Ok(parts)
}

#[cfg(test)]
mod tests {
    use super::parse_currency_args;

    #[test]
    fn test_parse_currency_args() {
        assert_eq!(
            parse_currency_args("100 USD EUR".to_string()),
            Ok(vec!["100".into(), "USD".into(), "EUR".into()])
        );

        assert!(parse_currency_args("USD EUR".to_string()).is_err());
        assert!(parse_currency_args("abc USD EUR".to_string()).is_err());
        assert!(parse_currency_args("100 USD EUR XYZ".to_string()).is_err());
    }
}
