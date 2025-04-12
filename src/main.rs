use teloxide::{dispatching::Dispatcher, error_handlers::LoggingErrorHandler, prelude::*};
use teloxide::{prelude::*, utils::command::BotCommands};
// use teloxide::dispatching::dialogue::InMemStorage;
// Later: dialogue middlewares, state enums, and transitions

// use dotenv::dotenv;
// use log::info;

// This tells Rust that main() is async and runs on the Tokio runtime
#[tokio::main]
async fn main() {
    // Loads environment variables from `.env` file
    dotenv::dotenv().ok();

    // Starts a pretty logger (for nice log output)
    pretty_env_logger::init();
    log::info!("Starting titanio-rust-telegram-bot...");

    // Gets the token from the TELOXIDE_TOKEN variable
    let bot = Bot::from_env();

    Dispatcher::builder(bot.clone(), Update::filter_message().filter_command::<Command>().endpoint(handle_command))
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text("Error in dispatcher"))
        .build()
        .dispatch()
        .await;
}

// Define your commands as an enum with the `BotCommands` derive macro
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

// Respond to commands
async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => handle_help(bot, msg).await,
        Command::Start => handle_start(bot, msg).await,
        Command::Echo(text) => handle_echo(bot, msg, text).await,
    }
}


async fn handle_help(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn handle_start(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Welcome! I'm your helpful Rusty titanio bot 🦀!").await?;
    Ok(())
}

async fn handle_echo(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, format!("You said: {text}")).await?;
    Ok(())
}