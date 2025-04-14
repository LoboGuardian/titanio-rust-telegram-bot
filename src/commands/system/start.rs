use teloxide::{prelude::*, types::Message};

/// Sends a welcome message when the user starts the bot.
pub async fn handle_start(bot: Bot, msg: Message) -> ResponseResult<()> {
    let welcome = "Welcome! I'm your helpful Rusty titanio bot 🦀!";
    bot.send_message(msg.chat.id, welcome).await?;
    Ok(())
}
