# Multi-stage build for optimized Rust backend
FROM rust:latest AS builder

# Install system dependencies needed for Solana and OpenSSL
RUN apt-get update && apt-get upgrade -y && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached and generates Cargo.lock)
RUN cargo build --release && rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Runtime stage - smaller image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get upgrade -y && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -r -s /bin/false solana-wallet

COPY --from=builder /app/target/release/solana-wallet /usr/local/bin/solana-wallet

# Make binary executable and change ownership
RUN chmod +x /usr/local/bin/solana-wallet && \
    chown solana-wallet:solana-wallet /usr/local/bin/solana-wallet

# Switch to non-root user
USER solana-wallet

# Expose port 8080
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080
ENV SOLANA_RPC_URL=https://api.devnet.solana.com

# HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
#     CMD curl -f http://localhost:8080/ || exit 1

CMD ["solana-wallet"]
