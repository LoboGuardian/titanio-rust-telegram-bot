use std::time::Instant;

/// Modules containing handlers for fun commands.
pub mod fun {
    pub mod joke;
    pub mod roll;
}

/// Modules containing handlers for informational commands.
pub mod info {
    pub mod about;
    pub mod help;
    pub mod id;
    pub mod time;
}

/// Modules containing handlers for system-related commands.
pub mod system {
    pub mod ping;
    pub mod start;
}

/// Modules containing handlers for utility commands.
pub mod utils {
    pub mod currency;
    pub mod echo;
    pub mod weather;
}

/// Module containing the handler for unknown commands.
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


// For any reason if I use `///` instead of `//` the bot send it as a message
// to the user. This is a bug in the `teloxide` crate.
// This is a workaround to avoid that behavior.
// Check teloxide code to help fix it

// / Enumeration of supported bot commands.
// /
// / This `enum` defines all the commands that the bot can understand and process.
// / Each variant of the `Command` enum is associated with a description
// / that is used when generating the help message. The `BotCommands` derive macro
// / from the `teloxide` crate automatically handles parsing commands from user input
// / and generating help text based on the attributes specified here.
// /
// / The `rename_rule = "lowercase"` attribute ensures that commands are matched
// / case-insensitively and are expected in lowercase.
// /
// / # Variants
// /
// / ## System
// /
// / * `Start`: Initiates the bot and typically sends a welcome message.
// / * `Ping`: Checks the bot's responsiveness and confirms it's online.
// /
// / ## Info
// /
// / * `Help`: Displays a list of available commands and their descriptions.
// / * `About`: Shows information about the bot, such as its name and version.
// / * `Id`: Returns the user ID of the sender and the ID of the current chat.
// / * `Time`: Provides the current time.
// /
// / ## Utils
// /
// / * `Echo(String)`: Repeats the text provided by the user after the command.
// / * `Weather(String)`: Retrieves and displays the weather information for the specified city.
// / * `Currency(String)`: Converts an amount from one currency to another based on the provided input string (e.g., "10 USD to EUR").
// /
// / ## Fun
// /
// / * `Roll`: Generates and displays a random number (typically between 1 and 100).
// / * `Joke`: Tells a random humorous joke.
// /

// / This enum is used to define the commands that the bot can handle.
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
/// Logs the execution details of a command, including the command itself,
/// the user who initiated it, the chat ID, the outcome (success or error),
/// and the execution duration.
///
/// This macro provides valuable insights for debugging, monitoring command usage,
/// and understanding potential issues. It automatically logs both successful
/// and failed command executions with relevant context.
///
/// # Arguments
///
/// * `$msg`: An expression that evaluates to a message type containing user and chat information.
/// * `$cmd`: An expression that evaluates to the command being executed (e.g., an enum variant or a string).
/// * `$result`: An expression that evaluates to a `Result` indicating the outcome of the command execution.
/// * `$duration`: An expression that evaluates to a `Duration` representing the time taken to execute the command.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// // Assuming you have a `Message` struct with `from` (Option<User>) and `chat` (Chat) fields.
/// // And `User` and `Chat` structs with `id` and `username` fields.
/// #[derive(Debug)]
/// struct User {
///     id: i64,
///     username: Option<String>,
/// }
///
/// #[derive(Debug)]
/// struct Chat {
///     id: i64,
/// }
///
/// #[derive(Debug)]
/// struct Message {
///     from: Option<User>,
///     chat: Chat,
/// }
///
/// fn some_command() -> Result<(), String> {
///     // Simulate a successful operation
///     Ok(())
/// }
///
/// fn another_command() -> Result<(), String> {
///     // Simulate a failed operation
///     Err("Something went wrong!".to_string())
/// }
///
/// fn main() {
///     let user = Some(User { id: 12345, username: Some("testuser".to_string()) });
///     let chat = Chat { id: 67890 };
///     let message = Message { from: user, chat };
///     let duration = Duration::from_millis(50);
///
///     log_command_execution!(message, "start", some_command(), duration);
///
///     let user_no_username = Some(User { id: 54321, username: None });
///     let message_no_username = Message { from: user_no_username, chat: Chat { id: 98765 } };
///     let duration_err = Duration::from_millis(120);
///
///     log_command_execution!(message_no_username, "fail", another_command(), duration_err);
///
///     let message_no_user = Message { from: None, chat: Chat { id: 112233 } };
///     let duration_no_user = Duration::from_millis(80);
///
///     log_command_execution!(message_no_user, "info", Ok(()), duration_no_user);
/// }
/// ```
macro_rules! log_command_execution {
    ($msg:expr, $cmd:expr, $result:expr, $duration:expr) => {{
        let user = $msg.from.as_ref();
        let chat_id = $msg.chat.id;
        let user_id = user.map(|u| u.id.to_string()).unwrap_or_else(|| "unknown".to_string());
        let username = user
            .and_then(|u| u.username.as_ref())
            .map(String::as_str)
            .unwrap_or("unknown");

        let ms = $duration.as_millis();

        const COLOR_GREEN: &str = "\x1b[32m";
        const COLOR_RED: &str = "\x1b[31m";
        const COLOR_RESET: &str = "\x1b[0m";

        match $result {
            Ok(_) => log::info!(
                "{}[SUCCESS]{} Command: {:?} | User: {} (@{}) | Chat: {} | Duration: {} ms",
                COLOR_GREEN,
                COLOR_RESET,
                $cmd,
                user_id,
                username,
                chat_id,
                ms
            ),
            Err(err) => log::error!(
                "{}[ERROR]{} Command: {:?} | User: {} (@{}) | Chat: {} | Duration: {} ms | Error: {:?}",
                COLOR_RED,
                COLOR_RESET,
                $cmd,
                user_id,
                username,
                chat_id,
                ms,
                err
            ),
        }
    }};
}

/// Main command handler function.
///
/// This asynchronous function takes a `Bot`, a `Message`, and a `Command` enum variant
/// as input and dispatches the command to its corresponding handler function.
/// It also measures the execution time of the command and logs the details
/// using the `log_command_execution!` macro, including the command, user, chat,
/// and the outcome (success or error).
///
/// # Arguments
///
/// * `bot`: A reference to the `Bot` instance used for sending responses.
/// * `msg`: The `Message` object that triggered the command.
/// * `cmd`: An enum variant of the `Command` type representing the specific command to be executed.
///
/// # Returns
///
/// A `ResponseResult<()>` indicating the success or failure of the command handling.
/// A `Ok(())` signifies that the command was processed successfully, while an `Err(_)`
/// indicates an error occurred during command execution.
///
/// # Example
///
/// ```rust,no_run
/// use teloxide::{Bot, types::Message, utils::command::BotCommands};
/// use std::time::Instant;
///
/// // Assuming you have a `Command` enum defined elsewhere, like this:
/// #[derive(BotCommands, Clone, Debug)]
/// #[command(rename_rule = "lowercase", description = "These are my commands:")]
/// enum Command {
///     #[command(help = "Display this help message.")]
///     Help,
///     #[command(help = "Starts the bot.")]
///     Start,
///     #[command(help = "Echoes the given text.")]
///     Echo(String),
///     // ... other commands
/// }
///
/// // Assuming you have handler functions defined elsewhere, e.g.:
/// // mod help;
/// // mod start;
/// // mod echo;
/// // ...
///
/// // Assuming you have the `log_command_execution!` macro available.
///
/// pub async fn dispatch_command(bot: Bot, msg: Message, cmd: Command) -> Result<(), teloxide::RequestError> {
///     let cmd_for_log = cmd.clone(); // Save for logging after match
///     let msg_clone = msg.clone(); // clone early to avoid double borrows
///     let start = Instant::now();
///
///     let result = match cmd {
///         Command::Help => {
///             async {
///                 println!("Handling /help command");
///                 Ok(()) // Replace with actual help handler
///             }.await
///         }
///         Command::Start => {
///             async {
///                 println!("Handling /start command");
///                 Ok(()) // Replace with actual start handler
///             }.await
///         }
///         Command::Echo(text) => {
///             async {
///                 println!("Handling /echo command with text: {}", text);
///                 Ok(()) // Replace with actual echo handler
///             }.await
///         }
///         // ... other command matches
///     };
///
///     let duration = start.elapsed();
///     // Assuming 'log_command_execution!' is defined elsewhere and accessible.
///     // log_command_execution!(msg, cmd_for_log, &result, duration);
///     println!("Command executed in {:?}", duration);
///
///     result
/// }
///
/// // Example of how this might be used in a main function or handler:
/// // async fn handle_message(bot: Bot, msg: Message) -> Result<(), teloxide::RequestError> {
/// //     if let Some(command) = Command::parse(&msg.text().unwrap_or_default(), "bot_name") {
/// //         dispatch_command(bot, msg, command).await?;
/// //     }
/// //     Ok(())
/// // }
/// ```
pub async fn dispatch_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let cmd_for_log = cmd.clone(); // Save for logging after match
    let msg_clone = msg.clone(); // clone early to avoid double borrows
    let start = Instant::now();

    // Pattern match dispatch
    let result = match cmd {
        Command::Help => help::handle_help(bot, msg_clone).await,
        Command::Start => start::handle_start(bot, msg_clone).await,
        Command::Echo(text) => echo::handle_echo(bot, msg_clone, text).await,
        Command::About => about::handle_about(bot, msg_clone).await,
        Command::Roll => roll::handle_roll(bot, msg_clone).await,
        Command::Id => id::handle_id(bot, msg_clone).await,
        Command::Time => time::handle_time(bot, msg_clone).await,
        Command::Ping => ping::handle_ping(bot, msg_clone).await,
        Command::Joke => joke::handle_joke(bot, msg_clone).await,
        Command::Weather(city) => weather::handle_weather(bot, msg_clone, city).await,
        Command::Currency(text) => currency::handle_currency(bot, msg_clone, text).await,
    };

    let duration = start.elapsed();
    log_command_execution!(msg, cmd_for_log, &result, duration);

    result
}
