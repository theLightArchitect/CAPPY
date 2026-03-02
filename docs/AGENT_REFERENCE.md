# CAPPY Investigation Agents - Architecture Reference

## Overview

CAPPY uses a **Main Claude Primary, CAPPY Assistant** model where Main Claude handles all primary responsibilities (tool execution, user interaction, phase decisions), and CAPPY is invoked for validation, gate checking, and guidance.

**Key Model**: Main Claude does the work first, then invokes CAPPY for validation. CAPPY reads sub-skill files for rules and returns recommendations. Main Claude makes all final decisions.

---

## Architecture

```
Main Claude (Primary)
    │
    ├── 1. Execute MCP tool (triage_case, analyze_evidence, etc.)
    ├── 2. Parse results, extract findings
    ├── 3. Manual verification (jq, grep)
    │
    └── 4. Invoke CAPPY for validation:
            ├── Read sub-skill: gate.md → Check thresholds
            ├── Read sub-skill: curator.md → Register claims
            ├── Read sub-skill: sherlock.md → Validate coherence
            └── Return: gate_status, recommendations
```

---

## CAPPY Agent

**Name**: CAPPY (Investigation Assistant)
**Type**: Task Agent (Invoked for Validation)
**Version**: 2.1.0
**Role**: Validation, Gate Checking, Claim Registration

### When Invoked

Main Claude invokes CAPPY AFTER completing primary phase work:

| Phase | Main Claude Does First | Then CAPPY Does |
|-------|------------------------|-----------------|
| 2 | Execute triage_case, parse patterns | Validate patterns, check confidence gate (≥90%) |
| 3 | Execute analyze_evidence, extract errors | Validate citations, check completeness gate (≥99%) |
| 4 | Execute cappy_synthesis, generate hypothesis | Validate coherence (≥90%), check environment |
| 5-6 | Execute validate_solution | Check validation gate (≥99%) |
| 7 | Execute generate_deliverables | Final verification checkpoint (≥99% claims) |

### Key Characteristics

- **Assistant Role**: Validates and checks, does not execute tools
- **Sub-Skill Reader**: Reads sub-skill files for rules and thresholds
- **Gate Enforcer**: Checks confidence/completeness thresholds
- **Claim Registrar**: Registers claims in inv_context.json
- **Recovery Advisor**: Returns recovery options when gates block

---

## Sub-Skills (CAPPY's Knowledge Base)

CAPPY reads these sub-skill files for rules and guidance:

| Sub-Skill | Purpose | When Used |
|-----------|---------|-----------|
| `curator.md` | Claim registration rules | Phase 2-3 validation |
| `gate.md` | Phase gate thresholds | All phase validations |
| `sherlock.md` | Hypothesis coherence rules | Phase 4 validation |
| `recon.md` | Environment validation | Phase 4 validation |
| `synthesis.md` | Narrative patterns | Phase 4 guidance |
| `validate.md` | Solution validation rules | Phase 5 validation |
| `escalation.md` | Escalation decision trees | When gates block |
| `initialize.md` | Phase 0 setup rules | Phase 0-1 |
| `logging.md` | Forensics logging | All phases |

**Location**: `mcp-cappy-prod/skills/investigate/sub-skills/`

---

## Phase Gate Thresholds

| Phase | Gate | Threshold |
|-------|------|-----------|
| 2 | Confidence | ≥ 90% |
| 3 | Completeness | ≥ 99% |
| 4 | Coherence | ≥ 90% |
| 5 | Validation | ≥ 99% |
| 7 | Verification | ≥ 99% confidence + ≥ 99% claims verified |

---

## CAPPY Response Format

```json
{
  "phase": 2,
  "gate_status": "PASSED|BLOCKED",
  "score": 92,
  "threshold": 90,
  "claims_registered": 5,
  "recommendation": "Proceed to Phase 3 evidence analysis",
  "recovery_options": []
}
```

When blocked:
```json
{
  "phase": 2,
  "gate_status": "BLOCKED",
  "score": 82,
  "threshold": 90,
  "reason": "Confidence 82% < 90% threshold",
  "recovery_options": [
    { "rank": 1, "option": "Gather more evidence", "expected_improvement": "+10-20%" },
    { "rank": 2, "option": "Refine triage keywords", "expected_improvement": "+5-15%" },
    { "rank": 3, "option": "Lower threshold (risky)", "expected_improvement": "None" }
  ]
}
```

---

## Invocation Example

```
Main Claude:
  ANNOUNCE: "Phase 2 triage complete. Invoking CAPPY for validation..."

  Task({
    subagent_type: "CAPPY",
    prompt: "Validate Phase 2 triage results. Confidence: 92%. Patterns found: 5.",
    inputs: {
      phase: 2,
      confidence_score: 92,
      patterns_found: 5,
      inv_context_path: "/case/SF-12345678/inv_context.json"
    }
  })

CAPPY:
  1. Read gate.md → Threshold is 90%
  2. Check: 92% >= 90%? → PASSED
  3. Read curator.md → Register claims
  4. Update inv_context.json
  5. Return: { gate_status: "PASSED", recommendation: "Proceed to Phase 3" }

Main Claude:
  CHECKPOINT: "Gate passed. Proceed to Phase 3?"
  User: "Yes"
  CONTINUE: Phase 3
```

---

## CAPPY Limits

**Cannot:**
- Execute MCP tools (Main Claude does this)
- Make phase transition decisions (Main Claude decides)
- Interact with user (Main Claude does this)
- Read files outside case directory

**Can:**
- Read sub-skill files for rules
- Validate findings against rules
- Check gate thresholds
- Register claims in inv_context.json
- Return recommendations and recovery options

---

## Escalation

CAPPY reads `escalation.md` when:
- Gate blocked 2x with no progress
- Investigation deadlock (3+ equally likely hypotheses)
- Context budget < 30% remaining
- Case severity SEV-1/2

Returns:
```json
{
  "escalation_needed": true,
  "reason": "Investigation deadlock",
  "recommendation": "Escalate to TAC expert"
}
```

---

**Version**: 2.1.0
**Last Updated**: 2026-02-13
**Architecture**: Main Claude Primary, CAPPY Assistant
