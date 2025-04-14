pub mod fun {
    pub mod joke;
    pub mod roll;
}

pub mod info {
    pub mod about;
    pub mod help;
    pub mod id;
    pub mod time;
}

pub mod system {
    pub mod ping;
    pub mod start;
    pub mod unrecognized;
}

pub mod utils {
    pub mod currency;
    pub mod echo;
    pub mod weather;
}

use teloxide::prelude::*;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;

use crate::commands::{
    fun::{joke, roll},
    info::{about, help, id, time},
    system::{ping, start},
    utils::{currency, echo, weather},
};

/// Enumeration of supported bot commands
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    // System
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Check if the bot is alive.")]
    Ping,

    // Info
    #[command(description = "Display this help message.")]
    Help,
    #[command(description = "Show bot info.")]
    About,
    #[command(description = "Show your user ID and chat ID.")]
    Id,
    #[command(description = "Show the current time.")]
    Time,

    // Utils
    #[command(description = "Echo a message.")]
    Echo(String),
    #[command(description = "Check the weather in a city.")]
    Weather(String),
    #[command(description = "Convert amount from one currency to another.")]
    Currency(String),

    // Fun
    #[command(description = "Roll a random number.")]
    Roll,
    #[command(description = "Tell a random joke.")]
    Joke,
}

/// Main command handler function.
/// Dispatches each command variant to its respective handler.
pub async fn dispatch_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => help::handle_help(bot, msg).await,
        Command::Start => start::handle_start(bot, msg).await,
        Command::Echo(text) => echo::handle_echo(bot, msg, text).await,
        Command::About => about::handle_about(bot, msg).await,
        Command::Roll => roll::handle_roll(bot, msg).await,
        Command::Id => id::handle_id(bot, msg).await,
        Command::Time => time::handle_time(bot, msg).await,
        Command::Ping => ping::handle_ping(bot, msg).await,
        Command::Joke => joke::handle_joke(bot, msg).await,
        Command::Weather(city) => weather::handle_weather(bot, msg, city).await,
        Command::Currency(text) => currency::handle_currency(bot, msg, text).await,
    }
}
