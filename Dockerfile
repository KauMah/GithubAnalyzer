FROM node:current-alpine

# Install Rust and Cargo
RUN apk update && \
    apk add --no-cache \
    build-base \
    curl \
    openssl-dev \
    openssl-libs-static \
    musl-dev \
    git

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    PATH="$HOME/.cargo/bin:$PATH" 

ENV CI=true \
    NODE_ENV="production"
# Set working directory
WORKDIR /app

# Copy both api and rs-bin folders
COPY api ./api
COPY rs-bin ./rs-bin

RUN corepack enable && corepack prepare pnpm@latest

WORKDIR  /app/rs-bin
# Build Rust binary
RUN source $HOME/.cargo/env \
    rustup default stable && \
    cargo build --release

# Install Node.js dependencies for the Fastify API
WORKDIR /app/api

RUN pnpm install && \
    pnpm build

RUN mv /app/rs-bin/target/release/github_analyzer /app/api/src/
# Expose the port your Fastify API uses (change if needed)
EXPOSE 10000

# Start both the Rust binary and Fastify API (example using concurrently)

CMD [ "node", "build/index.js" ]
