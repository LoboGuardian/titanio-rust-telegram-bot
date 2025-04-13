use reqwest::Client;
use serde::Deserialize;
use teloxide::{prelude::*, types::Message};

#[derive(Debug, Deserialize)]
struct WeatherData {
    current_condition: Vec<CurrentCondition>,
}

#[derive(Debug, Deserialize)]
struct CurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: String,

    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<WeatherDesc>,
}

#[derive(Debug, Deserialize)]
struct WeatherDesc {
    value: String,
}

pub async fn handle_weather(bot: Bot, msg: Message, city: String) -> ResponseResult<()> {
    let url = format!("https://wttr.in/{}?format=j1", city);

    let client = Client::new();
    let response = client.get(&url).send().await;

    let reply = match response {
        Ok(resp) => match resp.json::<WeatherData>().await {
            Ok(data) => {
                let temp = &data.current_condition[0].temp_c;
                let desc = data.current_condition[0]
                    .weather_desc
                    .get(0)
                    .map(|w| w.value.clone())
                    .unwrap_or_else(|| "unknown".to_string());
                format!("🌤️ Weather in {}: {}°C, {}", city, temp, desc)
            }
            Err(_) => "❌ Couldn't parse weather data.".to_string(),
        },
        Err(_) => "⚠️ Failed to fetch weather. Try again later.".to_string(),
    };

    bot.send_message(msg.chat.id, reply).await?;
    Ok(())
}
