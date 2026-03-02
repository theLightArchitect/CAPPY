# /curator Skill

**Version**: 1.0.0
**Component**: ICFP v2.0 Protocol Management
**Purpose**: Knowledge holder for CAPPY_CURATOR_AGENT
**Created**: 2026-02-05

---

## Overview

This skill contains complete ICFP v2.0 (Investigation Claim Flow) protocol specifications. CAPPY_CURATOR_AGENT reads this skill to intelligently manage the 8-step claim registration and verification process.

**When Invoked**: By CAPPY_CURATOR_AGENT after each phase (2-7)
**Output**: Claims registered in inv_context.json with verification status and audit trail

---

## ICFP v2.0 Protocol: 8-Step Claim Chain

### Step 1: Claim Capture Hook
**Purpose**: Extract all claimed facts from phase output with citations

**Agent Decision**:
- Parse Claude's phase output
- Identify claim statements
- Extract citation references (file:line, HAR:entry, timestamp)
- Format as structured claims

**Example Input from Claude**:
```
"Phase 3 analysis complete.
Found HTTP 429 errors in HAR entries 145, 234, 567.
Rate limit message in server.log line 4521.
Timeline: all events at 2026-01-15T10:30:45Z ±5 seconds."
```

**Extracted Claims**:
```json
[
  {
    "claim": "HTTP 429 errors found",
    "citation": "file.har:entry-145,234,567",
    "source": "HAR",
    "confidence": 1.0
  },
  {
    "claim": "Rate limit exceeded",
    "citation": "server.log:4521",
    "source": "Log file",
    "confidence": 0.95
  },
  {
    "claim": "Timeline correlation",
    "citation": "timestamp:2026-01-15T10:30:45Z",
    "source": "Timeline",
    "confidence": 0.90
  }
]
```

---

### Step 2: Claim Verification Hook
**Purpose**: Verify extracted claims against tool output

**Agent Decision**:
- For each claim, verify it matches source data
- Check citation exists in actual file
- Compare claim text with source content
- Mark as verified or unverified

**Verification Logic**:
```
Claim: "HTTP 429 errors found"
Citation: "file.har:entry-145"
Check: Does HAR entry 145 contain HTTP 429?
Result: YES → claim.verified = true
```

**Output**:
```json
{
  "verified_claims": 3,
  "unverified_claims": 0,
  "verification_rate": 1.0
}
```

---

### Step 3: Claim Validator Hook
**Purpose**: 3-pass validation (syntax, semantic, evidence)

**Pass 1 - Syntax**:
- Claim is well-formed sentence
- Citation format is valid (file:line or file:entry or timestamp)
- No typos or obvious errors

**Pass 2 - Semantic**:
- Claim is logically coherent
- No contradictions with other claims
- Makes sense in investigation context

**Pass 3 - Evidence**:
- Citation points to real evidence file
- Evidence supports the claim
- Confidence level appropriate

**Output**:
```json
{
  "pass_1_syntax": true,
  "pass_2_semantic": true,
  "pass_3_evidence": true,
  "validation_status": "PASS",
  "confidence": 0.95
}
```

---

### Step 4: Identifier Extraction Hook
**Purpose**: Extract pattern IDs, JIRA IDs, and other identifiers

**Patterns to Extract**:
- Pattern IDs (P-127, P-456)
- JIRA issue IDs (SF-12345678, XSUP-99999)
- Error codes (E001, ERR_TIMEOUT)
- Version/build info

**Agent Decision**:
- Scan all claims for identifier mentions
- Cross-reference with pattern database
- Link to JIRA if mentioned
- Store in inv_context.json.identifiers

**Example**:
```
Claim: "Pattern P-127 (rate limiting) matches with 85% confidence"
Extracted:
  - pattern_id: "P-127"
  - confidence: 0.85
  - reference: "JIRA XSUP-12345"
```

---

### Step 5: Forensic File Parser Hook
**Purpose**: Verify evidence file paths and line numbers exist

**For Each Citation**:
1. Verify file exists: `file -e path/to/file`
2. If file:line → verify line count sufficient
3. If HAR:entry → verify entry exists in JSON
4. If timestamp → verify format is ISO-8601

**Output**:
```json
{
  "citations_verified": 3,
  "citations_failed": 0,
  "files_checked": ["server.log", "file.har"],
  "forensic_status": "ALL_VERIFIED"
}
```

**Error Handling**:
If citation fails:
```json
{
  "failed_citation": "server.log:9999",
  "reason": "Line 9999 doesn't exist (file has 4521 lines)",
  "recommendation": "Verify line number or search logs for claim"
}
```

---

### Step 6: Research Queue Hook
**Purpose**: Queue unverified claims for future research

**When Claim is Unverified**:
- Add to research queue in inv_context.json
- Mark priority (HIGH, MEDIUM, LOW)
- Suggest research direction
- Set deadline for verification

**Example Queue Entry**:
```json
{
  "claim": "API rate limit is 100 req/min",
  "reason": "NOT_VERIFIED - assumption from error pattern",
  "priority": "HIGH",
  "research_needed": "Search Cortex docs for XSOAR rate limit specs",
  "phase": 4,
  "queued_at": "2026-02-05T14:45:00Z",
  "deadline": "2026-02-05T15:00:00Z"
}
```

---

### Step 7: Timeline Correlation Hook
**Purpose**: Merge timelines from multiple sources

**Sources**:
- HAR: Timestamps from HTTP requests/responses
- Logs: Timestamps from application logs
- Manual: Timestamps from user observations

**Agent Decision**:
- Extract all timestamps from evidence
- Merge into unified timeline
- Identify correlation points (same time ±5 seconds)
- Build event sequence

**Output**:
```json
{
  "timeline_events": [
    {
      "timestamp": "2026-01-15T10:30:45Z",
      "source": "HAR",
      "event": "HTTP 429 response",
      "entry": "entry-145"
    },
    {
      "timestamp": "2026-01-15T10:30:45Z",
      "source": "server.log",
      "event": "Rate limit exceeded",
      "line": 4521
    }
  ],
  "correlation_points": 2,
  "timeline_merged": true
}
```

---

### Step 8: Evidence Chain Hook
**Purpose**: Create complete audit trail of all claims

**Audit Trail Entries**:
```json
{
  "claim_id": "claim-001",
  "claim": "HTTP 429 errors found",
  "citation": "file.har:entry-145,234,567",
  "phase": 3,
  "extracted_by": "Main Claude",
  "verified_at": "2026-02-05T14:45:15Z",
  "verification_status": "VERIFIED",
  "hook_chain": [
    {"hook": "ClaimCapture", "status": "PASS"},
    {"hook": "ClaimVerification", "status": "PASS"},
    {"hook": "ClaimValidator", "status": "PASS"},
    {"hook": "IdentifierExtraction", "status": "PASS"},
    {"hook": "ForensicFileParser", "status": "PASS"},
    {"hook": "TimelineCorrelation", "status": "PASS"},
    {"hook": "EvidenceChain", "status": "PASS"}
  ],
  "confidence": 0.98,
  "audit_trail": [
    "2026-02-05T14:45:00Z - Claim captured from Phase 3 output",
    "2026-02-05T14:45:05Z - Verified against HAR entry 145",
    "2026-02-05T14:45:10Z - Citation verified (entry exists)",
    "2026-02-05T14:45:15Z - Added to evidence chain"
  ]
}
```

---

## Agent Orchestration Model

### When CAPPY_CURATOR_AGENT Reads This Skill

The agent will:

1. **Understand Protocol**: "8-step ICFP chain required"
2. **Decide Execution**: "Execute steps 1-8 in order"
3. **Call Hooks Intelligently**:
   - Step 1: Extract claims from Claude's output
   - Step 2: Verify against tool output
   - Step 3: Validate with 3-pass system
   - Step 4: Extract identifiers (patterns, JIRA)
   - Step 5: Verify file paths/line numbers
   - Step 6: Queue unverified for research
   - Step 7: Merge timelines
   - Step 8: Create audit trail
4. **Interpret Results**: "Claims registered, 92% verified"
5. **Return Feedback**: "Status: CURATED. Claims: 7. Verification: 92%. Queued: 1."

---

## Phase-Specific Hook Variations

### Phase 2 (Triage)
**Hooks Used**: 1, 2, 3, 4, 8 (skip 5, 6, 7 - no file:line citations yet)

Pattern: "P-127 (rate limiting) with 85% confidence"
Claim: "Pattern P-127 matches"
Citation: "pattern_database:P-127"
Verification: Extract pattern ID, link to JIRA

### Phase 3 (Evidence)
**Hooks Used**: 1, 2, 3, 4, 5, 6, 7, 8 (all hooks)

Claims: "HTTP 429 in HAR entry 145"
Citations: "file.har:entry-145", "server.log:4521"
Verification: Verify files exist, lines exist, timeline correlates

### Phase 4 (Hypothesis)
**Hooks Used**: 1, 2, 3, 4, 8

Hypothesis: "API rate limit exceeded"
Assumptions: "API limit is 100 req/min (assumed)"
Citation: "pattern_database:P-127" + "customer_config (assumed)"
Verification: Mark assumptions vs verified facts

---

## Error Handling Strategies

### What If Citation Doesn't Exist?

**Agent Decision**:
```
Claim: "HTTP 429 error at line 5000"
Citation: "server.log:5000"
Check: Does server.log have line 5000?
Result: NO - file only has 4521 lines

Action:
  1. Mark claim as "citation_missing"
  2. Queue for research: "Search logs for HTTP 429 error"
  3. Continue with other claims
  4. Report to Main Claude: "1 claim uncited, 6 verified"
```

### What If Claim Contradicts Another?

**Agent Decision**:
```
Claim 1: "Timeout occurred at 10:30:45Z"
Claim 2: "Error occurred at 10:30:42Z"
Check: Contradiction? YES - 3 seconds apart but called "timeout"

Action:
  1. Mark as "contradiction_detected"
  2. Queue for Claude: "Verify timeline inconsistency"
  3. Report: "Contradiction: timeout vs error timing"
```

### What If Hook Fails?

**Agent Decision**:
```
Hook: ClaimValidator (Pass 3 - Evidence)
Error: "Citation file not found"

Action:
  1. Log error
  2. Mark claim as "validation_failed"
  3. Continue (don't block on single claim)
  4. Report to Main Claude: "2 claims failed validation"
```

---

## inv_context.json Integration

The agent updates inv_context.json with:

```json
{
  "phases": {
    "phase_3": {
      "status": "COMPLETE",
      "claims_extracted": 7,
      "claims_verified": 6,
      "claims_queued": 1,
      "verification_rate": 0.857,
      "hook_chain_complete": true,
      "timeline_merged": true,
      "audit_chain_complete": true,
      "curated_at": "2026-02-05T14:45:30Z"
    }
  },

  "claims": [
    {
      "claim_id": "claim-001",
      "claim": "HTTP 429 errors found",
      "citation": "file.har:entry-145,234,567",
      "phase": 3,
      "verified": true,
      "verification_rate": 0.98,
      "hook_chain": [...]
    }
  ],

  "research_queue": [
    {
      "claim": "API rate limit is 100 req/min",
      "reason": "NOT_VERIFIED",
      "priority": "HIGH"
    }
  ],

  "timeline": [
    {
      "timestamp": "2026-01-15T10:30:45Z",
      "events": ["HTTP 429", "Rate limit exceeded"],
      "sources": ["HAR entry 145", "server.log:4521"]
    }
  ]
}
```

---

## Deliverable Citation Requirements (NON-NEGOTIABLE)

**ALL customer-facing deliverables MUST include a CITATIONS section at the bottom.**

### Citation Format for Deliverables

**Inline References**: Use `[1]`, `[2]`, `[1][2]` format in body text

**Citations Section** (MANDATORY at bottom of every deliverable):
```
---
CITATIONS
[1] HAR:GET /api/endpoint - description of evidence
[2] Screenshot: "filename.png" - what it shows
[3] Log:app-server.log:line_123 - specific entry
[4] Bundle:env.log - configuration detail
```

### What MUST Be Cited

| Claim Type | Source Format |
|------------|---------------|
| API responses | `HAR:GET/POST/DELETE /endpoint - status, data` |
| Errors | `HAR:entry_N - HTTP status, error message` |
| Timestamps | `HAR:entry_N - ISO timestamp` |
| UI observations | `Screenshot: "filename.png"` |
| Log entries | `Log:filename.log:line_N` |
| Config values | `Bundle:env.log - key=value` |
| Patterns | `Pattern:P-PRODUCT-NNN - pattern name` |

### Enforcement

- **Phase 7 BLOCKED** if deliverables lack citations section
- Main Claude MUST verify citations exist before finalizing
- Character limits apply AFTER citations added
- Each inline `[N]` must have corresponding entry in CITATIONS section

### Curator Responsibility

When curating for Phase 7:
1. Extract all claims from deliverable draft
2. Verify each claim has `[N]` inline reference
3. Verify CITATIONS section exists at bottom
4. Verify each `[N]` maps to valid evidence source
5. **FAIL** deliverable if any citation missing

---

## Success Criteria

Phase claims are CURATED when:

✅ All claims extracted with citations
✅ All claims pass 3-pass validation
✅ All file/line citations verified to exist
✅ Timeline merged from all sources
✅ Identifiers extracted and linked
✅ Unverified claims queued with research direction
✅ Complete audit trail created
✅ inv_context.json updated
✅ **Deliverables include CITATIONS section at bottom**

---

## Related Components

- **CAPPY_CURATOR_AGENT**: Reads this skill, intelligently orchestrates 8-step chain
- **inv_context.json v3.0**: Updated with claims, verification status, audit trail
- **/investigate SKILL.md**: Documents when CAPPY_CURATOR_AGENT is invoked
- **ICFP v2.0 Hooks**: 8 hooks referenced by this protocol

---

**Skill Version**: 1.1.0
**Last Updated**: 2026-02-14
**Status**: Ready for CAPPY_CURATOR_AGENT implementation
**Changes**: Added Deliverable Citation Requirements mandate (NON-NEGOTIABLE)
