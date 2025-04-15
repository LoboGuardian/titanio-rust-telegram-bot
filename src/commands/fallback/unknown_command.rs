use teloxide::Bot;
use teloxide::prelude::{Message, ResponseResult};
use teloxide::requests::Requester;

pub async fn unrecognized(bot: Bot, msg: Message) -> ResponseResult<()> {
    let text = "🤖 I didn't recognize that command. Type /help to see what I can do!".to_string();
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}
