use teloxide::prelude::*;
use teloxide::requests::Requester;

pub async fn unrecognized(bot: Bot, msg: Message) -> ResponseResult<()> {
    let response = "🤖 I didn't recognize that command.\nType /help to see what I can do.";
    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}
