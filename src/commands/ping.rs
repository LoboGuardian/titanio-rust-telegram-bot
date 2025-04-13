use teloxide::{prelude::*, types::Message};

// This function handles the `/ping` command and sends a "Pong!" message to the user.
pub async fn handle_ping(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "🏓 Pong! The bot is alive!")
        .await?;
    Ok(())
}
