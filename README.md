# Titanio-Rust-Telegram-Bot

This is a simple Telegram bot written in Rust using the [`teloxide`](https://docs.rs/teloxide) framework. Every time someone sends the bot a message, it replies by rolling a fun Telegram dice 🎲!

## 🚀 Features

- Built with [Rust](https://www.rust-lang.org/)
- Uses [`tokio`](https://tokio.rs/) for async magic
- Environment-based configuration using `.env`
- Clear, minimal, and beginner-friendly!

## 📦 Requirements

Before running the bot, make sure you have the following:

- Rust (Install from [rustup.rs](https://rustup.rs/))
- A Telegram bot token (create one via [@BotFather](https://t.me/BotFather))
- A `.env` file with your bot token

## 🛠️ Setup

1. **Clone the repo**

```bash
git clone https://github.com/your-username/titanio-rust-telegram-bot.git
cd titanio-rust-telegram-bot
```

2. **Create a .env file**

In the root folder, create a .env file that looks like this:

```ini
TELOXIDE_TOKEN=your_telegram_bot_token_here
```

3. **Run the bot**

```bash
cargo run
```

Your bot is now alive and rolling dice in reply to messages it receives! 🎉

## 📁 Project Structure

```bash
.
├── .gitignore         # Files to ignore in Git
├── Cargo.toml         # Project metadata and dependencies
├── LICENSE            # MIT License
├── README.md          # You're reading it!
└── src
    └── main.rs        # Main bot logic
```

## 📜 How It Works

The bot uses the [teloxide::repl] function to set up a message loop. For every incoming message, it sends a dice emoji back to the same chat:

```rust
teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    bot.send_dice(msg.chat.id).await?;
    Ok(())
}).await;
```

## We also use:

dotenv to load the token from a .env file

pretty_env_logger and log to log when the bot starts

## 🦀 Why Rust?

Rust is a fast, safe, and fun systems programming language. If you're new to it, check out the [official Rust book](https://doc.rust-lang.org/book/). This project is a great way to dip your claws into async programming with Rust!

## 📄 License

This project is licensed under the ![MIT License](LICENSE).

Happy botting! 🤖🐚