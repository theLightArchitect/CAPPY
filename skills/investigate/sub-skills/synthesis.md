# /synthesis - Narrative Synthesis & Phase 5 Preparation

**Version**: 1.0.0
**Purpose**: Create investigation narrative and prepare Phase 5 research direction
**Agent**: CAPPY_SYNTHESIS_AGENT
**Created**: 2026-02-05

---

## Hook Integration

### NarrativeCoherenceHook (Optional - Informational)

**When to Trigger**: After agent synthesizes investigation narrative (informational, not blocking)

**Hook Specification**:
```rust
NarrativeCoherenceHook::trigger(
  narrative: String,              // Synthesized 3-phase narrative
  coherence_score: f32,           // Logical flow quality (0.0-1.0)
  phase_2_to_3_connection: String, // How triage connects to evidence
  phase_3_to_4_connection: String, // How evidence connects to hypothesis
  gaps_identified: Vec<String>,   // Evidence gaps found during synthesis
  phase_5_targets: Vec<String>,   // Specific focus areas for validation
  details: HashMap {
    "narrative_length": usize,
    "assumptions_count": usize,
    "cited_evidence_count": usize,
    "evidence_gaps_count": usize
  }
)
```

**Agent Implementation**:
1. Synthesize narrative connecting Phases 2-4
2. Evaluate logical flow: triage → evidence → hypothesis
3. Identify evidence gaps and assumptions
4. Generate specific Phase 5 research targets
5. Trigger hook (informational - hook doesn't block)

**Hook Response Handling**:
```
Hook returns: { coherence_feedback: String, gap_severity: Vec<(String, String)> }

Response is INFORMATIONAL ONLY (doesn't block Phase 5):
  → Agent presents feedback to Claude for awareness
  → Claude may use feedback to refine Phase 5 strategy
  → Investigation proceeds regardless
```

**Why This Hook**: Synthesis is validation-stage work. Hook provides feedback on narrative quality but doesn't block progress (unlike gates). Helps Claude understand which gaps are most critical for Phase 5.

---

## Identity

The `/synthesis` skill provides **narrative structure templates** and **Phase 5 preparation guidance** to:
1. Synthesize findings from Phases 2-4 into coherent narrative
2. Connect triage → evidence → hypothesis logically
3. Identify key assumptions for Phase 5 verification
4. Generate specific Phase 5 research targets
5. Document evidence gaps

Agent CAPPY_SYNTHESIS creates investigation narrative and prepares Claude for Phase 5 validation.

---

## Narrative Structure Templates

### The Three-Phase Connection Pattern

```markdown
Phase 2 → Phase 3 → Phase 4 Narrative Arc:

[BEGINNING] What we found during triage
  "During triage, we identified [Pattern Name] (P-###) with [Confidence]% confidence."
  Fact: Concrete pattern match from Phase 2

[MIDDLE] What evidence revealed
  "Phase 3 evidence analysis confirms this: [Key Evidence Facts]."
  Facts: Direct quotes from Phase 3 evidence
  Citations: file:line or HAR:entry references

[END] What hypothesis explains
  "Our analysis indicates the root cause is: [Root Cause]."
  Logic: How evidence points to root cause
  Assumptions: What we haven't verified yet

[PHASE 5 FOCUS] What we need to verify
  "For Phase 5 validation, we'll focus on: [Key Verification Targets]"
  Action: Specific things to research/verify
```

### Narrative Templates by Root Cause Type

#### Template A: Configuration/Setting Issue

```markdown
During triage, we identified pattern P-### (Configuration) with X% confidence.

Phase 3 evidence analysis reveals:
- Configuration setting missing: [Setting Name]
- Or: Configuration has wrong value: [Expected] vs [Actual]
- Or: Feature disabled: [Feature]
- Evidence: HAR shows [error], logs show [message]

Our hypothesis: The root cause is [Setting] is incorrectly configured.
- Why this explains the symptoms: [Logic]
- What would fix it: [Solution]

For Phase 5 validation, we'll verify:
1. Confirm current [Setting] value in customer config
2. Compare against expected value from docs
3. Validate that changing [Setting] to [Expected] resolves issue
```

#### Template B: Rate Limiting/Throttling

```markdown
During triage, we identified pattern P-### (Rate Limiting) with X% confidence.

Phase 3 evidence analysis reveals:
- HTTP 429 (Too Many Requests) errors at [timestamp]
- Request rate was [Rate] requests/minute
- Started failing at [specific time]
- Evidence: HAR shows 429 responses, logs show rate messages

Our hypothesis: The root cause is rate limiting on [API/Integration].
- API/Integration has limit: [Limit] requests/minute
- Customer's request rate exceeded this: [Rate] > [Limit]
- Retry behavior created cascade: [Description]

For Phase 5 validation, we'll verify:
1. Confirm API rate limit in [Product] [Version] documentation
2. Verify customer's request rate during test window
3. Calculate if [Rate] exceeds [Limit] = rate limit exceeded
4. Review API docs for workarounds (pagination, backoff, etc.)
```

#### Template C: Timeout/Connectivity

```markdown
During triage, we identified pattern P-### (Timeout) with X% confidence.

Phase 3 evidence analysis reveals:
- Timeout occurred at [timestamp]
- Timeout after [X] seconds
- Integration type: [webhook/polling/API]
- Evidence: HAR shows timeout, logs show connection lost

Our hypothesis: The root cause is [Timeout Type] on [Integration].
- [Integration] has timeout: [X] seconds
- Customer's [operation] takes longer than [X] seconds
- Or: Network/connectivity issue causing delay

For Phase 5 validation, we'll verify:
1. Confirm [Integration] timeout setting in customer config
2. Check if customer's [operation] normally takes [Y] seconds
3. Verify network connectivity to [endpoint]
4. Review firewall rules or TLS certificate issues
```

#### Template D: Version/Feature Compatibility

```markdown
During triage, we identified pattern P-### (Version Compatibility) with X% confidence.

Phase 3 evidence analysis reveals:
- Customer is using [Product] [Version] [Build]
- Feature [Feature] behaves differently in this version
- Or: Feature [Feature] deprecated in [Version]
- Evidence: Docs show [behavior], customer sees [different behavior]

Our hypothesis: The root cause is [Feature] incompatibility with [Version].
- [Feature] changed in [Version]: [How it changed]
- Customer's workflow relied on old behavior
- Or: [Feature] removed/deprecated in [Version]

For Phase 5 validation, we'll verify:
1. Confirm feature behavior in [Product] [Version] documentation
2. Review changelog for [Version] regarding [Feature]
3. Identify migration path or workaround
4. Test recommended alternative: [Alternative]
```

---

## Phase 5 Preparation Framework

### Identifying Key Assumptions

```yaml
Process:
  1. Extract all assumptions from hypothesis
  2. For each assumption, ask: "Is this verified in Phase 3 evidence?"
  3. If NO → Mark as KEY_ASSUMPTION
  4. If YES → Don't include (already verified)

Example:

Hypothesis: "API has 100 req/min rate limit, customer exceeded it"

Assumption 1: "API has 100 req/min rate limit"
  Verified in Phase 3? NO (inferred from 429 errors)
  Mark as: KEY_ASSUMPTION
  Verification Strategy: Search customer config, API docs

Assumption 2: "Customer's request rate exceeded 100 req/min"
  Verified in Phase 3? YES (HAR shows 150 req/min at failure time)
  Mark as: VERIFIED, don't include as assumption

Assumption 3: "Retry behavior created cascade"
  Verified in Phase 3? PARTIALLY (see retries, not full logic)
  Mark as: WEAK_ASSUMPTION
  Verification Strategy: Search integration config, docs
```

### Generating Research Targets

```yaml
For Each Key Assumption, Generate Phase 5 Target:

Assumption: "API rate limit is 100 req/min"
Target 1 (Primary):
  description: "Confirm API endpoint rate limit value"
  sources: ["Customer config file", "API documentation"]
  search_terms: ["rate_limit", "max_requests", "throttle", "429"]
  expected: "Find configuration or documentation confirming limit value"

Target 2 (Secondary):
  description: "Verify customer's actual request rate"
  sources: ["HAR file", "Integration logs", "Monitoring data"]
  search_terms: ["request rate", "requests per minute", "polling interval"]
  expected: "Confirm customer was sending > limit requests"

Target 3 (Secondary):
  description: "Check for API workarounds or best practices"
  sources: ["API docs", "TAC playbooks", "Cortex documentation"]
  search_terms: ["backoff", "retry logic", "pagination", "batch"]
  expected: "Identify recommended approach for high-volume requests"
```

### Evidence Gaps Analysis

```yaml
Evidence Gap Types:

1. Missing Configuration
   gap: "Customer's integration polling interval unknown"
   impact: "Can't confirm if polling rate caused issue"
   verification: "Search customer config files in Phase 5"

2. Unverified Behavior
   gap: "Webhook retry logic not confirmed"
   impact: "Can't confirm cascade effect"
   verification: "Search integration docs, customer logs"

3. Timing Uncertainty
   gap: "Exact timeout value unknown (assumed 30s)"
   impact: "Can't confirm if timeout caused issue"
   verification: "Check customer config or product docs"

4. Architecture Assumptions
   gap: "Webhook integration architecture not confirmed"
   impact: "May be wrong integration type"
   verification: "Verify with customer or JIRA history"

Phase 5 Focus:
  - Prioritize gaps that affect root cause confirmation
  - Mark gaps that would change hypothesis if filled
  - Document workaround for gaps that can't be filled
```

---

## Validation Output Schema

```json
{
  "status": "NARRATIVE_READY",
  "investigation_summary": {
    "phase_2": "[Pattern found at X% confidence]",
    "phase_3": "[Key evidence facts from logs/HAR]",
    "phase_4": "[Root cause hypothesis]"
  },
  "narrative": "[Complete 3-phase narrative connecting findings]",
  "narrative_structure": "CONFIGURATION|RATE_LIMITING|TIMEOUT|VERSION_COMPATIBILITY",
  "coherence_check": {
    "phases_connected": true,
    "logic_flow": "STRONG|WEAK",
    "assumptions_explicit": true
  },
  "validation_focus": [
    "Primary: [Most critical thing to verify]",
    "Secondary: [Supporting verification]",
    "Secondary: [Workaround verification]"
  ],
  "key_assumptions": [
    {
      "assumption": "[Unverified claim]",
      "verified_confidence": 0.3,
      "importance": "CRITICAL|HIGH|MEDIUM",
      "test_strategy": "[How to verify in Phase 5]",
      "research_sources": ["[Source 1]", "[Source 2]"]
    }
  ],
  "evidence_gaps": [
    {
      "gap": "[What's missing]",
      "impact": "[How it affects hypothesis]",
      "verification": "[How to fill in Phase 5]"
    }
  ],
  "phase_5_preparation": {
    "primary_target": "[Main focus for Phase 5]",
    "secondary_targets": ["[Support 1]", "[Support 2]"],
    "sources_to_search": [
      "Customer config files",
      "JIRA tickets",
      "Cortex documentation",
      "Product changelog"
    ],
    "search_strategy": "[Recommended approach]",
    "success_criteria": "[What constitutes verification]"
  }
}
```

---

## Agent Methods Available

```python
def get_narrative_template(root_cause_type: str) -> str:
    """Returns narrative template for root cause type"""
    # CONFIGURATION, RATE_LIMITING, TIMEOUT, VERSION_COMPATIBILITY

def create_narrative(phase_summaries: Dict) -> str:
    """Creates 3-phase narrative from phase summaries"""
    # Takes: {phase_2: "...", phase_3: "...", phase_4: "..."}
    # Returns: Coherent narrative connecting all phases

def extract_validation_targets(hypothesis: str, inv_context: Dict) -> List[str]:
    """Extracts what needs verification in Phase 5"""
    # Returns: [primary_target, secondary_1, secondary_2, ...]

def extract_key_assumptions(inv_context: Dict) -> List[Dict]:
    """Extracts unverified assumptions from Phase 4"""
    # Returns: [{assumption, verified, importance, test_strategy}]

def extract_evidence_gaps(inv_context: Dict) -> List[Dict]:
    """Identifies evidence gaps from Phase 3"""
    # Returns: [{gap, impact, verification}]

def prepare_phase_5(hypothesis: str, assumptions: List, gaps: List) -> Dict:
    """Generates Phase 5 research direction"""
    # Returns: {primary_target, secondary_targets, sources, search_strategy}

def validate_narrative_coherence(narrative: str, inv_context: Dict) -> bool:
    """Validates that narrative connects all phases logically"""
    # Returns: True if coherent, False otherwise
```

---

## How CAPPY_SYNTHESIS_AGENT Uses This Skill

1. **Load skill** - `synthesis_skill = load_skill("/synthesis")`
2. **Summarize phases** - Extract summaries from Phase 2, 3, 4 in inv_context.json
3. **Get template** - `template = synthesis_skill.get_narrative_template(root_cause_type)`
4. **Create narrative** - `narrative = synthesis_skill.create_narrative({phase_2, phase_3, phase_4})`
5. **Validate coherence** - `synthesis_skill.validate_narrative_coherence(narrative, inv_context)`
6. **Extract targets** - `targets = synthesis_skill.extract_validation_targets(hypothesis)`
7. **Extract assumptions** - `assumptions = synthesis_skill.extract_key_assumptions(inv_context)`
8. **Extract gaps** - `gaps = synthesis_skill.extract_evidence_gaps(inv_context)`
9. **Prepare Phase 5** - `phase_5_prep = synthesis_skill.prepare_phase_5(hypothesis, assumptions, gaps)`
10. **Return** - Return `{narrative, validation_focus, key_assumptions, phase_5_preparation}`

---

## Success Criteria

Narrative synthesis is working when:
- ✅ Narrative connects Phase 2 → Phase 3 → Phase 4 logically
- ✅ All assumptions explicitly stated
- ✅ Evidence gaps identified
- ✅ Phase 5 validation targets specific and actionable
- ✅ Claude can dive directly into Phase 5 without re-summarizing
- ✅ Key assumptions ranked by importance

---

**Skill Version**: 1.0.0
**Last Updated**: 2026-02-05
**Status**: Ready for CAPPY_SYNTHESIS_AGENT integration
