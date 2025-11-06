// src/commands/mod.rs (OPTIMIZED)

//! Command routing and execution infrastructure.
//!
//! This module defines the command enum, dispatch logic, and execution monitoring.
//! It maintains separation between command parsing (teloxide) and handler execution (submodules).

use teloxide::prelude::*;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;

use crate::services::ApiService;
use std::sync::Arc;
use std::time::Instant;

/// Command handler submodules organized by domain.
pub mod fallback;
pub mod fun;
pub mod info;
pub mod system;
pub mod utils;

/// Enumeration of all supported bot commands.
///
/// Each variant maps to a specific handler function in the corresponding submodule.
/// The `BotCommands` derive macro handles parsing from user input and generates
/// help text from the `#[command(description)]` attributes.
#[derive(BotCommands, Clone, Debug, PartialEq, Eq)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Check if the bot is alive.")]
    Ping,
    #[command(description = "Display this help message.")]
    Help,
    #[command(description = "Show bot information.")]
    About,
    #[command(description = "Show your user ID and chat ID.")]
    Id,
    #[command(description = "Show the current time.")]
    Time,
    #[command(description = "Echo a message.")]
    Echo(String),
    #[command(description = "Check weather in a city.")]
    Weather(String),
    #[command(description = "Convert currency (e.g., '10 USD to EUR').")]
    Currency(String),
    #[command(description = "Roll a random number.")]
    Roll,
    #[command(description = "Tell a random joke.")]
    Joke,
}

/// Compact execution metadata for structured logging.
struct ExecutionMetrics {
    user_id: String,
    username: String,
    chat_id: i64,
}

impl ExecutionMetrics {
    /// Extracts minimal execution context from a message.
    ///
    /// Allocates owned strings to avoid lifetime entanglement with `Message`.
    #[inline]
    fn from_message(msg: &Message) -> Self {
        Self {
            user_id: msg
                .from
                .as_ref()
                .map(|u| u.id.0.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            username: msg
                .from
                .as_ref()
                .and_then(|u| u.username.as_ref())
                .cloned()
                .unwrap_or_else(|| "unknown".to_string()),
            chat_id: msg.chat.id.0,
        }
    }

    /// Logs command execution with ANSI color codes.
    #[inline]
    fn log<T, E: std::fmt::Debug>(
        &self,
        cmd: &Command,
        result: &Result<T, E>,
        duration_ms: u128,
    ) {
        match result {
            Ok(_) => log::info!(
                "\x1b[32m[SUCCESS]\x1b[0m Command: {:?} | User: {} (@{}) | Chat: {} | {}ms",
                cmd,
                self.user_id,
                self.username,
                self.chat_id,
                duration_ms
            ),
            Err(err) => log::error!(
                "\x1b[31m[ERROR]\x1b[0m Command: {:?} | User: {} (@{}) | Chat: {} | {}ms | {:?}",
                cmd,
                self.user_id,
                self.username,
                self.chat_id,
                duration_ms,
                err
            ),
        }
    }
}

/// Primary command dispatch handler.
///
/// Routes incoming commands to their respective handlers while measuring
/// execution time and logging outcomes for observability.
///
/// # Arguments
///
/// * `bot` - Telegram bot instance for sending responses.
/// * `msg` - Original message that triggered the command.
/// * `cmd` - Parsed command variant.
/// * `api` - Shared API service for external data fetching.
///
/// # Errors
///
/// Propagates handler-specific errors as `RequestError` variants.
pub async fn dispatch_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    api: Arc<ApiService>,
) -> ResponseResult<()> {
    let metrics = ExecutionMetrics::from_message(&msg);
    let start = Instant::now();

    let result = route_command(bot, msg, &cmd, api).await;

    metrics.log(&cmd, &result, start.elapsed().as_millis());

    result
}

/// Internal routing logic separated for testability.
///
/// This pure function allows unit testing of command dispatch without
/// timer instrumentation or logging side effects.
async fn route_command(
    bot: Bot,
    msg: Message,
    cmd: &Command,
    api: Arc<ApiService>,
) -> ResponseResult<()> {
    match cmd {
        Command::Start => system::start::handle_start(bot, msg).await,
        Command::Ping => system::ping::handle_ping(bot, msg).await,
        Command::Help => info::help::handle_help(bot, msg).await,
        Command::About => info::about::handle_about(bot, msg).await,
        Command::Id => info::id::handle_id(bot, msg).await,
        Command::Time => info::time::handle_time(bot, msg).await,
        Command::Echo(text) => utils::echo::handle_echo(bot, msg, text.clone()).await,
        Command::Weather(city) => {
            utils::weather::handle_weather(bot, msg, city.clone(), api).await
        }
        Command::Currency(input) => {
            utils::currency::handle_currency(bot, msg, input.clone(), api).await
        }
        Command::Roll => fun::roll::handle_roll(bot, msg).await,
        Command::Joke => fun::joke::handle_joke(bot, msg, api).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        assert_eq!(Command::parse("/start", "bot").unwrap(), Command::Start);
        assert_eq!(
            Command::parse("/echo hello", "bot").unwrap(),
            Command::Echo("hello".to_string())
        );
    }
}