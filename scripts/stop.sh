# scripts/stop.sh

#!/bin/bash
set -e

echo "Stopping Titanio Rust Telegram Bot..."

if docker ps -a --format '{{.Names}}' | grep -q "^titanio-bot$"; then
    docker stop titanio-bot
    docker rm titanio-bot
    echo "Bot stopped and container removed."
else
    echo "Container 'titanio-bot' not found."
fi