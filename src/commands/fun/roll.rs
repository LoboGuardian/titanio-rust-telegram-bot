use teloxide::{prelude::*, types::Message};

// Sends Telegram's animated dice message 🥳
pub async fn handle_roll(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_dice(msg.chat.id).await?;
    Ok(())
}
