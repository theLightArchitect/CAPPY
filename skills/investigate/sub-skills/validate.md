# /validate Skill

**Version**: 1.0.0
**Component**: Post-Phase 3 Citation Validation
**Purpose**: Knowledge holder for CAPPY_GURU_AGENT
**Created**: 2026-02-05

---

## Overview

This skill contains complete citation validation specifications and contradiction detection rules. CAPPY_GURU_AGENT reads this skill to intelligently validate that all Phase 3 evidence extraction claims are properly cited.

**When Invoked**: By CAPPY_GURU_AGENT after Phase 3 evidence extraction
**Output**: PASSED (all claims cited) or BLOCKED (claims uncited, with recovery options)

---

## Hook Integration (CRITICAL)

### ClaimValidatorHook (Step 3 of ICFP v2.0)

**When to Trigger**: After agent validates citation format and content

**Hook Specification**:
```rust
ClaimValidatorHook::trigger(
  claims: Vec<Claim>,        // Claims extracted by agent from Phase 3
  pass_count: usize,         // Number of claims that passed validation
  fail_count: usize,         // Number of claims that failed validation
  verification_rate: f32,    // pass_count / (pass_count + fail_count)
  failed_claims: Vec<String> // Citations that failed validation
)
```

**Agent Implementation**:
1. Validate each citation format (7 formats: file:line, HAR:entry, timestamp, etc.)
2. Verify citation points to real evidence file
3. For failed citations, collect reason (file not found, entry doesn't exist, timestamp invalid)
4. Calculate verification_rate = valid_citations / total_citations
5. Trigger hook with results

**Hook Response Handling**:
```
Hook returns: { passed: true/false, confidence_delta: float, recommendations: [] }

If passed:
  → Return "PASSED: All claims validated" to agent

If failed:
  → Collect failed_claims from hook response
  → Return "BLOCKED: [N] claims failed validation" with details
  → Agent provides recovery options to Claude
```

**Why This Hook**: Guards against uncited claims entering Phase 4. Every claim must be traceable to evidence.

---

## Core Principle

**All factual claims must be verifiable.**

Before advancing to Phase 4, Main Claude's Phase 3 findings must pass this validation:
- ✅ Every claim has a citation
- ✅ Citation format is valid
- ✅ Citation points to real evidence
- ✅ No contradictions in timeline/events
- ✅ Verification rate ≥ 90%

---

## 7 Citation Formats (Validation Rules)

### Format 1: File:Line
**Pattern**: `filename:123` or `path/to/file:45-67`

**Examples**:
```
server.log:4521
access.log:156-178
config.yaml:23
```

**Validation**:
```
1. File exists? ✓
2. Is numeric line reference? ✓
3. Line count sufficient? (file has 4521 lines, citing line 4521) ✓
4. Multiple lines? (156-178) Extract range ✓
```

**Agent Check**:
```bash
# Verify file exists
test -f server.log

# Verify line count
wc -l server.log  # returns 4521

# Verify specific line content (optional)
sed -n '4521p' server.log | grep "rate limit"
```

---

### Format 2: HAR:Entry
**Pattern**: `file.har:entry-123` or `file.har:entries-123,456,789`

**Examples**:
```
capture.har:entry-145
trace.har:entries-145,234,567
api.har:entry-1
```

**Validation**:
```
1. File is HAR? ✓
2. Entry number valid? ✓
3. Entry exists in JSON? ✓
4. Entry has HTTP content? ✓
```

**Agent Check**:
```bash
# Verify HAR is valid JSON
jq . file.har > /dev/null

# Verify entry exists
jq '.log.entries[144]' file.har

# Verify it's HTTP entry (has status code)
jq '.log.entries[144].response.status' file.har
```

---

### Format 3: Timestamp
**Pattern**: `timestamp:2026-01-15T10:30:45Z` or `timestamp:2026-01-15T10:30:45Z±5s`

**Examples**:
```
timestamp:2026-01-15T10:30:45Z
timestamp:2026-01-15T10:30:45Z±5s
time:10:30:45
```

**Validation**:
```
1. ISO-8601 format? ✓
2. Reasonable date? (not 1970, not future) ✓
3. Supported by evidence? (found in HAR or logs) ✓
4. Timeline consistent with other events? ✓
```

**Agent Check**:
```bash
# Verify ISO-8601 format
echo "2026-01-15T10:30:45Z" | grep -E '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z'

# Verify exists in evidence
grep "10:30:45" server.log
```

---

### Format 4: Pattern ID
**Pattern**: `pattern:P-127` or `pattern_database:P-127`

**Examples**:
```
pattern:P-127
P-127 (rate limiting)
Pattern P-127 from database
```

**Validation**:
```
1. Pattern ID format valid? (P-###) ✓
2. Pattern exists in database? ✓
3. Pattern matches symptom? ✓
4. Confidence level recorded? ✓
```

**Agent Check**:
```bash
# Verify pattern format
echo "P-127" | grep -E '^P-[0-9]+$'

# Verify in pattern database
jq '.patterns[] | select(.id == "P-127")' cappy-cache_latest.json
```

---

### Format 5: JIRA Reference
**Pattern**: `JIRA:SF-12345678` or `ticket:XSUP-99999`

**Examples**:
```
JIRA:SF-DEMO-002
ticket:XSUP-54321
issue:CAC-12345
```

**Validation**:
```
1. JIRA ID format valid? ✓
2. JIRA issue exists? ✓
3. Issue relates to symptom? ✓
4. Has relevant comments/attachments? ✓
```

**Agent Check**:
```bash
# Verify format
echo "SF-DEMO-002" | grep -E '^[A-Z]+-[0-9]+$'

# Verify in JIRA (via call_tool)
# call_tool("jira_get_issue", { issue_key: "SF-DEMO-002" })
```

---

### Format 6: Configuration File Reference
**Pattern**: `config:section.key` or `env_log:product`

**Examples**:
```
config:integration.timeout_ms
env_log:product.version
about_info:build_number
```

**Validation**:
```
1. Config file exists? ✓
2. Section/key path valid? ✓
3. Value supports claim? ✓
4. Format matches expected? (number, string, etc.) ✓
```

**Agent Check**:
```bash
# Verify config file exists
test -f config.json

# Verify key path
jq '.integration.timeout_ms' config.json

# Check if value supports claim
jq '.integration.timeout_ms' config.json | grep "5000"
```

---

### Format 7: Environment Metadata
**Pattern**: `env:product`, `env:version`, `env:hostname`

**Examples**:
```
env:product=XSOAR
env:version=8.9.0-2464525
env:hostname=xsoar.example.com
```

**Validation**:
```
1. Environment detected correctly? ✓
2. Metadata from env.log? ✓
3. Consistent throughout investigation? ✓
4. Confirmed by user? ✓
```

**Agent Check**:
```bash
# Verify in env.log
grep "product.*XSOAR" env.log
grep "version.*8.9.0" env.log

# Verify in inv_context.json
jq '.environment.product' inv_context.json
```

---

## Contradiction Detection Rules

### Rule 1: Temporal Contradictions
**Check**: Do claims about same event have conflicting timestamps?

**Example**:
```
Claim A: "Timeout occurred at 10:30:45Z"
Claim B: "Error message appeared at 10:30:42Z"

Issue: Timeout is 3 seconds AFTER error, but claim says timeout "occurred"
       (implies it was first event)

Resolution: "Error appeared first (10:30:42Z), then timeout after 3 seconds"
```

**Agent Logic**:
```
1. Extract all timestamps from claims
2. For same event: check if timestamps differ >10 seconds
3. If yes: mark as contradiction
4. Suggest clarification to Main Claude
```

---

### Rule 2: Causal Contradictions
**Check**: Do claims have impossible cause-effect relationships?

**Example**:
```
Claim A: "HTTP 429 caused rate limiting"
Claim B: "Integration sent 50 requests/minute (below rate limit)"

Issue: Can't get HTTP 429 if below rate limit

Resolution: "Actually sent 150 requests/minute (exceeds limit)"
            OR "HTTP 429 was for different endpoint"
```

**Agent Logic**:
```
1. Extract causal statements ("caused", "due to", "because of")
2. Verify cause-effect makes logical sense
3. Cross-check with evidence
4. If impossible: mark contradiction
```

---

### Rule 3: Factual Contradictions
**Check**: Do claims state conflicting facts about same thing?

**Example**:
```
Claim A: "XSOAR version is 8.9.0"
Claim B: "XSOAR version is 8.8.0"

Issue: Clear contradiction

Resolution: Verify actual version in env.log
```

**Agent Logic**:
```
1. Group claims by topic (version, hostname, etc.)
2. Check if multiple claims state different values
3. If yes: contradiction
4. Resolve from env.log or ask Claude
```

---

### Rule 4: Evidence Contradictions
**Check**: Does evidence support contradictory claims?

**Example**:
```
Claim A: "No HTTP 429 errors found"
Evidence: HAR entries 145, 234, 567 contain HTTP 429

Issue: Claim contradicts evidence

Resolution: "Multiple HTTP 429 errors found in HAR"
```

**Agent Logic**:
```
1. For each claim, verify against evidence
2. If evidence contradicts: mark
3. This is RED FLAG - claim is wrong
4. Require Claude to fix claim
```

---

## Validation Workflow

### Step 1: Extract Claims from Phase 3 Output
Agent reads Main Claude's Phase 3 analysis and identifies all factual claims.

**Example From Claude**:
```
"Phase 3 analysis complete.
I found HTTP 429 errors in HAR entries 145, 234, 567 - indicates rate limiting.
server.log line 4521 shows 'Rate limit exceeded' message.
Timeline analysis: all errors occurred within 60-second window at 2026-01-15T10:30:45Z.
Integration was sending ~150 requests/minute, exceeding expected API limit."
```

**Extracted Claims**:
1. "HTTP 429 errors found" → Citation: `file.har:entries-145,234,567`
2. "Rate limit exceeded message found" → Citation: `server.log:4521`
3. "All errors at specific time" → Citation: `timestamp:2026-01-15T10:30:45Z`
4. "Integration sent ~150 req/min" → Citation: `config:integration.polling_rate` + evidence

---

### Step 2: Validate Each Claim

**For Each Claim**:
1. Is there a citation? (YES/NO)
2. Is citation format valid? (YES/NO)
3. Does citation exist in evidence? (YES/NO)
4. Does evidence support claim? (YES/NO)

**Results**:
```json
{
  "claim": "HTTP 429 errors found",
  "citation": "file.har:entries-145,234,567",
  "citation_format": "HAR:Entry",
  "citation_exists": true,
  "evidence_supports": true,
  "validated": true
}
```

---

### Step 3: Check for Contradictions

**For Each Pair of Claims**:
- Temporal: Same event, different times? (check)
- Causal: Cause → effect relationships valid? (check)
- Factual: Same fact, different values? (check)
- Evidence: Evidence supports both claims? (check)

---

### Step 4: Calculate Verification Rate

```
verification_rate = (verified_claims) / (total_claims)

Example:
  Total claims: 7
  Verified: 6
  Uncited: 1
  Verification rate: 6/7 = 0.857 = 85.7%
```

**Threshold**: ≥ 90% required to PASS

---

### Step 5: Decision

**IF verification_rate ≥ 90% AND no contradictions**:
```json
{
  "status": "PASSED",
  "verification_rate": 0.93,
  "contradictions": 0,
  "uncited_claims": 0,
  "message": "All Phase 3 claims validated. Ready for Phase 4."
}
```

**IF verification_rate < 90% OR contradictions found**:
```json
{
  "status": "BLOCKED",
  "verification_rate": 0.85,
  "contradictions": 1,
  "uncited_claims": 1,
  "uncited_claims_details": [
    "API rate limit is 100 req/min (assumption, not verified)"
  ],
  "contradictions_details": [
    "Timeout at 10:30:45Z contradicts error at 10:30:42Z (timing inconsistency)"
  ],
  "recovery_options": [
    "Search logs for HTTP 429 to verify rate limit value",
    "Clarify timeline: which event occurred first?",
    "Request API docs from customer confirming rate limit"
  ]
}
```

---

## Agent Orchestration Model

### When CAPPY_GURU_AGENT Reads This Skill

The agent will:

1. **Extract Claims**: Parse Claude's Phase 3 output
2. **Validate Citations**: Check 7 formats against evidence
3. **Detect Contradictions**: Check 4 contradiction rules
4. **Calculate Rate**: (verified / total) × 100
5. **Make Decision**: PASSED (≥90%) or BLOCKED (<90%)
6. **Return Feedback**:
   - If PASSED: "Ready for Phase 4"
   - If BLOCKED: "Fix these items: [list], Try: [recovery options]"

---

## Recovery Options (For Main Claude When BLOCKED)

### If Uncited Claims
**Option 1**: Search evidence for missing citation
```
Claim: "API rate limit is 100 req/min"
Search: grep -r "100.*req\|rate.*limit" evidence/
Result: Found in docs.json line 234
```

**Option 2**: Request missing evidence from customer
```
"I need the API documentation to confirm rate limit.
Currently assuming 100 req/min from error pattern."
```

**Option 3**: Mark as "unverified observation"
```
"Note: Unverified assumption - API limit assumed to be 100 req/min
       based on error pattern, but not confirmed in customer docs."
```

### If Contradictions
**Option 1**: Verify timeline manually
```
Extract all timestamps:
  HAR entry 145: 2026-01-15T10:30:45.123Z
  server.log line 4521: 2026-01-15T10:30:45.456Z
Check: Same second, different milliseconds → NOT a contradiction
```

**Option 2**: Clarify causality
```
"Timeout occurred BECAUSE of rate limiting"
vs
"Rate limiting caused the integration timeout"
→ Both mean same thing, rephrase for clarity
```

**Option 3**: Re-examine evidence
```
"Re-read HAR entries 145-150 to understand sequence of HTTP errors"
→ Rebuild timeline with precise order
```

---

## Success Criteria

Phase 3 claims are VALIDATED when:

✅ Every claim has a citation
✅ Every citation uses valid format (one of 7)
✅ Every citation exists in evidence
✅ Evidence supports each claim
✅ No temporal contradictions
✅ No causal contradictions
✅ No factual contradictions
✅ Verification rate ≥ 90%
✅ Ready to advance to Phase 4

---

## Related Components

- **CAPPY_GURU_AGENT**: Reads this skill, validates claims, blocks if needed
- **inv_context.json v3.0**: Updated with verification status and uncited claims
- **/investigate SKILL.md**: Documents when CAPPY_GURU_AGENT is invoked
- **7 Citation Formats**: Defined here, validated by agent

---

**Skill Version**: 1.0.0
**Last Updated**: 2026-02-05
**Status**: Ready for CAPPY_GURU_AGENT implementation
