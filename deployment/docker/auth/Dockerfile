# BadgerGuard Auth Service Dockerfile
FROM rust:1.75-slim as builder

WORKDIR /build

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY backend/auth/Cargo.toml backend/auth/Cargo.lock ./
COPY backend/shared/Cargo.toml ../shared/

# Create dummy source to cache dependencies
RUN mkdir -p src ../shared/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > ../shared/src/lib.rs && \
    cargo build --release && \
    rm -rf src ../shared/src

# Copy actual source
COPY backend/auth/src ./src
COPY backend/shared/src ../shared/src
COPY backend/auth/migrations ./migrations

# Build
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/badgerguard-auth /app/badgerguard-auth
COPY --from=builder /build/migrations /app/migrations

EXPOSE 50051

CMD ["/app/badgerguard-auth"]

