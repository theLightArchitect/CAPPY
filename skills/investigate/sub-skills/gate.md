# /gate - Phase Gate Specifications

**Version**: 1.0.0
**Purpose**: Define quality gates for phase boundaries
**Agent**: CAPPY_GUARDIAN_AGENT
**Created**: 2026-02-05

---

## Hook Integration (CRITICAL)

### Phase-Specific Gate Hooks

Each phase has a corresponding **gate hook** that enforces quality standards. CAPPY_GUARDIAN_AGENT triggers these hooks to validate phase completion:

#### Phase 2: ConfidenceGateHook
**Trigger**: After Phase 2 triage completes with confidence score
```rust
ConfidenceGateHook::trigger(
  confidence: f32,           // Triage overall_confidence (0.0-1.0)
  threshold: f32,            // 0.90 for Phase 2
  passed: bool,              // confidence >= threshold
  details: HashMap {
    "patterns_found": usize,
    "best_pattern_score": f32,
    "evidence_count": usize
  }
)
```

#### Phase 3: CompletenessGateHook
**Trigger**: After Phase 3 evidence extraction with sufficiency score
```rust
CompletenessGateHook::trigger(
  evidence_sufficiency: f32, // Phase 3 evidence coverage (0.0-1.0)
  threshold: f32,            // 0.99 for Phase 3
  passed: bool,              // evidence_sufficiency >= threshold
  details: HashMap {
    "evidence_files_analyzed": usize,
    "key_evidence_found": Vec<String>,
    "gaps_identified": Vec<String>
  }
)
```

#### Phase 4: HypothesisCoherenceGateHook
**Trigger**: After Phase 4 hypothesis design with alignment score
```rust
HypothesisCoherenceGateHook::trigger(
  alignment_score: f32,      // Hypothesis-evidence alignment (0.0-1.0)
  threshold: f32,            // 0.90 for Phase 4
  passed: bool,              // alignment_score >= threshold
  details: HashMap {
    "weak_assumptions": Vec<String>,
    "contradictions": Vec<String>,
    "verification_rate": f32
  }
)
```

#### Phase 5-6: SolutionValidationGateHook
**Trigger**: After Phase 5 validation with solution quality score
```rust
SolutionValidationGateHook::trigger(
  solution_quality: f32,     // Solution validation score (0.0-1.0)
  threshold: f32,            // 0.99 for Phase 5-6
  passed: bool,              // solution_quality >= threshold
  details: HashMap {
    "sources_verified": usize,
    "customer_implications": String,
    "implementation_complexity": String
  }
)
```

#### Phase 7: VerificationCheckpointGateHook (P-007)
**Trigger**: Before Phase 7 deliverables with verification rate
```rust
VerificationCheckpointGateHook::trigger(
  verification_rate: f32,    // Percentage of claims with citations (0.0-1.0)
  threshold: f32,            // 0.99 for Phase 7
  passed: bool,              // verification_rate >= threshold
  details: HashMap {
    "total_claims": usize,
    "verified_claims": usize,
    "uncited_claims": usize,
    "verification_deadline": bool  // Hard block before delivery
  }
)
```

**Agent Implementation Pattern**:
1. Collect gate-specific metric from phase completion
2. Compare metric against threshold
3. Trigger corresponding hook with results
4. If hook passes: Allow phase advancement
5. If hook blocks: Return recovery options to Claude

**Why These Hooks**: Gates are quality control points. They ensure phases are complete before moving forward. Blocking here prevents low-quality deliverables.

---

## Identity

The `/gate` skill defines **hard quality gates** that block phase advancement if thresholds aren't met. Each gate has:
- **Name**: What the gate checks
- **Threshold**: Minimum/maximum value required
- **Operation**: How to compare (>=, <=, >, <, range, exact)
- **Recovery Options**: What to do if gate blocks
- **Phase**: When this gate applies

Gates are **hard blocks** - investigation cannot advance without passing them.

---

## Phase Gate Specifications

### Phase 2: Confidence Gate (≥90%)

**Purpose**: Ensure triage identified sufficient patterns with high confidence

**Specification**:
```yaml
phase: 2
name: "confidence_gate"
operation: ">="
threshold: 0.90
extract_field: "overall_confidence"

pass_reason: "Confidence meets or exceeds 90% threshold"
fail_reason: "Confidence below 90% threshold"
fail_details: "Triage confidence insufficient. Need more/better evidence or different pattern."

next_action: "Ready for Phase 3 evidence extraction."

recovery_options:
  - priority: 1
    action: "Request HAR file from customer"
    impact: "HAR analysis typically increases confidence 10-15%"
    effort: "Customer communication + re-analyze"
  - priority: 2
    action: "Get manual guidance from Kevin"
    impact: "Human review may identify patterns Claude missed"
    effort: "Escalation"
  - priority: 3
    action: "Explore different pattern"
    impact: "May identify alternative root cause with higher confidence"
    effort: "Restart triage with different hypothesis"

confidence_ranges:
  0.0 - 0.5: "Very low - restart with different approach"
  0.5 - 0.7: "Low - request additional evidence"
  0.7 - 0.9: "Medium - borderline, needs recovery"
  0.9 - 0.95: "High - passes gate"
  0.95 - 1.0: "Very high - strong pattern identified"
```

---

### Phase 3: Completeness Gate (≥99% Evidence Sufficiency)

**Purpose**: Ensure evidence extraction is comprehensive enough to support hypothesis

**Specification**:
```yaml
phase: 3
name: "completeness_gate"
operation: ">="
threshold: 0.99
extract_field: "evidence_sufficiency"

pass_reason: "Evidence sufficiency meets or exceeds 99% threshold"
fail_reason: "Evidence sufficiency below 99% threshold"
fail_details: "Evidence extraction incomplete. Key evidence missing or unverified."

next_action: "Ready for Phase 4 hypothesis design."

recovery_options:
  - priority: 1
    action: "Request specific missing evidence"
    impact: "Targeted evidence request fills gaps"
    effort: "Identify gaps, customer communication"
  - priority: 2
    action: "Deep-dive additional log analysis"
    impact: "Manual parsing may find evidence Claude missed"
    effort: "Additional forensics work"
  - priority: 3
    action: "Accept partial evidence, flag assumptions"
    impact: "Proceed with caveats, mark unverified"
    effort: "Mark Phase 5 focus for verification"

sufficiency_ranges:
  0.0 - 0.5: "Very incomplete - major evidence gaps"
  0.5 - 0.8: "Incomplete - significant gaps"
  0.8 - 0.95: "Borderline - some gaps remain"
  0.95 - 0.99: "Nearly complete - close to passing"
  0.99 - 1.0: "Complete - passes gate"
```

---

### Phase 4: Hypothesis Coherence Gate (≥90% Evidence Alignment)

**Purpose**: Ensure hypothesis aligns with extracted evidence and has no contradictions

**Specification**:
```yaml
phase: 4
name: "hypothesis_coherence_gate"
operation: ">="
threshold: 0.90
extract_field: "hypothesis_coherence_score"

pass_reason: "Hypothesis alignment meets or exceeds 90% threshold"
fail_reason: "Hypothesis alignment below 90% threshold"
fail_details: "Hypothesis doesn't align with evidence or has contradictions."

next_action: "Ready for Phase 5 validation research."

recovery_options:
  - priority: 1
    action: "Revise hypothesis based on evidence gaps"
    impact: "Adjust hypothesis to match actual evidence"
    effort: "Re-analyze Phase 4 work"
  - priority: 2
    action: "Identify and resolve contradictions"
    impact: "Remove conflicting assumptions"
    effort: "Deep evidence review"
  - priority: 3
    action: "Request additional evidence for verification"
    impact: "New evidence may reconcile gaps"
    effort: "Customer communication"

coherence_ranges:
  0.0 - 0.5: "Poor - major gaps, restart hypothesis"
  0.5 - 0.7: "Weak - significant misalignment"
  0.7 - 0.9: "Borderline - needs revision"
  0.9 - 0.95: "Strong - passes gate"
  0.95 - 1.0: "Excellent - highly coherent"

contradiction_types:
  - "Assumption contradicts Phase 3 evidence"
  - "Hypothesis claims unverified in evidence"
  - "Timeline doesn't match evidence events"
  - "Architecture doesn't match customer setup"
```

---

### Phase 5: Solution Validation Gate (≥99% Quality)

**Purpose**: Ensure solution is validated against multiple sources and customer-specific

**Specification**:
```yaml
phase: 5
name: "solution_validation_gate"
operation: ">="
threshold: 0.99
extract_field: "solution_quality_score"

pass_reason: "Solution quality meets or exceeds 99% threshold"
fail_reason: "Solution quality below 99% threshold"
fail_details: "Solution not sufficiently validated or has gaps."

next_action: "Ready for Phase 6 solution design and Phase 7 deliverables."

recovery_options:
  - priority: 1
    action: "Validate against additional sources"
    impact: "Multi-source validation increases confidence"
    effort: "Additional research"
  - priority: 2
    action: "Test solution with customer data"
    impact: "Proof-of-concept validation"
    effort: "Hands-on testing"
  - priority: 3
    action: "Get peer review from TAC"
    impact: "Expert validation"
    effort: "Escalation"

quality_ranges:
  0.0 - 0.5: "Poor solution - restart"
  0.5 - 0.8: "Weak - needs more validation"
  0.8 - 0.95: "Borderline - close to passing"
  0.95 - 0.99: "Strong - nearly passing"
  0.99 - 1.0: "Excellent - passes gate"

validation_sources:
  - "Customer configuration files"
  - "JIRA tickets for this product/version"
  - "Cortex official documentation"
  - "TAC playbooks"
  - "Knowledge base articles"
  - "Community forums/discussions"
```

---

### Phase 7: Deliverable QA Gate (≥99% Quality)

**Purpose**: Final quality check before sending to customer - highest bar

**Specification**:
```yaml
phase: 7
name: "deliverable_qa_gate"
operation: ">="
threshold: 0.99
extract_field: "deliverable_quality_score"

pass_reason: "Deliverable quality meets or exceeds 99% threshold"
fail_reason: "Deliverable quality below 99% threshold"
fail_details: "Deliverable has issues - incomplete, unclear, or inaccurate."

next_action: "Ready to deliver to customer."

recovery_options:
  - priority: 1
    action: "Fix specific issues in deliverable"
    impact: "Resolve quality gaps"
    effort: "Editing/clarification"
  - priority: 2
    action: "Enhance with additional analysis"
    impact: "Provide more value/clarity"
    effort: "Additional research"
  - priority: 3
    action: "Get peer review from TAC"
    impact: "Expert feedback before delivery"
    effort: "Escalation"

quality_ranges:
  0.0 - 0.7: "Poor - major issues, needs rewrite"
  0.7 - 0.9: "Fair - significant issues"
  0.9 - 0.95: "Good - needs improvement"
  0.95 - 0.99: "Very good - close to passing"
  0.99 - 1.0: "Excellent - passes gate"

deliverable_checks:
  - "All claims have citations (file:line, HAR:entry, etc.)"
  - "No placeholder text remains"
  - "Professional language throughout"
  - "Clear problem statement"
  - "Evidence clearly presented"
  - "Root cause explained logically"
  - "Solution actionable for customer"
  - "Grammar/spelling correct"
  - "Formatting consistent"
  - "No sensitive data exposed"
```

---

## Agent Methods Available

```python
def get_gate_for_phase(phase: int) -> GateSpec:
    """Returns gate specification for given phase"""
    # Returns: GateSpec with name, threshold, operation, recovery_options

def get_threshold(phase: int) -> float:
    """Returns numerical threshold for phase"""
    # phase 2 → 0.90
    # phase 3 → 0.99
    # phase 4 → 0.90
    # phase 5 → 0.99
    # phase 7 → 0.99

def get_recovery_options(phase: int, gate_spec: GateSpec, actual_value: float) -> List[str]:
    """Returns prioritized recovery options if gate is blocked"""
    # Returns options ranked by priority and estimated impact

def get_fail_details(phase: int) -> str:
    """Returns detailed failure message for blocked gate"""
    # Explains why gate blocked and what's needed

def get_next_action(phase: int) -> str:
    """Returns what happens if gate passes"""
    # e.g., "Ready for Phase 5 validation research"
```

---

## How CAPPY_GUARDIAN_AGENT Uses This Skill

1. **Load skill** - `guardian_skill = load_skill("/gate")`
2. **Get gate spec** - `gate_spec = guardian_skill.get_gate_for_phase(phase)`
3. **Extract value** - `gate_value = extract_from_phase_output(phase_output)`
4. **Compare** - `if gate_value >= gate_spec.threshold: PASS else: BLOCK`
5. **Return feedback** - Return `{status: PASSED/BLOCKED, gate_value, threshold, options}`

---

## Success Criteria

Gate enforcement is working when:
- ✅ Each phase has appropriate gate applied
- ✅ Gate blocks if threshold not met
- ✅ Recovery options are ranked by priority
- ✅ Agent can extract gate value from phase output
- ✅ All gate failures logged to inv_context.json
- ✅ Investigation cannot advance without gate passing

---

**Skill Version**: 1.0.0
**Last Updated**: 2026-02-05
**Status**: Ready for CAPPY_GUARDIAN_AGENT integration

---

### Phase 7: Verification Checkpoint Gate (≥99% Verification Rate)

**Purpose**: Ensure all deliverables meet verification standard before customer delivery - P-007 final quality gate

**Specification**:
```yaml
phase: 7
name: "verification_checkpoint_gate"
operation: ">="
threshold: 0.99
extract_field: "verification_rate"

pass_reason: "Verification rate meets or exceeds 99% threshold - ready for customer delivery"
fail_reason: "Verification rate below 99% threshold - some claims unverified"
fail_details: "Not all claims have been verified against Phase 3 evidence. Recovery options available: additional research, delivery with caveats, or Kevin override."

next_action: "Approved for Phase 7 deliverable generation and customer delivery."

recovery_options:
  - priority: 1
    action: "Return to Phase 5 for additional verification research"
    impact: "Research and verify remaining unverified claims"
    effort: "Additional research time (varies by claim complexity)"
    expected_improvement: "10-30% increase in verification rate"
  - priority: 2
    action: "Deliver with verification caveats"
    impact: "Mark unverified claims explicitly in deliverables as 'unverified assumption'"
    effort: "Minimal - update deliverable templates"
    expected_improvement: "N/A - same verification rate, but documented"
  - priority: 3
    action: "Get Kevin approval to override gate"
    impact: "Proceed to delivery regardless of verification rate"
    effort: "Escalation"
    expected_improvement: "N/A - bypasses gate"

verification_ranges:
  0.0 - 0.70: "Poor - major verification gaps, recommend Phase 5 return"
  0.70 - 0.90: "Fair - significant unverified claims, recommend Phase 5 return"
  0.90 - 0.95: "Good - borderline, needs improvement"
  0.95 - 0.99: "Very good - close to passing"
  0.99 - 1.0: "Excellent - passes gate, delivery ready"

verification_sources:
  - "Phase 3 evidence extraction (primary - file:line citations)"
  - "Phase 5 research validation (secondary - documentation/specs)"
  - "Contradiction detection (if no contradictions, strengthens verification)"

verification_metrics:
  claims_total: "Total claims extracted during investigation"
  claims_verified: "Claims with Phase 3 citations to evidence files"
  claims_unverified: "Claims without citations or unresolved contradictions"
  verification_rate: "claims_verified / claims_total"
  unverified_list: "Explicit list of unverified claims for recovery planning"
  contradictions: "Any conflicting claims that require resolution"

gate_blocking_conditions:
  - "verification_rate < 0.99 AND claims_unverified > 0"
  - "unresolved contradictions exist"
  - "manual verification found discrepancies not yet fixed"
```

---

## How CAPPY_GUARDIAN_AGENT Uses Phase 7 Gate

1. **Load skill** - `guardian_skill = load_skill("/gate")`
2. **Get gate spec** - `gate_spec = guardian_skill.get_gate_for_phase(7)`
3. **Extract value** - `verification_checkpoint = extract_from_inv_context("verification_checkpoint")`
4. **Compare** - `verification_rate = verification_checkpoint.verification_rate`
5. **Decision** - `if verification_rate >= 0.99: PASS else: BLOCK`
6. **Return feedback** - Return structured response with recovery options

### Phase 7 Gate Recovery Workflow

**If gate BLOCKS** (verification_rate < 0.99):

```
BLOCKED State
    │
    ├─ Option 1: Return to Phase 5 (Additional Research)
    │   ├─ Target unverified claims from unverified_list
    │   ├─ Search documentation/JIRA/KB for citations
    │   ├─ Run Phase 5 validation on targeted claims
    │   └─ Re-run Phase 7 gate check
    │
    ├─ Option 2: Deliver with Caveats (Document Unverified)
    │   ├─ Mark each unverified claim in deliverables
    │   ├─ Label as "Based on analysis but unverified in evidence"
    │   ├─ Customer knows which parts are verified vs assumption
    │   └─ Still proceed to Phase 7 deliverable generation
    │
    └─ Option 3: Get Kevin Approval (Override Gate)
        ├─ Escalate to Kevin with verification_rate and unverified_list
        ├─ Kevin decides: deliver as-is or request changes
        ├─ If approved: proceed to Phase 7
        └─ If rejected: return to Phase 5
```

---

**Phase 7 Gate Version**: 2.0.0
**Added**: 2026-02-06
**Praetorian Alignment**: ✅ Hard gate enforcement, explicit recovery paths, no ambiguity
