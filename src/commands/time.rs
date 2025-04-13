use teloxide::{prelude::*, types::Message};
use chrono::Local;

// This function handles the `/time` command and sends the current time to the user.
pub async fn handle_time(bot: Bot, msg: Message) -> ResponseResult<()> {
    let now = Local::now();
    let formatted = now.format("🕒 %Y-%m-%d %H:%M:%S").to_string();
    bot.send_message(msg.chat.id, format!("Current time: {}", formatted)).await?;
    Ok(())
}