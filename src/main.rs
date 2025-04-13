use teloxide::{dispatching::Dispatcher, error_handlers::LoggingErrorHandler, prelude::*};

// use teloxide::dispatching::dialogue::InMemStorage;
// This is for future use: dialogue-based state management for multi-step flows.

use dotenv::dotenv;
use log::info;

mod commands;
use commands::{Command, dispatch_command};

// Entry point of the bot.
// The `#[tokio::main]` macro starts the Tokio async runtime automatically.
#[tokio::main]
async fn main() {
    // Load environment variables from `.env` file, such as TELOXIDE_TOKEN
    dotenv().ok();

    // Initialize a pretty logger (uses `RUST_LOG` env var for filtering)
    pretty_env_logger::init();
    info!("Starting titanio-rust-telegram-bot...");

    // Retrieve the bot token from the TELOXIDE_TOKEN environment variable
    let bot = Bot::from_env();

    // Build the dispatcher that handles incoming Telegram updates
    Dispatcher::builder(
        // Cloning the bot is cheap: it's internally reference-counted
        bot.clone(),
        Update::filter_message()
            // Only handle messages that are bot commands
            .filter_command::<Command>()
            // Route matching commands to `handle_command`
            .endpoint(dispatch_command),
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
