# BadgerGuard Gateway

The BadgerGuard Gateway is a high-performance reverse proxy written in Rust that enforces zero-trust authentication and policy decisions before forwarding requests to upstream applications.

## Features

- TLS termination with Let's Encrypt support
- Session validation via auth service
- Policy evaluation via OPA
- Rate limiting and DDoS protection
- Health check endpoint (`/healthz`)

## Configuration

See `config.toml.example` for configuration options.

## Building

```bash
cargo build --release
```

## Running

```bash
./target/release/badgerguard-gateway --config config.toml
```

