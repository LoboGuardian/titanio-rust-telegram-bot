use reqwest::Client;
use serde::Deserialize;
use teloxide::{prelude::*, types::Message};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum JokeResponse {
    Single { joke: String },
    TwoPart { setup: String, delivery: String },
}

pub async fn handle_joke(bot: Bot, msg: Message) -> ResponseResult<()> {
    let client = Client::new();

    let res = client
        .get("https://v2.jokeapi.dev/joke/Any?safe-mode&type=single,twopart")
        .send()
        .await;

    let joke_text = match res {
        Ok(response) => match response.json::<JokeResponse>().await {
            Ok(JokeResponse::Single { joke }) => joke,
            Ok(JokeResponse::TwoPart { setup, delivery }) => format!("{}\n{}", setup, delivery),
            Err(_) => "😵 Couldn't parse the joke!".to_string(),
        },
        Err(_) => "😓 Couldn't fetch a joke right now. Try again later.".to_string(),
    };

    bot.send_message(msg.chat.id, joke_text).await?;
    Ok(())
}
