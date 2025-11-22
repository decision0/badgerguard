# BadgerGuard MVP Roadmap

## Phase 1: MVP Foundation

### Backend Gateway
- [ ] **GATEWAY-1**: Implement HTTP server with hyper
- [ ] **GATEWAY-2**: Add `/healthz` endpoint
- [ ] **GATEWAY-3**: Implement basic reverse proxy routing
- [ ] **GATEWAY-4**: Add TLS termination support
- [ ] **GATEWAY-5**: Implement session cookie extraction
- [ ] **GATEWAY-6**: Add gRPC client for auth service
- [ ] **GATEWAY-7**: Add gRPC client for policy engine
- [ ] **GATEWAY-8**: Implement auth middleware
- [ ] **GATEWAY-9**: Add upstream request forwarding

### Backend Auth Service
- [ ] **AUTH-1**: Implement gRPC server with tonic
- [ ] **AUTH-2**: Add PostgreSQL connection pool
- [ ] **AUTH-3**: Add Redis connection for sessions
- [ ] **AUTH-4**: Implement OIDC client (basic flow)
- [ ] **AUTH-5**: Implement Google Workspace integration
- [ ] **AUTH-6**: Implement session creation/validation
- [ ] **AUTH-7**: Implement TOTP secret generation
- [ ] **AUTH-8**: Implement TOTP verification
- [ ] **AUTH-9**: Add QR code generation for TOTP
- [ ] **AUTH-10**: Implement JWT token signing
- [ ] **AUTH-11**: Add user registration/login flow
- [ ] **AUTH-12**: Implement session storage in Redis

### Backend Policy Engine
- [ ] **POLICY-1**: Implement gRPC server with tonic
- [ ] **POLICY-2**: Add OPA HTTP client
- [ ] **POLICY-3**: Implement policy evaluation endpoint
- [ ] **POLICY-4**: Add policy loading from files
- [ ] **POLICY-5**: Implement basic policy templates
- [ ] **POLICY-6**: Add policy hot-reload support

### Frontend Admin UI
- [ ] **UI-1**: Set up API client with axios
- [ ] **UI-2**: Implement login page
- [ ] **UI-3**: Implement dashboard with stats
- [ ] **UI-4**: Implement application list/create/edit
- [ ] **UI-5**: Implement user list/view
- [ ] **UI-6**: Implement policy editor (basic)
- [ ] **UI-7**: Implement audit log viewer with filters
- [ ] **UI-8**: Implement IdP configuration UI
- [ ] **UI-9**: Add form validation with zod

### Database & Migrations
- [ ] **DB-1**: Run initial schema migration
- [ ] **DB-2**: Add seed data for testing
- [ ] **DB-3**: Create database models in Rust

## Phase 2: Zero-Trust Features

### WebAuthn Support
- [ ] **MFA-1**: Implement WebAuthn registration flow
- [ ] **MFA-2**: Implement WebAuthn authentication flow
- [ ] **MFA-3**: Add credential storage in database
- [ ] **MFA-4**: Implement challenge/response validation

### Policy Engine Enhancements
- [ ] **POLICY-7**: Add geo-based policies
- [ ] **POLICY-8**: Add time-based policies
- [ ] **POLICY-9**: Add device-based policies
- [ ] **POLICY-10**: Add IP reputation integration
- [ ] **POLICY-11**: Implement policy versioning

### Additional IdP Integrations
- [ ] **IDP-1**: Implement Okta integration
- [ ] **IDP-2**: Implement JumpCloud integration
- [ ] **IDP-3**: Implement Active Directory/Entra ID integration

## Phase 3: Mobile & Push MFA

### Mobile Apps
- [ ] **MOBILE-1**: Set up React Native project structure
- [ ] **MOBILE-2**: Implement push notification handling
- [ ] **MOBILE-3**: Implement challenge approval/denial
- [ ] **MOBILE-4**: Implement TOTP token display
- [ ] **MOBILE-5**: Implement device registration
- [ ] **MOBILE-6**: Add secure storage (Keychain/Keystore)
- [ ] **MOBILE-7**: Implement offline TOTP support

### Push MFA Backend
- [ ] **PUSH-1**: Implement FCM integration
- [ ] **PUSH-2**: Implement APNs integration
- [ ] **PUSH-3**: Add push challenge creation
- [ ] **PUSH-4**: Implement challenge state management
- [ ] **PUSH-5**: Add push notification queue

## Phase 4: Hardening & Polish

### Security
- [ ] **SEC-1**: Implement mTLS between services
- [ ] **SEC-2**: Add rate limiting
- [ ] **SEC-3**: Implement brute-force protection
- [ ] **SEC-4**: Add security headers (CSP, etc.)
- [ ] **SEC-5**: Implement key rotation strategy
- [ ] **SEC-6**: Add secret management (Vault/KMS integration)

### Observability
- [ ] **OBS-1**: Add OpenTelemetry instrumentation
- [ ] **OBS-2**: Export metrics to Prometheus
- [ ] **OBS-3**: Add structured JSON logging
- [ ] **OBS-4**: Create Grafana dashboards

### Deployment
- [ ] **DEPLOY-1**: Create Helm charts
- [ ] **DEPLOY-2**: Add Kubernetes manifests
- [ ] **DEPLOY-3**: Create docker-compose for production
- [ ] **DEPLOY-4**: Add health check endpoints
- [ ] **DEPLOY-5**: Implement graceful shutdown

### Documentation
- [ ] **DOC-1**: Complete API documentation
- [ ] **DOC-2**: Add developer guide
- [ ] **DOC-3**: Create deployment guides
- [ ] **DOC-4**: Add troubleshooting guide
- [ ] **DOC-5**: Create security best practices guide

### Testing
- [ ] **TEST-1**: Add unit tests for backend services
- [ ] **TEST-2**: Add integration tests
- [ ] **TEST-3**: Add E2E tests for frontend
- [ ] **TEST-4**: Add security testing (SAST/DAST)

## Priority Order for MVP

1. **GATEWAY-1, GATEWAY-2, GATEWAY-3** - Basic gateway functionality
2. **AUTH-1, AUTH-2, AUTH-3, AUTH-4, AUTH-5** - Basic auth service
3. **AUTH-6, AUTH-7, AUTH-8** - TOTP MFA
4. **POLICY-1, POLICY-2, POLICY-3** - Basic policy evaluation
5. **GATEWAY-6, GATEWAY-7, GATEWAY-8** - Connect gateway to services
6. **UI-1, UI-2, UI-3, UI-4** - Basic admin UI
7. **DB-1, DB-2, DB-3** - Database setup

This provides a working MVP where:
- Gateway can proxy requests
- Users can authenticate via Google Workspace
- TOTP MFA is enforced
- Basic policies can be evaluated
- Admin can configure apps via UI

