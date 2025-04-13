use teloxide::{prelude::*, types::Message};
use crate::commands::Command;

/// Sends the list of available commands to the user.
pub async fn handle_help(bot: Bot, msg: Message) -> ResponseResult<()> {
    let help_text = Command::descriptions().to_string();
    bot.send_message(msg.chat.id, help_text).await?;
    Ok(())
}
