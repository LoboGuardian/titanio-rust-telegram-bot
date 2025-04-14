use teloxide::{dispatching::Dispatcher, error_handlers::LoggingErrorHandler, prelude::*};

// use teloxide::dispatching::dialogue::InMemStorage;
// This is for future use: dialogue-based state management for multi-step flows.

use dotenv::dotenv;
use log::info;

mod commands;
use commands::{Command, dispatch_command};
use crate::commands::unrecognized::handle_unrecognized;
// Entry point of the bot.
// The `#[tokio::main]` macro starts the Tokio async runtime automatically.

// The `flavor` attribute allows you to specify the type of runtime.
// The `worker_threads` attribute allows you to specify the number of worker threads.
// The default is `multi_thread` with 4 worker threads.
// You can uncomment the following line to use a multi-threaded runtime with 4 worker threads.
// This is useful for CPU-bound tasks or when you want to run multiple tasks in parallel.
// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]

#[tokio::main]
async fn main() {
    // Load environment variables from `.env` file, such as TELOXIDE_TOKEN
    dotenv().ok();

    // Initialize a pretty logger (uses `RUST_LOG` env var for filtering)
    pretty_env_logger::init();
    info!("Starting titanio-rust-telegram-bot...");

    // Retrieve the bot token from the TELOXIDE_TOKEN environment variable
    let bot = Bot::from_env();

    // Set the bot's name and username
    // This is optional but can be useful for debugging or logging purposes.
    match bot.get_me().send().await {
        Ok(me) => info!("Bot started as: @{}", me.user.username.unwrap_or_default()),
        Err(err) => log::error!("Failed to verify bot identity: {:?}", err),
    }

    let command_handler = dptree::entry()
        .branch(Update::filter_message()
            // Only handle messages that are bot commands
            .filter_command::<Command>()
            // Route matching commands to `handle_command`
            .endpoint(dispatch_command))
        // Fallback for unrecognized commands.
        .branch(Update::filter_message().endpoint(handle_unrecognized));

    // Build the dispatcher that handles incoming Telegram updates
    Dispatcher::builder(
        // Cloning the bot is cheap: it's internally reference-counted
        bot.clone(), command_handler
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
