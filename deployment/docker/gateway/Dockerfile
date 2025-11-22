# BadgerGuard Gateway Dockerfile
FROM rust:1.75-slim as builder

WORKDIR /build

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY backend/gateway/Cargo.toml backend/gateway/Cargo.lock ./
COPY backend/shared/Cargo.toml ../shared/

# Create dummy source to cache dependencies
RUN mkdir -p src ../shared/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > ../shared/src/lib.rs && \
    cargo build --release && \
    rm -rf src ../shared/src

# Copy actual source
COPY backend/gateway/src ./src
COPY backend/shared/src ../shared/src

# Build
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/badgerguard-gateway /app/badgerguard-gateway

EXPOSE 8080 8443

CMD ["/app/badgerguard-gateway"]

