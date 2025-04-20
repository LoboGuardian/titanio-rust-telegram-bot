use teloxide::{prelude::*, types::Message};

/// Handles the `/ping` command by replying with a "Pong!" message.
pub async fn handle_ping(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let response_text = "🏓 Pong! The bot is alive!";

    bot.send_message(chat_id, response_text).await?;

    Ok(())
}
