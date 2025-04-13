use teloxide::{
    dispatching::Dispatcher,
    error_handlers::LoggingErrorHandler,
    prelude::*,
    utils::command::BotCommands,
};
// use teloxide::dispatching::dialogue::InMemStorage;
// This is for future use: dialogue-based state management for multi-step flows.

// use dotenv::dotenv;
// use log::info;

// Entry point of the bot.
// The `#[tokio::main]` macro starts the Tokio async runtime automatically.
#[tokio::main]
async fn main() {
    // Load environment variables from `.env` file, such as TELOXIDE_TOKEN
    dotenv::dotenv().ok();

    // Initialize a pretty logger (uses `RUST_LOG` env var for filtering)
    pretty_env_logger::init();
    log::info!("Starting titanio-rust-telegram-bot...");

    // Retrieve the bot token from the TELOXIDE_TOKEN environment variable
    let bot = Bot::from_env();

    // Build the dispatcher that handles incoming Telegram updates
    Dispatcher::builder(
        bot.clone(), // Cloning the bot is cheap: it's internally reference-counted
        Update::filter_message()
            // Only handle messages that are bot commands
            .filter_command::<Command>()
            // Route matching commands to `handle_command`
            .endpoint(handle_command),
    )
    // Handle updates that didn't match any known command
    .default_handler(|upd| async move {
        log::warn!("Unhandled update: {:?}", upd);
    })
    // Customize how errors are logged
    .error_handler(LoggingErrorHandler::with_custom_text("Error in dispatcher"))
    .build()
    .dispatch()
    .await;
}

/// Enumeration of supported bot commands
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "Display this help message.")]
    Help,
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Echo a message.")]
    Echo(String),
}

/// Main command handler function.
/// Dispatches each command variant to its respective handler.
async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => handle_help(bot, msg).await,
        Command::Start => handle_start(bot, msg).await,
        Command::Echo(text) => handle_echo(bot, msg, text).await,
    }
}

/// Sends the list of available commands to the user.
async fn handle_help(bot: Bot, msg: Message) -> ResponseResult<()> {
    let help_text = Command::descriptions().to_string();
    bot.send_message(msg.chat.id, help_text).await?;
    Ok(())
}

/// Sends a welcome message when the user starts the bot.
async fn handle_start(bot: Bot, msg: Message) -> ResponseResult<()> {
    let welcome = "Welcome! I'm your helpful Rusty titanio bot 🦀!";
    bot.send_message(msg.chat.id, welcome).await?;
    Ok(())
}

/// Echoes back whatever message the user provides after the /echo command.
async fn handle_echo(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, format!("You said: {text}")).await?;
    Ok(())
}