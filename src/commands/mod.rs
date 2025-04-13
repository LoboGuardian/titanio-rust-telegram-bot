pub mod help;
pub mod start;
pub mod echo;
pub mod about;
pub mod roll;
pub mod id;

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
    #[command(description = "Show bot info.")]
    About,
    #[command(description = "Roll a random number.")]
    Roll,
    #[command(description = "Show your user ID and chat ID.")]
    Id,
}

/// Main command handler function.
/// Dispatches each command variant to its respective handler.
pub async fn dispatch_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => handle_help(bot, msg).await,
        Command::Start => handle_start(bot, msg).await,
        Command::Echo(text) => handle_echo(bot, msg, text).await,
        Command::About => about::handle_about(bot, msg).await,
        Command::Roll => roll::handle_roll(bot, msg).await,
        Command::Id => id::handle_id(bot, msg).await,
    }
}
