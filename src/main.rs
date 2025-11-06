// src/main.rs

use teloxide::{
    dispatching::{Dispatcher, UpdateHandler},
    error_handlers::LoggingErrorHandler,
    prelude::*,
    RequestError,
};

use dotenv::dotenv;
use log::{error, info};

mod commands;
mod services;

use crate::commands::{dispatch_command, fallback::unknown_command::unrecognized, Command};
use crate::services::ApiService;
use dptree::deps;
use std::sync::Arc;

/// Constructs the update handling schema for the bot.
///
/// Separating schema construction from runtime initialization enables
/// unit testing of routing logic without spawning a full dispatcher.
fn build_command_schema() -> UpdateHandler<RequestError> {
    dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(dispatch_command),
        )
        .branch(
            Update::filter_message()
                .filter(|msg: Message| msg.text().map_or(false, |t| t.starts_with('/')))
                .endpoint(unrecognized),
        )
}

/// Initializes the bot and verifies connectivity.
///
/// # Errors
///
/// Returns error if bot token is invalid or network initialization fails.
async fn initialize_bot() -> Result<Bot, Box<dyn std::error::Error>> {
    let bot = Bot::from_env();
    
    let me = bot.get_me().send().await?;
    let username = me.user.username.as_deref().unwrap_or("<unknown>");
    
    info!("Bot initialized as @{}", username);
    
    Ok(bot)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let bot = match initialize_bot().await {
        Ok(bot) => bot,
        Err(err) => {
            error!("Failed to initialize bot: {}", err);
            std::process::exit(1);
        }
    };

    let api_service = Arc::new(ApiService::new(
        std::env::var("EXCHANGERATE_TOKEN").ok(),
    ));

    let command_handler = build_command_schema();

    Dispatcher::builder(bot, command_handler)
        .dependencies(deps![api_service])
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text("Error in dispatcher"))
        .build()
        .dispatch()
        .await;
}