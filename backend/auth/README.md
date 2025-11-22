# BadgerGuard Auth Service

The BadgerGuard Auth Service handles identity provider integration, MFA enrollment/authentication, and session management.

## Features

- OIDC integration with Google Workspace, Okta, JumpCloud, and Active Directory
- WebAuthn (FIDO2) support
- TOTP (RFC 6238) with QR code provisioning
- Push notification MFA
- Session management with Redis
- User and tenant management

## Configuration

See `config.toml.example` for configuration options.

## Building

```bash
cargo build --release
```

## Running

```bash
./target/release/badgerguard-auth --config config.toml
```

## Database Migrations

```bash
sqlx migrate run
```

