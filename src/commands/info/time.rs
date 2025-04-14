use chrono::Utc;
// use chrono::Local;
use teloxide::{prelude::*, types::Message};

// This function handles the `/time` command and sends the current time to the user.
pub async fn handle_time(bot: Bot, msg: Message) -> ResponseResult<()> {
    let now = Utc::now();
    // let now = Local::now();
    let formatted = now.format("🕒 %Y-%m-%d %H:%M:%S").to_string();
    bot.send_message(msg.chat.id, format!("Current UTC time: {}", formatted))
        .await?;
    Ok(())
}
