# scripts/run.sh

#!/bin/bash
set -e

# Load environment variables from .env if it exists
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Validate required environment variables
if [ -z "$TELOXIDE_TOKEN" ]; then
    echo "Error: TELOXIDE_TOKEN is not set"
    exit 1
fi

echo "Starting Titanio Rust Telegram Bot..."

docker run -d \
    --name titanio-bot \
    --restart unless-stopped \
    --env TELOXIDE_TOKEN="$TELOXIDE_TOKEN" \
    --env EXCHANGERATE_TOKEN="${EXCHANGERATE_TOKEN:-}" \
    --env RUST_LOG="${RUST_LOG:-info}" \
    --memory="256m" \
    --cpus="0.5" \
    --security-opt no-new-privileges:true \
    titanio-rust-telegram-bot:latest

echo "Bot started successfully!"
echo "Container name: titanio-bot"
echo ""
echo "View logs with: docker logs -f titanio-bot"
echo "Stop bot with: docker stop titanio-bot"
echo "Remove container with: docker rm titanio-bot"