# Structured Logging & Stacktracing for CAPPY v2.0

**Version**: 1.0.0

**Purpose**: Comprehensive structured logging with correlation IDs, enabling full observability across CAPPY orchestration and sub-skill execution.

---

## Logging Architecture

### Entry Point Logging (src/mcp.rs)

**When**: Every /investigate skill invocation starts

**Generates**: Correlation ID (UUID v4)

```json
{
  "timestamp": "2026-02-10T14:30:45.123Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "INFO",
  "component": "mcp_entry",
  "operation": "skill_invocation",
  "skill": "investigate",
  "case_id": "SF-12345678",
  "message": "Investigation skill invoked"
}
```

**Propagation**: Correlation ID included in ALL subsequent logs for this investigation

### Phase Transition Logging

**When**: Each phase boundary (Phase 0→1, 1→2, 2→3, etc.)

```json
{
  "timestamp": "2026-02-10T14:35:00.456Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "INFO",
  "component": "CAPPY",
  "phase": 2,
  "operation": "phase_transition",
  "from_phase": 1,
  "to_phase": 2,
  "status": "INITIATED",
  "message": "Starting Phase 2: Triage"
}
```

### Orchestrator Execution Logging

**When**: Each MPC tool invocation (triage_case, analyze_evidence, etc.)

```json
{
  "timestamp": "2026-02-10T14:35:15.789Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "INFO",
  "component": "orchestrator",
  "tool_name": "triage_case",
  "phase": 2,
  "operation": "tool_execution",
  "status": "INITIATED",
  "parameters": {
    "symptom": "XSOAR integration timeout",
    "product": "XSOAR",
    "min_confidence": 60
  }
}
```

### Orchestrator Result Logging

**When**: MPC tool execution completes

```json
{
  "timestamp": "2026-02-10T14:35:45.234Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "INFO",
  "component": "orchestrator",
  "tool_name": "triage_case",
  "phase": 2,
  "operation": "tool_execution",
  "status": "COMPLETED",
  "result_summary": {
    "patterns_found": 5,
    "confidence_score": 82,
    "execution_time_ms": 1234
  }
}
```

### Skill Invocation Logging

**When**: CAPPY invokes a sub-skill (/cappy-guru, /cappy-guardian, etc.)

```json
{
  "timestamp": "2026-02-10T14:35:50.567Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "INFO",
  "component": "CAPPY",
  "operation": "skill_invocation",
  "skill_name": "cappy-guru",
  "phase": 3,
  "purpose": "Validate citations for all evidence claims",
  "status": "INITIATED"
}
```

### Skill Result Logging

**When**: Sub-skill execution completes

```json
{
  "timestamp": "2026-02-10T14:36:15.890Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "INFO",
  "component": "CAPPY",
  "operation": "skill_invocation",
  "skill_name": "cappy-guru",
  "phase": 3,
  "status": "COMPLETED",
  "result": {
    "claims_validated": 42,
    "citation_rate": 0.95,
    "uncited_claims": 2,
    "execution_time_ms": 567
  }
}
```

### Gate Enforcement Logging

**When**: Phase gate checks occur (P-003 confidence, P-006 completeness, etc.)

```json
{
  "timestamp": "2026-02-10T14:36:20.123Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "INFO",
  "component": "gate",
  "operation": "gate_check",
  "gate_name": "Phase 2→3 Confidence Gate (P-003)",
  "phase": 2,
  "threshold": 70,
  "measured_value": 82,
  "status": "PASSED",
  "margin": 12
}
```

### Gate Blocking Logging

**When**: Phase gate blocks investigation

```json
{
  "timestamp": "2026-02-10T14:40:00.456Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "WARN",
  "component": "gate",
  "operation": "gate_block",
  "gate_name": "Phase 2→3 Confidence Gate (P-003)",
  "phase": 2,
  "threshold": 70,
  "measured_value": 62,
  "status": "BLOCKED",
  "recovery_options": 3
}
```

### Error Logging with Stack Traces

**When**: Errors occur during investigation

```json
{
  "timestamp": "2026-02-10T14:45:30.789Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "ERROR",
  "component": "triage_case",
  "operation": "tool_execution",
  "phase": 2,
  "error_type": "MissingEvidenceError",
  "error_message": "HAR file not found in case directory",
  "error_context": {
    "case_directory": "/case/SF-12345678",
    "expected_file": "evidence/traffic.har",
    "available_files": ["env.log", "server.log"]
  },
  "stack_trace": "at triage_case (src/orchestrators/triage_case.rs:245)\n  at handle_tool_call (src/mcp.rs:189)\n  at process_request (src/mcp.rs:95)"
}
```

### Performance Metrics Logging

**When**: Investigation phase completes

```json
{
  "timestamp": "2026-02-10T14:50:00.234Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "INFO",
  "component": "metrics",
  "operation": "phase_complete",
  "phase": 4,
  "metrics": {
    "phase_duration_seconds": 125,
    "orchestrator_calls": 3,
    "skill_invocations": 4,
    "gate_checks": 1,
    "patterns_evaluated": 47,
    "claims_registered": 8,
    "memory_used_mb": 234,
    "cpu_percent": 45
  }
}
```

### Investigation Summary Logging

**When**: Investigation completes (end of Phase 4 or escalation)

```json
{
  "timestamp": "2026-02-10T14:55:30.567Z",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "level": "INFO",
  "component": "investigation",
  "operation": "investigation_complete",
  "case_id": "SF-12345678",
  "status": "COMPLETE_READY_FOR_PHASE_5",
  "summary": {
    "total_duration_seconds": 325,
    "phases_completed": 4,
    "patterns_matched": 5,
    "hypothesis_confidence": 88,
    "environment_match": 98,
    "citations_valid": true,
    "escalation_triggered": false
  }
}
```

---

## Correlation ID Propagation

Every log entry includes the correlation ID set at investigation start:

```
Investigation Start:
  correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479

Phase 2 (Triage):
  correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479
  ├─ triage_case call: correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479
  ├─ /cappy-guru invocation: correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479
  └─ CAPPY_GUARDIAN gate: correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479

Phase 3 (Evidence):
  correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479
  ├─ analyze_evidence call: correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479
  └─ /cappy-curator registration: correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479

Phase 4 (Hypothesis):
  correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479
  ├─ cappy_synthesis call: correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479
  ├─ /cappy-sherlock validation: correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479
  └─ /cappy-recon environment check: correlation_id = f47ac10b-58cc-4372-a567-0e02b2c3d479
```

**Enable Log Filtering**: All logs for a specific investigation queryable by correlation ID:
```bash
grep "f47ac10b-58cc-4372-a567-0e02b2c3d479" investigation.log

# Or with jq for JSON parsing:
jq '.[] | select(.correlation_id == "f47ac10b-58cc-4372-a567-0e02b2c3d479")' investigation.log
```

---

## Log Configuration

### Log File Locations

```
~/.cappy/logs/
├── investigation.log          # All investigation logs (rotating)
├── investigation_errors.log   # Error logs only
├── investigation_metrics.log  # Performance metrics only
└── investigation/
    ├── SF-12345678/
    │   ├── log.jsonl          # Investigation-specific logs
    │   └── metrics.json       # Phase completion metrics
    └── SF-87654321/
        └── ...
```

### Log Format

All logs are **structured JSON, one entry per line** (JSONL format):

```
{"timestamp":"2026-02-10T14:30:45.123Z","correlation_id":"...","level":"INFO",...}
{"timestamp":"2026-02-10T14:30:46.456Z","correlation_id":"...","level":"INFO",...}
{"timestamp":"2026-02-10T14:30:47.789Z","correlation_id":"...","level":"ERROR",...}
```

**Advantages**:
- Machine-readable and parseable
- Queryable with jq, grep, or log aggregators
- Can be imported into ELK stack, Splunk, or CloudWatch
- One entry per line enables streaming analysis

### Log Levels

| Level | When Used | Examples |
|-------|-----------|----------|
| **DEBUG** | Detailed diagnostic info | Parameter values, intermediate results |
| **INFO** | General informational | Phase transitions, tool invocations, skill calls |
| **WARN** | Warning conditions | Gate blocks, low confidence, missing optional data |
| **ERROR** | Error conditions | Tool failures, missing required files, exceptions |
| **FATAL** | Critical failures | Unrecoverable errors, investigation termination |

---

## Log Querying Examples

### Find all logs for a specific investigation

```bash
grep "SF-12345678" ~/.cappy/logs/investigation.log | jq .
```

### Find all gate blocks

```bash
jq '.[] | select(.operation == "gate_block")' ~/.cappy/logs/investigation.log
```

### Find all errors in past 24 hours

```bash
jq 'select(.level == "ERROR" and .timestamp > "2026-02-09T14:30:00Z")' ~/.cappy/logs/investigation.log
```

### Show execution timeline for a correlation ID

```bash
jq 'select(.correlation_id == "f47ac10b-58cc-4372-a567-0e02b2c3d479") |
    {timestamp, component, operation, status}' ~/.cappy/logs/investigation.log |
    jq -s 'sort_by(.timestamp)'
```

### Calculate performance metrics by phase

```bash
jq '.[] |
    select(.operation == "phase_complete") |
    {phase, duration_seconds: .metrics.phase_duration_seconds}' ~/.cappy/logs/investigation.log
```

---

## Integration with inv_context.json

Log entries reference investigation state via correlation ID:

```json
{
  "investigation_id": "SF-12345678",
  "correlation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "logging": {
    "enabled": true,
    "log_file": "~/.cappy/logs/investigation/SF-12345678/log.jsonl",
    "log_level": "INFO",
    "metrics_enabled": true
  },
  "log_summary": {
    "total_entries": 47,
    "errors": 0,
    "warnings": 2,
    "average_phase_duration_ms": 1234
  }
}
```

---

## Performance Baseline

### Expected Metrics (Phase 1 v1.7.0)

| Component | Target | Notes |
|-----------|--------|-------|
| Triage (Phase 2) | <2000ms | Pattern database lookup + ranking |
| Evidence (Phase 3) | <5000ms | HAR parsing + timeline correlation |
| Hypothesis (Phase 4) | <3000ms | Synthesis + validation checks |
| Gate checks | <500ms | Confidence/completeness/coherence scoring |
| Skill invocations | <1000ms | Sub-skill network latency |
| **Total Investigation** | **<15000ms** | Phases 2-4 combined |

### Metrics Tracked

- **Execution Time**: Per phase, per tool, per skill
- **Tool Efficiency**: Patterns evaluated per second
- **Gate Performance**: Confidence scoring latency
- **Memory Usage**: Baseline vs peak during investigation
- **CPU Utilization**: Percent utilization by phase
- **Skill Performance**: Response time per sub-skill
- **Error Rate**: Failures per 1000 invocations

---

## Logging Best Practices

### DO

✅ Include correlation ID in ALL logs
✅ Use structured JSON format
✅ Log phase transitions
✅ Log tool invocations and results
✅ Log skill calls with parameters
✅ Log gate decisions and margins
✅ Log errors with full stack traces
✅ Log performance metrics
✅ Include context (case_id, phase, tool_name)

### DON'T

❌ Log raw customer data (PII, credentials)
❌ Mix structured and unstructured logs
❌ Use different timestamp formats
❌ Omit component/operation fields
❌ Lose correlation ID in nested calls
❌ Log overly verbose debug info in production
❌ Create unstructured log files
❌ Forget to include error context

---

## Monitoring & Alerting

### Critical Log Patterns to Monitor

1. **Gate Blocks**: Multiple blocks indicate investigation difficulty
2. **Errors**: Any ERROR level should trigger review
3. **Slow Phases**: Phase duration > 2x baseline
4. **Citation Failures**: Evidence citation rate < 95%
5. **Escalations**: Auto-trigger when CAPPY_CORTEX invoked

### Dashboard Metrics

Recommended metrics for observability dashboard:

- Investigation success rate (% reaching Phase 7)
- Gate pass rate (% of gates passed)
- Average investigation duration
- Top failing patterns/symptoms
- Citation coverage trend
- Error rate by phase
- Performance distribution (histogram)

---

**Document**: CAPPY v2.0 Structured Logging & Stacktracing
**Purpose**: Comprehensive logging guide for Phase 6 implementation
**Version**: 1.0.0
**Status**: READY FOR IMPLEMENTATION

