# BadgerGuard Threat Model

## Attack Surfaces

### 1. Network Layer
- **Threat**: DDoS, MITM, network sniffing
- **Mitigation**: TLS termination, rate limiting, DDoS protection

### 2. Authentication Layer
- **Threat**: Credential theft, session hijacking, MFA bypass
- **Mitigation**: OIDC integration, secure session management, multiple MFA methods

### 3. Policy Layer
- **Threat**: Policy bypass, privilege escalation
- **Mitigation**: OPA evaluation, audit logging, policy versioning

### 4. Data Layer
- **Threat**: Data breach, unauthorized access
- **Mitigation**: Encryption at rest, access controls, minimal data retention

## Security Assumptions

1. IdP is trusted and secure
2. Network between services is trusted (use mTLS in production)
3. Database and Redis are secured
4. Secrets are managed securely (Vault/KMS)

## Limitations

- BadgerGuard does not protect against compromised IdP
- Policy evaluation depends on accurate input data (IP, geo, device)
- MFA effectiveness depends on user education

