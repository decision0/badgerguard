# BadgerGuard Policy Engine

The BadgerGuard Policy Engine integrates with OPA (Open Policy Agent) to evaluate access policies based on user attributes, device posture, IP reputation, geo, and time-of-day.

## Features

- OPA/Rego policy evaluation
- Policy hot-reload
- Policy versioning
- Support for complex conditional access rules

## Configuration

See `config.toml.example` for configuration options.

## Policy Development

Policies are written in Rego and stored in the `policies/` directory. See `policies/example.rego` for examples.

## Building

```bash
cargo build --release
```

## Running

```bash
./target/release/badgerguard-policy --config config.toml
```

