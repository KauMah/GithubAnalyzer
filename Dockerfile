# Use official Node.js image as base (includes npm)
FROM node:20-bullseye

# Install Rust and Cargo
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH="$HOME/.cargo/bin:$PATH"

# Set working directory
WORKDIR /app

# Copy both api and rs-bin folders
COPY api ./api
COPY rs-bin ./rs-bin

# Build Rust binary
RUN . "$HOME/.cargo/env" && \
    cd rs-bin && \
    cargo build --release

RUN corepack enable && corepack prepare pnpm@latest
# Install Node.js dependencies for the Fastify API
WORKDIR /app/api
RUN pnpm install

# Expose the port your Fastify API uses (change if needed)
EXPOSE 3000

# Start both the Rust binary and Fastify API (example using concurrently)
WORKDIR /app

