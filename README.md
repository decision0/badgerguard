# BadgerGuard

**BadgerGuard** is an open-source, self-hosted Zero-Trust MFA Router and policy gateway designed for SMEs with a strong focus on privacy, security, and self-hostability.

## Features

- 🔒 **Zero-Trust Architecture**: Every request evaluated by policy engine
- 🔐 **Multi-Factor Authentication**: WebAuthn, TOTP, and push notifications
- 🌍 **Identity Provider Integration**: Google Workspace, Okta, JumpCloud, Active Directory
- 📋 **Policy Engine**: OPA/Rego-based conditional access policies
- 📱 **Mobile Apps**: iOS and Android apps for push MFA approvals
- 🛡️ **Privacy-First**: No telemetry, no third-party trackers, minimal data retention
- 🚀 **Self-Hostable**: Docker, Kubernetes, and docker-compose support

## Architecture

BadgerGuard consists of:

- **Gateway** (Rust): High-performance reverse proxy with TLS termination
- **Auth Service** (Rust): IdP integration and MFA management
- **Policy Engine** (Rust): OPA integration for policy evaluation
- **Admin UI** (React + TypeScript): Web interface for configuration
- **Mobile Apps** (React Native): Push MFA and TOTP management

## Quick Start

See [docs/deployment/quickstart.md](docs/deployment/quickstart.md) for detailed setup instructions.

```bash
cd deployment
docker-compose up -d
```

## Documentation

- [Architecture Overview](docs/architecture/overview.md)
- [Deployment Guide](docs/deployment/quickstart.md)
- [Threat Model](docs/security/threat-model.md)

## Development

### Backend (Rust)

```bash
cd backend/gateway
cargo build --release
```

### Frontend

```bash
cd frontend/admin
npm install
npm run dev
```

### Mobile

```bash
cd mobile
npm install
npm run ios  # or npm run android
```

## Security

BadgerGuard is built with security-first principles:

- Memory-safe languages (Rust)
- Defense-in-depth architecture
- Zero-trust policy evaluation
- Strong encryption and session management
- Comprehensive audit logging

See [docs/security/threat-model.md](docs/security/threat-model.md) for details.

## License

Apache-2.0 OR MIT

## Contributing

Contributions welcome! Please see CONTRIBUTING.md (coming soon).

## Status

🚧 **Work in Progress** - This project is in active development.
