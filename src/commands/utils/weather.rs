use teloxide::{prelude::*, types::Message};
use std::sync::Arc;
use crate::services::ApiService;


pub async fn handle_weather(bot: Bot, msg: Message, city: String, api: Arc<ApiService>) -> ResponseResult<()> {
    if city.trim().is_empty() {
        bot.send_message(msg.chat.id, "⚠️ Please enter a valid city name.").await?;
        return Ok(());
    }

    let reply = match api.get_weather(&city).await {
        Ok(data) => {
            let temp = &data.current_condition[0].temp_c;
            let desc = data.current_condition[0]
                .weather_desc
                .first()
                .map(|w| w.value.clone())
                .unwrap_or_else(|| "unknown".to_string());
            format!("🌤️ Weather in {}: {}°C, {}", city, temp, desc)
        }
        Err(e) => format!("❌ {}", e),
    };

    bot.send_message(msg.chat.id, reply).await?;
    Ok(())
}