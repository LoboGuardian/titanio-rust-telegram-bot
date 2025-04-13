pub mod help;
pub mod start;
pub mod echo;

use teloxide::types::Message;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use help::handle_help;
use start::handle_start;
use echo::handle_echo;

/// Enumeration of supported bot commands
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "Display this help message.")]
    Help,
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Echo a message.")]
    Echo(String),
}

/// Main command handler function.
/// Dispatches each command variant to its respective handler.
pub async fn dispatch_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => handle_help(bot, msg).await,
        Command::Start => handle_start(bot, msg).await,
        Command::Echo(text) => handle_echo(bot, msg, text).await,
    }
}
