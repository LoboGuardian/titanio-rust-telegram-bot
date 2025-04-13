use teloxide::{prelude::*, types::Message};

// This function sends a message to the user with their user ID and chat ID.
pub async fn handle_id(bot: Bot, msg: Message) -> ResponseResult<()> {
    let user_id_text = match msg.from.as_ref() {
        Some(user) => format!("👤 Your user ID: {}\n💬 Chat ID: {}", user.id.0, msg.chat.id),
        None => format!("❓ Could not determine your user ID.\n💬 Chat ID: {}", msg.chat.id),
    };

    bot.send_message(msg.chat.id, user_id_text).await?;
    Ok(())
}