# scripts/build.sh

#!/bin/bash
set -e

echo "Building Titanio Rust Telegram Bot Docker image..."

# Build with BuildKit for better caching
DOCKER_BUILDKIT=1 docker build \
    --tag titanio-rust-telegram-bot:latest \
    --tag titanio-rust-telegram-bot:$(git rev-parse --short HEAD 2>/dev/null || echo "dev") \
    --build-arg BUILDKIT_INLINE_CACHE=1 \
    .

echo "Build complete!"
echo "Image: titanio-rust-telegram-bot:latest"