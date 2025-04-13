use teloxide::{prelude::*, types::Message};

/// Echoes back whatever message the user provides after the /echo command.
pub async fn handle_echo(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, format!("You said: {text}")).await?;
    Ok(())
}
