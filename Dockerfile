# Dockerfile

# Build stage: Compile Rust binary with full toolchain
FROM rust:1.83-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifest files first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Rebuild only the application code (dependencies are cached)
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage: Minimal Debian image
FROM debian:bookworm-slim

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 -s /bin/bash titaniors

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/titanio-rust-telegram-bot /app/bot

# Copy .env file if it exists (optional, prefer environment variables)
# COPY .env /app/.env

# Change ownership to non-root user
RUN chown -R titaniors:titaniors /app

# Switch to non-root user
USER titaniors

# Set environment variables
ENV RUST_LOG=info

# Expose no ports (bot uses polling, not webhooks)

# Health check (optional - checks if process is running)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep -f bot || exit 1

# Run the bot
CMD ["/app/bot"]