# Architecture Decision Records

## ADR-001: Rust Over TypeScript

**Status**: Accepted
**Date**: 2026-01-01

### Context
The initial prototype used 7 separate TypeScript orchestrators with individual MCP tool registrations. Performance was acceptable but cold starts were slow (3-5s) and memory usage was high (~400MB with 7 Node.js processes).

### Decision
Consolidate all orchestrators into a single Rust binary using a meta-orchestrator pattern. One `call_tool` entry point routes to domain-specific orchestrators internally.

### Consequences
- **Cold start**: 3-5s -> 150ms (20x improvement)
- **Memory**: ~400MB -> ~30MB (13x reduction)
- **Single binary**: Simplified deployment and MCP configuration
- **Trade-off**: Higher development cost per feature (Rust vs TypeScript)

---

## ADR-002: Hook Pipeline Architecture

**Status**: Accepted
**Date**: 2026-01-15

### Context
Investigation quality depends on systematic verification at each phase. Manual checklists were inconsistent and easy to skip.

### Decision
Implement a pre/post hook pipeline where hooks execute before and after every tool invocation. Hooks are prioritized (lower number = higher priority) and can modify, enrich, or reject tool executions.

### Hook Categories
| Category | Examples | Purpose |
|----------|----------|---------|
| Validation | ParameterValidator, PhaseGate | Enforce input correctness |
| Evidence | ClaimCapture, EvidenceChain | Track citation provenance |
| Quality | ConfidenceAuditor, NarrativeCoherence | Prevent hallucination |
| Security | PiiGuard, DataGuard | Protect sensitive data |
| Operational | Cache, DebugTracer, FeedbackCollector | Performance and observability |

### Consequences
- 30+ hooks execute on every tool call (typical overhead: 5-15ms)
- Investigation quality is enforced programmatically, not by convention
- New validation rules can be added without modifying orchestrator code
- Trade-off: Hook ordering bugs can cause subtle failures

---

## ADR-003: Container Sandbox for Tool Execution

**Status**: Accepted
**Date**: 2026-02-22

### Context
CAPPY processes customer evidence files (HAR captures, log bundles, screenshots). These files could contain malicious content. Tools that parse these files need isolation.

### Decision
Route all tool executions through sandboxed containers with:
- Read-only input mounts
- Network isolation for forensics tools
- Non-root execution
- Resource limits (memory, PIDs, CPU)
- Multi-backend support: Podman > Docker > Bubblewrap > Job Objects

### Consequences
- Strong isolation guarantees for untrusted file processing
- Graceful degradation when containers unavailable (with warnings)
- Tools return `suggested_writes` instead of writing directly
- Trade-off: ~50ms overhead per sandboxed execution

---

## ADR-004: Multi-Provider AI Routing

**Status**: Accepted
**Date**: 2026-01-20

### Context
Different investigation tasks have different cost/quality/latency requirements. Triage benefits from fast responses; synthesis needs highest quality.

### Decision
Implement 3-tier provider routing:

| Tier | Provider | Cost | Use Case |
|------|----------|------|----------|
| Tier 0 | Ollama (local) | Free | Pattern matching, classification |
| Tier 1 | Claude | $$$ | Synthesis, hypothesis generation |
| Tier 2 | Gemini | $$ | Research, document analysis |

### Consequences
- Cost optimization: ~60% of operations use free local models
- Latency optimization: Local inference for time-sensitive operations
- Trade-off: Tier 0 quality lower for complex reasoning tasks
- Fallback chain ensures availability if any provider is down

---

## ADR-005: Pattern Database Design

**Status**: Accepted
**Date**: 2026-01-01

### Context
Investigation acceleration depends on matching observed symptoms against known issues. Patterns need confidence levels and rich metadata.

### Decision
JSON-based pattern database with:
- Unique pattern IDs (P001-P540+)
- Confidence levels: Definitive, Strong, Moderate
- Match patterns (regex) and exclusion patterns
- Causality chains for root cause tracing
- Product and component classification

### Consequences
- Fast local matching (no network required)
- Patterns are version-controlled and auditable
- Trade-off: JSON doesn't scale beyond ~10K patterns (sufficient for current needs)
- Pattern quality depends on manual curation and validation
