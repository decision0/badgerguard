# BadgerGuard Quick Start

## Prerequisites

- Docker and Docker Compose
- PostgreSQL 16+ (or use included docker-compose)
- Redis 7+ (or use included docker-compose)

## Local Development Setup

1. Clone the repository:
```bash
git clone https://github.com/yourorg/badgerguard.git
cd badgerguard
```

2. Start services with docker-compose:
```bash
cd deployment
docker-compose up -d
```

3. Configure your first tenant and IdP in the admin UI:
   - Navigate to http://localhost:3000
   - Go to Settings → Identity Providers
   - Add your IdP (Google Workspace, Okta, etc.)

4. Configure an upstream application:
   - Go to Applications
   - Add a new application with route and upstream URL

## Production Deployment

See `deployment/kubernetes/` for Kubernetes manifests and Helm charts.

## Configuration

Each service requires a `config.toml` file. See service-specific READMEs for configuration options.

