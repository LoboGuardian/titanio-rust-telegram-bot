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
}

pub mod utils {
    pub mod currency;
    pub mod echo;
    pub mod weather;
}

pub mod fallback {
    pub mod unknown_command;
}

use teloxide::prelude::*;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;

// Importing the bot commands and their handlers
use crate::commands::{
    fun::{joke, roll},
    info::{about, help, id, time},
    system::{ping, start},
    utils::{currency, echo, weather},
};

/// Enumeration of supported bot commands
#[derive(BotCommands, Clone, Debug, PartialEq, Eq)]
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

// This macro should move to a separate file in the future.
//
/// Macro to log command execution details.
/// It logs the command being executed, the user who executed it, and the chat ID.
/// This is useful for debugging and tracking command usage.
macro_rules! log_command {
    ($msg:expr, $cmd:expr) => {
        if let Some(user) = $msg.from.as_ref() {
            log::info!(
                "Executing command: {:?} by user: {} (@{}) in chat: {}",
                $cmd,
                user.id,
                user.username.as_deref().unwrap_or("unknown"),
                $msg.chat.id
            );
        } else {
            log::info!("Executing command: {:?} in chat: {}", $cmd, $msg.chat.id);
        }
    };
}


/// Main command handler function.
/// Dispatches each command variant to its respective handler.
pub async fn dispatch_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    log_command!(msg, &cmd);
    
    let cmd_for_log = cmd.clone(); // <--- Clone BEFORE match
    
    // Pattern match dispatch
    let result = match cmd {
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
    };
    
    // Log outcome
    match &result {
        Ok(_) => log::info!("Command executed successfully: {:?}", cmd_for_log),
        Err(err) => log::error!("Error executing command {:?}: {:?}", cmd_for_log, err),
    }
    
    result
}
