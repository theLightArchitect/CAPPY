# /sherlock - Hypothesis Validation Rules

**Version**: 1.0.0
**Purpose**: Validate Phase 4 hypothesis against Phase 3 evidence
**Agent**: CAPPY_SHERLOCK_AGENT
**Created**: 2026-02-05

---

## Hook Integration (CRITICAL)

### HypothesisCoherenceHook (Part of Gate System)

**When to Trigger**: After agent validates hypothesis alignment with evidence

**Hook Specification**:
```rust
HypothesisCoherenceHook::trigger(
  hypothesis: String,         // Root cause hypothesis from Phase 4
  alignment_score: f32,       // Evidence alignment score (0.0-1.0)
  threshold: f32,             // 0.85 (hard threshold for Phase 4)
  weak_assumptions: Vec<String>, // Unverified claims in hypothesis
  contradictions: Vec<String>, // Evidence contradicting hypothesis
  confidence_delta: f32,       // Adjustment to overall confidence
  details: HashMap {
    "matches": usize,         // Claims supported by evidence
    "gaps": usize,            // Claims not supported by evidence
    "evidence_alignment_rate": f32
  }
)
```

**Agent Implementation**:
1. Extract hypothesis claims from Phase 4 output
2. For each claim, find supporting evidence from Phase 3
3. Calculate alignment_score = matches / total_claims
4. Identify weak assumptions (inferred, not directly verified)
5. Check for contradictions between hypothesis and evidence
6. Trigger hook with all results

**Hook Response Handling**:
```
Hook returns: { passed: bool, confidence_adjustment: float, recommendations: Vec<String> }

If passed (alignment >= 0.85):
  → Return "PASSED: Hypothesis aligns with evidence" to agent
  → Confidence can increase by delta (typically +0.05 to +0.15)

If blocked (alignment < 0.85):
  → Return "BLOCKED: Weak hypothesis-evidence alignment" with weak_assumptions and contradictions
  → Agent provides recovery options to Claude:
    Option 1: Accept weak assumptions, flag for Phase 5 verification
    Option 2: Revise hypothesis to align better with evidence
    Option 3: Request Kevin guidance on alternative hypotheses
```

**Why This Hook**: Hypothesis is the core of the investigation. If it doesn't align with evidence, Phase 5 validation will be wasted. This hook catches misalignment early.

---

## Identity

The `/sherlock` skill defines **hypothesis validation rules** that check if a proposed root cause:
1. Aligns with extracted evidence
2. Has no contradictions with evidence
3. Has verifiable assumptions
4. Is logically coherent
5. Accounts for all key facts

Agent CAPPY_SHERLOCK validates Phase 4 hypothesis and identifies weak assumptions for Phase 5 verification.

---

## Validation Framework

### What Constitutes Valid Hypothesis

A hypothesis is **VALID** when:

```yaml
Alignment:
  - All direct claims in hypothesis are supported by Phase 3 evidence
  - Evidence patterns match hypothesis explanation
  - Timeline matches hypothesis sequence

Contradictions:
  - No evidence contradicts hypothesis
  - No assumptions conflict with facts
  - No timeline mismatches

Assumptions:
  - All unverified claims clearly marked as assumptions
  - Assumptions are testable (can be verified in Phase 5)
  - Assumptions are documented with what would verify them

Completeness:
  - Hypothesis accounts for all key evidence
  - Hypothesis explains observed behavior
  - No major facts left unexplained

Coherence:
  - Logical flow from evidence to conclusion
  - Root cause explains symptoms
  - Solution would resolve root cause
```

---

## Validation Rules

### Rule 1: Evidence Alignment

**Check**: Does hypothesis align with Phase 3 evidence?

```yaml
Evidence Alignment Checks:
  - For each claim in hypothesis:
      ✓ Find supporting evidence in Phase 3
      ✓ Check claim type matches evidence type
      ✓ Check confidence level reasonable

  - Matching patterns:
      evidence: "HTTP 429 at 20:51:45Z"
      hypothesis_claim: "Rate limit exceeded"
      match: YES (429 = rate limit error)

      evidence: "Request rate 150 req/min"
      hypothesis_claim: "Exceeded limit of 100 req/min"
      match: YES (150 > 100)

  - Non-matching patterns:
      evidence: "Timeout after 30 seconds"
      hypothesis_claim: "Webhook integration timeout"
      match: WEAK if no webhook in architecture
        → Flag as: "Assumption: Uses webhook integration"
```

**Agent Action**:
```python
matches = []
gaps = []

for claim in hypothesis_claims:
    supporting = find_evidence(claim, phase_3_evidence)
    if supporting:
        matches.append({claim, evidence: supporting})
    else:
        gaps.append(claim)

alignment_score = len(matches) / len(hypothesis_claims)
```

---

### Rule 2: Contradiction Detection

**Check**: Does evidence contradict hypothesis?

```yaml
Contradiction Types:
  1. Temporal Contradiction
     hypothesis: "Error occurs at 10:30:45Z"
     evidence: "Error log shows 10:30:42Z"
     conflict: YES (3-second difference significant?)

  2. Behavioral Contradiction
     hypothesis: "Integration disabled"
     evidence: "Integration ran at 10:30:45Z"
     conflict: YES (can't run if disabled)

  3. Data Type Contradiction
     hypothesis: "String overflow in field"
     evidence: "Field is integer type"
     conflict: YES (can't overflow string in int field)

  4. Architecture Contradiction
     hypothesis: "Webhook integration timeout"
     evidence: "Customer uses REST polling, no webhooks"
     conflict: YES (wrong integration type)

Severity Levels:
  CRITICAL: Hypothesis impossible given evidence
  HIGH: Hypothesis contradicted by evidence
  MEDIUM: Timing mismatch or unclear
  LOW: Minor discrepancy, likely explanation exists
```

**Agent Action**:
```python
contradictions = []

for assumption in hypothesis_assumptions:
    contradicting = find_contradiction(assumption, phase_3_evidence)
    if contradicting:
        contradictions.append({
            assumption: assumption,
            contradiction: contradicting,
            severity: assess_severity(assumption, contradicting)
        })

has_critical = any(c.severity == CRITICAL for c in contradictions)
```

---

### Rule 3: Assumption Validation

**Check**: Which assumptions are unverified?

```yaml
Assumption Types:
  1. Evidence-based Assumption
     "API has 100 req/min rate limit"
     evidence: "HTTP 429 errors when request rate > 100"
     verification: WEAK (inferred, not directly stated)
     action: Mark as assumption, verify in Phase 5

  2. Configuration Assumption
     "Polling interval is 60 seconds"
     evidence: "Integration config not in bundle"
     verification: UNVERIFIED (not in Phase 3)
     action: Retrieve config in Phase 5

  3. Behavioral Assumption
     "System retry behavior causes cascade"
     evidence: "Single failed request, then 5 retries"
     verification: WEAK (inferred from observed pattern)
     action: Verify in logs/docs in Phase 5

  4. Capability Assumption
     "Webhook timeout fires after 30s"
     evidence: "Not tested in customer environment"
     verification: UNVERIFIED (from documentation, not observation)
     action: Verify against actual customer setup in Phase 5

Test Criteria:
  ✓ Can we test this assumption in Phase 5?
  ✓ What evidence would verify/refute it?
  ✓ How important is it to the root cause?
```

**Agent Action**:
```python
assumptions = extract_assumptions(hypothesis)
key_assumptions = []

for assumption in assumptions:
    verification_level = assess_verification(assumption, phase_3_evidence)
    if verification_level < 0.80:  # Below 80% verified
        key_assumptions.append({
            assumption: assumption,
            verified: verification_level,
            test_in_phase_5: generate_test_strategy(assumption)
        })
```

---

## Confidence Delta Calculation

**How Validation Changes Confidence**

```yaml
Confidence Adjustment Rules:

Base Confidence (from Phase 2 triage):
  phase_2_confidence: 0.85  # e.g., 85%

Evidence Alignment Score:
  matches: 10/12 claims = 0.83
  delta: 0.83 - 1.0 = -0.17

Contradiction Check:
  critical_contradictions: 0
  high_contradictions: 0
  delta: 0.0 (none found)

Assumption Risk:
  unverified_assumptions: 2
  importance: medium, low
  delta: -0.05

Final Confidence:
  0.85 + (-0.17) + 0.0 + (-0.05) = 0.63

Interpretation:
  Phase 2 confidence: 85%
  Phase 4 validation confidence: 63%
  Delta: -22% (significant downward adjustment)
  Meaning: Phase 3 evidence less aligned than Phase 2 suggested
  Action: Revalidate triage assumptions, adjust hypothesis
```

---

## Validation Output Schema

```json
{
  "status": "VALID|WEAK|INVALID",
  "confidence_before": 0.85,
  "confidence_after": 0.63,
  "confidence_delta": -0.22,
  "evidence_alignment": {
    "matches": [
      {claim: "HTTP 429 errors", evidence: "HAR entry 145", strength: 0.95},
      {claim: "Rate limit", evidence: "429 = rate limit", strength: 0.90}
    ],
    "gaps": [
      {claim: "API limit is 100 req/min", evidence: "NONE", strength: 0.0}
    ],
    "alignment_score": 0.83
  },
  "contradictions": {
    "critical": [],
    "high": [],
    "medium": [],
    "low": [],
    "total": 0
  },
  "key_assumptions": [
    {
      "assumption": "API rate limit is 100 req/min",
      "verified": 0.30,
      "importance": "CRITICAL",
      "test_strategy": "Search customer config, API docs, JIRA history"
    },
    {
      "assumption": "Webhook integration NOT in use",
      "verified": 0.70,
      "importance": "HIGH",
      "test_strategy": "Verify in JIRA, customer architecture docs"
    }
  ],
  "weak_assumptions_count": 2,
  "recommendation": "Hypothesis WEAK - proceed with Phase 5 focus on verifying key assumptions"
}
```

---

## Agent Methods Available

```python
def get_validation_rules() -> Dict:
    """Returns all validation rules for hypothesis checking"""
    # Returns: {alignment_rules, contradiction_rules, assumption_rules}

def get_evidence_patterns() -> List[str]:
    """Returns patterns to look for in evidence"""
    # What types of evidence support hypothesis?

def get_assumption_patterns() -> Dict:
    """Returns what counts as unverified assumption"""
    # How to identify assumptions vs facts

def get_contradiction_types() -> List[str]:
    """Returns types of contradictions to check for"""
    # 4 types: Temporal, Behavioral, DataType, Architecture

def assess_assumption_importance(assumption: str) -> str:
    """Returns importance level of assumption"""
    # CRITICAL, HIGH, MEDIUM, LOW

def get_phase_5_focus(weak_assumptions: List) -> Dict:
    """Returns what to focus on in Phase 5"""
    # Based on weak assumptions identified
```

---

## How CAPPY_SHERLOCK_AGENT Uses This Skill

1. **Load skill** - `sherlock_skill = load_skill("/sherlock")`
2. **Extract hypothesis claims** - `direct_claims = extract_direct_claims(hypothesis)`
3. **Extract assumptions** - `assumptions = extract_assumptions(hypothesis)`
4. **Check alignment** - `matches, gaps = check_alignment(claims, phase_3_evidence)`
5. **Check contradictions** - `contradictions = check_contradictions(hypothesis, phase_3_evidence)`
6. **Identify weak assumptions** - `weak = [a for a in assumptions if not verified(a)]`
7. **Calculate confidence delta** - `new_confidence = adjust_confidence(matches, gaps, weak)`
8. **Return feedback** - `{status, confidence_before, confidence_after, key_assumptions, recommendation}`

---

## Success Criteria

Hypothesis validation is working when:
- ✅ All hypothesis claims checked against Phase 3 evidence
- ✅ Contradictions detected and reported
- ✅ Weak/unverified assumptions clearly identified
- ✅ Confidence adjusted based on evidence alignment
- ✅ Phase 5 focus generated from key assumptions
- ✅ Agent provides actionable feedback to Main Claude

---

**Skill Version**: 1.0.0
**Last Updated**: 2026-02-05
**Status**: Ready for CAPPY_SHERLOCK_AGENT integration
