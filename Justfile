# Justfile for titanio-rust-telegram-bot 🦀🤖
# Run commands with: just <recipe-name>
# 
# // Aliases in your shell (optional)
# // Add these to your shell config (.zshrc, .bashrc, etc.):

# // alias j="just"
# // alias jb="just build"
# // alias jr="just run"
# // alias jw="just watch"

# Automatically sets environment variables from `.env`
set dotenv-load := true

# Run the bot
run:
    cargo run

# Run the bot with live reload on file changes
watch:
    cargo watch -x run

# Build the project (release mode optional)
build:
    cargo build --release

# Run tests (including async ones)
test:
    cargo test

# Expand macros (useful for debugging derives and #[tokio::main])
expand:
    cargo expand

# Check for outdated or vulnerable dependencies
audit:
    cargo audit

# Format your code
fmt:
    cargo fmt --all

# Lint your code
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Clean target directory
clean:
    cargo clean

# Show help (all recipes)
default:
    just --summary
