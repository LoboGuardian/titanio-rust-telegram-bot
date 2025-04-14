use teloxide::{prelude::*, types::Message};

// This function sends a message to the user with information about the bot.
pub async fn handle_about(bot: Bot, msg: Message) -> ResponseResult<()> {
    let text = "I'm Titanio 🤖, a Rust-powered Telegram bot 🦀. Built with 💖 and teloxide!";
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}
