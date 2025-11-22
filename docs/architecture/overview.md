# BadgerGuard Architecture Overview

## System Components

BadgerGuard consists of four main services:

1. **Gateway** - Reverse proxy that handles TLS termination and request routing
2. **Auth Service** - Identity provider integration and MFA management
3. **Policy Engine** - OPA-based policy evaluation
4. **Admin UI** - Web interface for configuration and monitoring

## Request Flow

```
User Request
    ↓
Gateway (TLS termination)
    ↓
Auth Middleware (session validation)
    ↓
Policy Engine (access decision)
    ↓
Upstream Application
```

## Authentication Flow

```
User → Gateway → Auth Service → IdP (OIDC)
    ↓
MFA Challenge (WebAuthn/TOTP/Push)
    ↓
Session Creation → Redis
    ↓
Policy Evaluation → OPA
    ↓
Access Granted/Denied
```

## Data Flow

- **PostgreSQL**: User data, tenants, authenticators, audit logs
- **Redis**: Session state, rate limiting, MFA challenges
- **OPA**: Policy rules (Rego files)

## Security Model

- Zero-trust: Every request evaluated
- Defense-in-depth: Network, auth, sessions, crypto
- Memory-safe: Rust for backend services
- Privacy-first: No telemetry, minimal data retention

