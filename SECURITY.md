# Security Policy

**Version**: 1.0.0
**Created**: 2026-02-22
**Component**: security

---

## Overview

CAPPY TAC Toolkit processes sensitive customer evidence (HAR files, log bundles, screenshots). This document describes the security architecture, isolation guarantees, and threat mitigations.

---

## Sandbox Architecture

### Multi-Backend Isolation

All tool executions are routed through sandboxed environments. The system automatically selects the best available backend:

| Priority | Backend | Platform | License | Isolation Level |
|----------|---------|----------|---------|-----------------|
| 1 | **Podman** | All | FREE (Apache 2.0) | Container (rootless) |
| 2 | Docker | All | Paid (enterprise) | Container |
| 3 | Bubblewrap | Linux | FREE | Namespace |
| 4 | Job Objects | Windows | FREE | Process |
| 5 | Direct | All | - | None (warning logged) |

### Container Configuration

Sandboxed containers run with:

- **Non-root user**: `sandbox` (UID 1000)
- **Read-only root filesystem**: No writes outside `/tmp` and `/output`
- **No capabilities**: `--cap-drop=ALL`
- **No new privileges**: `--security-opt=no-new-privileges`
- **Resource limits**: Memory (512MB default), PIDs (100 default), CPU (1 core)

---

## Isolation Guarantees

### 1. Filesystem Isolation

| Mount Type | Access | Purpose |
|------------|--------|---------|
| `/input` | Read-only | Customer evidence files |
| `/data` | Read-only | Pattern database |
| `/output` | Read-write | Analysis results |
| `/tmp` | Read-write | Ephemeral workspace |

**Guarantee**: Tools cannot read files outside mounted paths. Path traversal attempts are blocked.

### 2. Network Isolation

| Tool Category | Network Policy | Allowed Destinations |
|---------------|---------------|----------------------|
| Forensics | `none` | No network access |
| Knowledge (local) | `none` | No network access |
| Gateway | `allow_outbound_https` | Port 443 only |
| Salesforce | `allow_outbound_https` | Port 443 only |

**Guarantee**: Network-isolated tools cannot exfiltrate data. HTTPS-allowed tools can only reach port 443.

### 3. Write Prevention

Tools do not write directly to the host filesystem. Instead, they return `suggested_writes` in the output envelope:

```json
{
  "success": true,
  "output": {...},
  "suggested_writes": [
    {
      "path": "JIRA_DRAFT.md",
      "content": "...",
      "mode": "create",
      "reason": "JIRA escalation draft"
    }
  ]
}
```

Main Claude reviews and executes suggested writes with user approval.

**Exception**: `case-private-comment` is the ONLY tool that writes to external systems (Salesforce). It requires explicit `approved: true` parameter (HITL gate).

---

## Threat Model

### In Scope

| Threat | Mitigation |
|--------|------------|
| Malicious HAR/bundle files | Container isolation, resource limits |
| Path traversal in params | Absolute path requirement, `.validate()` checks |
| Command injection | Static command strings, no shell expansion |
| Data exfiltration via network | Network policy enforcement per tool |
| Container escape | Non-root user, no capabilities, read-only rootfs |
| Archive bombs | Size limits (100MB archive, 20MB per file) |
| Symlink attacks | Symlink rejection in tar extraction |

### Out of Scope

| Threat | Reason |
|--------|--------|
| Compromised container runtime | Trust boundary at runtime level |
| Supply chain attacks on base image | Use official images, pin versions |
| Kernel vulnerabilities | OS-level concern, not application-level |

---

## Audit Logging

Every sandboxed execution produces audit metadata:

```json
{
  "execution_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2026-02-22T10:30:45Z",
  "sandbox_backend": "podman",
  "container_id": "abc123def456",
  "image": "cappy-sandbox:latest",
  "network_policy": "none",
  "mounts": [
    {"host_path": "/path/to/case", "container_path": "/input", "read_only": true}
  ],
  "duration_ms": 150
}
```

This enables:
- Forensic reconstruction of tool executions
- Detection of anomalous behavior
- Compliance auditing

---

## Security Testing

### Automated Tests

Located in `tests/sandbox_security.rs`:

- Path traversal prevention
- Mount isolation verification
- Network policy enforcement
- Write operation control
- Command injection prevention
- Edge case handling

Run with:
```bash
cargo test --test sandbox_security
```

### Manual Verification

```bash
# Verify container isolation
podman run --rm cappy-sandbox:latest cat /etc/passwd  # Should show container's file

# Verify network isolation
podman run --rm --network none cappy-sandbox:latest curl https://google.com  # Should fail

# Verify non-root execution
podman run --rm cappy-sandbox:latest whoami  # Should show "sandbox"
```

---

## Configuration

### Environment Variables

```bash
# Enable/disable sandbox (default: enabled)
CAPPY_SANDBOX_ENABLED=1

# Policy level (strict, standard, permissive)
CAPPY_SANDBOX_POLICY=standard

# Container image
CAPPY_SANDBOX_IMAGE=cappy-sandbox:latest

# Tools to bypass sandbox (comma-separated, for debugging)
CAPPY_SANDBOX_BYPASS=

# Fallback behavior when container unavailable
# fail = error out
# warn = warn and proceed without sandbox
CAPPY_SANDBOX_FALLBACK=warn
```

### Policy Levels

| Policy | Resource Limits | Network | Use Case |
|--------|----------------|---------|----------|
| `strict` | 256MB, 50 PIDs | Always none | Untrusted files |
| `standard` | 512MB, 100 PIDs | Per-tool | Normal operation |
| `permissive` | 1GB, 200 PIDs | Per-tool | Large bundles |

---

## Vulnerability Reporting

Report security vulnerabilities to: kf.tan@lightarchitects.io

Do NOT open public issues for security vulnerabilities.

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-02-22 | Initial security documentation |
