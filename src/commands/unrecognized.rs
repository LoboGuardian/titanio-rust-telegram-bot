use teloxide::Bot;
use teloxide::prelude::{Message, ResponseResult};
use teloxide::requests::Requester;

pub async fn handle_unrecognized(bot: Bot, msg: Message) -> ResponseResult<()> {
    let text = "Unrecognized input. Use /help for a list of commands.".to_string();
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}
