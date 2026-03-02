# /recon - Architecture Knowledge & Validation

**Version**: 1.0.0
**Purpose**: Validate Phase 4 hypothesis against customer's actual environment
**Agent**: CAPPY_RECON_AGENT
**Created**: 2026-02-05

---

## Hook Integration (CRITICAL)

### ArchitectureValidationHook

**When to Trigger**: After agent validates hypothesis against customer's known environment

**Hook Specification**:
```rust
ArchitectureValidationHook::trigger(
  hypothesis: String,              // Root cause hypothesis from Phase 4
  customer_product: String,        // From inv_context.json environment
  customer_version: String,        // From inv_context.json environment
  customer_build: String,          // From inv_context.json environment
  customer_architecture: String,   // From inv_context.json environment
  hypothesis_matches_env: bool,    // Does hypothesis fit customer's setup?
  mismatches: Vec<String>,         // What doesn't match
  cortex_docs_findings: Vec<String>, // What we found in docs/JIRA/Confluence
  details: HashMap {
    "product_match": bool,
    "version_support": bool,
    "architecture_compatibility": bool,
    "integration_type_matches": bool
  }
)
```

**Agent Implementation**:
1. Extract from inv_context.json: customer_product, version, build, architecture
2. Research customer's actual product/version combination:
   - Search Cortex docs for version info
   - Check JIRA tickets for this customer/product/version
   - Search Confluence for architecture patterns
3. Compare hypothesis assumptions against actual environment:
   - If hypothesis assumes webhook integration but customer uses REST polling → MISMATCH
   - If hypothesis assumes v8.8+ feature but customer is on v7.x → MISMATCH
   - If hypothesis assumes clustering but customer is standalone → MISMATCH
4. Identify what would fix the mismatch
5. Trigger hook with detailed findings

**Hook Response Handling**:
```
Hook returns: { status: MATCH|MISMATCH, recommendation: String }

If MATCH:
  → Return "PASSED: Hypothesis matches customer's actual environment" to agent

If MISMATCH:
  → Return "BLOCKED: Hypothesis doesn't match customer's environment" with details
  → Agent provides recovery options to Claude:
    Option 1: Revise hypothesis to match customer's actual setup
    Option 2: Deep-dive research on customer's setup (unusual config?)
    Option 3: Escalate to Kevin for customer clarification on setup
```

**Why This Hook**: Prevents wasted effort validating a hypothesis against the wrong environment. Architecture mismatch is a deal-breaker that must be caught before Phase 5.

---

## Identity

The `/recon` skill provides **customer architecture knowledge** and **validation rules** to ensure hypothesis matches:
1. Customer's actual product/version/build
2. Customer's actual integration architecture
3. Known constraints and limitations
4. Integration patterns for the product

Agent CAPPY_RECON researches customer's actual setup and validates hypothesis alignment before Phase 5.

---

## Architecture Knowledge Base

### Product Architectures

#### XSOAR (Palo Alto Automation & Orchestration)

```yaml
XSOAR:
  versions:
    6.x:
      release_date: "2021-2022"
      support: "legacy"
      architecture: "Container + database"
      integrations:
        - webhook
        - rest_polling
        - syslog
        - api
    7.x:
      release_date: "2022-2023"
      support: "standard"
      architecture: "Container + database"
      integrations:
        - webhook (supported)
        - rest_polling (supported)
        - syslog (supported)
    8.x:
      release_date: "2023-2025"
      support: "current"
      architecture: "Container + Kubernetes ready"
      integrations:
        - webhook (deprecated in 8.8+, REST recommended)
        - rest_polling (primary)
        - syslog (supported)
        - api (primary)
      major_changes:
        - "8.8+: Webhook deprecation"
        - "8.9+: REST polling recommended"
        - "Rate limiting on fetch operations"

  integration_patterns:
    webhook:
      description: "Push-based - external system sends events to XSOAR"
      timeout: "30 seconds"
      constraints:
        - "Requires internet-facing endpoint"
        - "Firewall rules needed"
        - "Deprecated in 8.8+"
      failure_modes:
        - "Webhook unreachable (firewall)"
        - "Timeout after 30 seconds"
        - "SSL/TLS cert issues"
    rest_polling:
      description: "Pull-based - XSOAR polls external API"
      timeout: "Configurable, default 60s"
      constraints:
        - "Requires outbound connectivity"
        - "Rate limiting may apply"
        - "Polling interval matters"
      failure_modes:
        - "Rate limit exceeded (HTTP 429)"
        - "Timeout (connectivity, DNS, TLS)"
        - "Authentication failures"
        - "Rate limiting cascades"
```

#### XSIAM (Palo Alto Extended Security Information & Analytics)

```yaml
XSIAM:
  versions:
    1.x:
      release_date: "2024"
      support: "current"
      architecture: "SaaS only"
      collectors:
        - agent-based
        - agentless (API, Syslog)
      integrations:
        - api (primary)
        - syslog
        - syslog-ng

  architecture:
    components:
      - "Tenants (multi-tenant)"
      - "Data sources (collectors)"
      - "Correlation engine (XQL)"
      - "Alert engine"
    constraints:
      - "API rate limits per tenant"
      - "XQL query limits"
      - "Data retention policies"

  failure_modes:
    - "Collector connectivity lost"
    - "Rate limiting on API calls"
    - "XQL query timeout"
    - "TLS/authentication issues"
```

#### XDR (Palo Alto Cortex XDR)

```yaml
XDR:
  versions:
    2.x:
      release_date: "2024+"
      support: "current"
      architecture: "SaaS only"

  components:
    - "Agents (endpoint protection)"
    - "API (data ingestion)"
    - "Backend (cloud-based)"

  failure_modes:
    - "Agent connectivity issues"
    - "Certificate problems"
    - "Rate limiting"
    - "Data ingestion delays"
```

---

## Research Sources

### Priority Order for Architecture Validation

```yaml
Priority 1: Customer's Own Environment
  - env.log (from Phase 0-1) → MOST AUTHORITATIVE
  - Customer-provided architecture diagram
  - Customer configuration files
  - What customer told us

Priority 2: JIRA Tickets
  - Search for customer's case history
  - Version-specific issues
  - Known constraints documented
  - Previous solutions

Priority 3: Cortex Official Documentation
  - Product version specs
  - Integration documentation
  - Known limitations
  - Architecture guides

Priority 4: Confluence (Internal TAC)
  - TAC playbooks for product/version
  - Known patterns
  - Best practices
  - Integration guides

Priority 5: Community/Public Sources
  - Forums
  - User groups
  - GitHub issues
  - Blog posts
```

### Source Research Methods

```yaml
JIRA Research:
  search_strategy:
    - "Product + version: XSOAR 8.9.0"
    - "Customer name or environment"
    - "Integration type: webhook, polling"
    - "Error type: timeout, rate limit"

  what_to_look_for:
    - "Known issues in this version"
    - "Customer's previous problems"
    - "How issues were resolved"
    - "Constraints mentioned"

  example_search: 'summary ~ "XSOAR 8.9" AND text ~ "webhook"'

Cortex Docs Research:
  search_strategy:
    - "Product documentation for exact version"
    - "Integration guides"
    - "Architecture diagrams"
    - "Limitations and constraints"

  what_to_look_for:
    - "Supported integration types"
    - "Timeout values"
    - "Rate limiting info"
    - "Deprecated features"

  example_search: "XSOAR 8.9 webhook integration architecture"

Confluence Research:
  search_strategy:
    - "TAC playbooks for product"
    - "Known patterns"
    - "Best practices"

  what_to_look_for:
    - "Troubleshooting guides"
    - "Integration patterns"
    - "Common issues"

Customer Config Files:
  what_to_extract:
    - "Integration type (webhook/polling/syslog)"
    - "Polling interval (if polling)"
    - "Timeout settings"
    - "Rate limit settings"
    - "Authentication method"
```

---

## Validation Rules

### Architecture Alignment Check

```yaml
Validation Process:

1. Extract Hypothesis Architecture Assumptions
   hypothesis: "Webhook integration timeout"
   assumptions:
     - Uses webhook integration (NOT REST polling)
     - Webhook timeout is 30 seconds
     - Webhook is configured and enabled

2. Extract Customer's Actual Architecture from env.log
   from_env_log:
     - product: "XSOAR"
     - version: "8.9.0-2464525"
     - build: "2464525"
     - detected_features: [...]

3. Research Customer's Actual Architecture
   research:
     - JIRA: Found CAC-12345 "Disabled webhooks, using REST polling"
     - Docs: "XSOAR 8.9.0: Webhooks deprecated, use REST polling"
     - Confluence: "Customer setup: REST polling every 60 seconds"

4. Detect Mismatch
   comparison:
     hypothesis_expects: webhook
     customer_actually_uses: REST polling
     status: ARCHITECTURE_MISMATCH

5. Return Feedback
   mismatch: "You hypothesized webhook timeout, but customer uses REST polling"
   recommendation: "Revise to: REST polling timeout. Check polling config, interval, rate limits."
```

### Constraint Validation

```yaml
Known Constraints by Product/Version:

XSOAR 8.x + REST Polling:
  - "Rate limit: Check API endpoint docs"
  - "Polling interval matters"
  - "Request rate may exceed limit"
  - "Rate limit errors: HTTP 429"
  - "Timeout: 60 seconds default"

XSOAR 8.x Webhooks (8.8+):
  - "DEPRECATED - avoid assuming webhook"
  - "Timeout: 30 seconds"
  - "Requires internet-facing endpoint"
  - "Firewall rules required"

XSIAM + API Integration:
  - "Multi-tenant architecture"
  - "API rate limits per tenant"
  - "TLS/certificate issues common"
  - "Authentication complexity"
```

---

## Validation Output Schema

```json
{
  "status": "ARCHITECTURE_MATCH|ARCHITECTURE_MISMATCH|ARCHITECTURE_UNKNOWN",
  "hypothesis": "Root cause is webhook integration timeout",
  "customer_architecture": {
    "product": "XSOAR",
    "version": "8.9.0-2464525",
    "build": "2464525",
    "source": "env.log + JIRA CAC-12345"
  },
  "detected_architecture": {
    "integration_type": "REST polling (not webhooks)",
    "polling_interval": 60,
    "timeout_seconds": 60,
    "rate_limiting": true,
    "sources": ["JIRA ticket", "Cortex docs", "env.log"]
  },
  "research_findings": {
    "jira": [
      {
        "ticket": "CAC-12345",
        "summary": "Disabled webhooks due to firewall constraints",
        "relevant_to_hypothesis": true
      }
    ],
    "cortex_docs": [
      {
        "topic": "XSOAR 8.9.0 Webhooks",
        "content": "Webhooks deprecated in 8.8+, REST polling recommended",
        "relevant_to_hypothesis": true
      }
    ],
    "confluence": [
      {
        "article": "XSOAR Customer Setup Guide",
        "content": "Customer uses REST polling with 60s interval",
        "relevant_to_hypothesis": true
      }
    ]
  },
  "verdict": "ARCHITECTURE_MISMATCH",
  "mismatch_detail": "You hypothesized webhook timeout, but customer uses REST polling on v8.9.0",
  "recommendation": "Revise hypothesis to: REST polling timeout. Focus on polling config, request rates, rate limit thresholds.",
  "next_action": "Phase 5: Verify polling timeout settings and customer request rate"
}
```

---

## Agent Methods Available

```python
def get_architecture_knowledge(product: str, version: str) -> Dict:
    """Returns known architecture for product/version"""
    # XSOAR 8.9.0 → integration types, timeouts, constraints

def get_integration_patterns(product: str) -> List[Dict]:
    """Returns integration patterns for product"""
    # XSOAR → [webhook, REST polling, syslog, API]

def get_research_sources() -> List[str]:
    """Returns priority-ordered research sources"""
    # [JIRA, Cortex Docs, Confluence, ...]

def get_known_constraints(product: str, version: str) -> List[str]:
    """Returns constraints for product/version"""
    # XSOAR 8.x → [rate limiting, timeout=60s, ...]

def get_architecture_mismatch_recovery(actual: str, hypothesized: str) -> Dict:
    """Returns correction guidance if mismatch detected"""
    # {mismatch, recommendation, next_action}

def validate_against_environment(hypothesis: str, env_data: Dict) -> Dict:
    """Validates hypothesis against customer environment"""
    # Returns: {status, mismatch_detail, recommendation}
```

---

## How CAPPY_RECON_AGENT Uses This Skill

1. **Load skill** - `recon_skill = load_skill("/recon")`
2. **Extract customer architecture** - `customer_arch = extract_from_inv_context(env.log, JIRA)`
3. **Extract hypothesis assumptions** - `hyp_arch = extract_architecture_assumptions(hypothesis)`
4. **Research actual architecture** - Call JIRA, Cortex Docs, Confluence via call_tool
5. **Detect mismatch** - `if hyp_arch != detected_arch: MISMATCH`
6. **Return feedback** - `{status, mismatch_detail, recommendation, research_findings}`

---

## Success Criteria

Architecture validation is working when:
- ✅ Customer's actual architecture extracted from env.log
- ✅ Hypothesis architecture assumptions identified
- ✅ Research conducted from JIRA, Docs, Confluence
- ✅ Mismatches detected and reported
- ✅ Recommendations clear and actionable
- ✅ Phase 5 focus adjusted based on actual architecture

---

**Skill Version**: 1.0.0
**Last Updated**: 2026-02-05
**Status**: Ready for CAPPY_RECON_AGENT integration
