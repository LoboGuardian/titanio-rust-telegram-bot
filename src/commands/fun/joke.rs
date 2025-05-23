use crate::services::ApiService;
use std::sync::Arc;
use teloxide::{prelude::*, types::Message};

pub async fn handle_joke(bot: Bot, msg: Message, api: Arc<ApiService>) -> ResponseResult<()> {
    let reply = match api.get_joke().await {
        Ok(joke) => joke,
        Err(err) => format!("😓 Failed to fetch a joke: {}", err),
    };

    bot.send_message(msg.chat.id, reply).await?;
    Ok(())
}
