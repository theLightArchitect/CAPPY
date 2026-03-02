# /investigate Skill - Quick Reference Card (v8.0.0-v1.7)

**Use this when you need fast answers during investigation**

---

## Hook Status Decision Tree

After EVERY tool call, check the result JSON in this order:

```
Step 1: Error field present?
  YES → Tool was BLOCKED by pre-hook
        Display: error message + remediation hint
        Action: Fix input, retry

Step 2: "requires_review": true?
  YES → Tool execution PREVENTED
        Display: review_prompt + threshold_violated
        Action: Ask user what to do

Step 3: "verifications" field present?
  YES → Tool succeeded BUT post-checks FAILED
        Display: Each failed verification
        Severity: Error (blocks) vs Warning (caution)
        Action: Interpret, fix, or proceed with caution

Step 4: None of above?
  → Tool ran successfully, no issues
  → Use output, proceed normally
```

---

## Phase Gates Quick Check

### Phase 5 Gate (Before: validate_solution)

```
Confidence >= 99%?

YES  → Proceed to validate_solution
NO   → BLOCKED

Options if blocked:
  A) Gather more evidence → Re-run Phase 3
  B) Improve hypothesis → Re-run Phase 4
  C) Lower threshold (risky) → Override gate
```

### Phase 7 Gate (Before: generate_deliverables)

```
Confidence >= 99%?
All critical claims Verified?

YES to both → Generate deliverables
NO to either → BLOCKED

Options if blocked:
  A) Gather more evidence → Re-run Phase 3-4
  B) Resolve claims → Fix pending/failed claims
  C) Escalate to expert → Get outside help
```

---

## Troubleshooting Fast Lookup

| Error | Quick Fix |
|-------|-----------|
| "Symptom required" | Ask user for symptom → Retry |
| "Confidence 88% < 99%" | Gather more evidence → Re-run Phase 3 |
| "Uncited claims" | Read curator.md for citation fix guidance |
| "File not found" | Check bundle extraction → Re-extract |
| "Claims Pending" | Resolve claims → Retry delivery |
| "No CITATIONS section" | Add `---\nCITATIONS\n[1]...` to deliverable bottom |
| "Character limit exceeded" | Condense text, keep citations, target < 2900 chars |

**Full guide**: See SKILL.md lines 972-1070

---

## Manual Verification Quick Commands

### After triage_case:
```bash
cd {case_directory}/analysis
jq '.confidence_score' patterns.json
# Expected: >= 90% (Phase 2 minimum)
```

### After analyze_evidence:
```bash
cd {case_directory}/analysis
jq '.files_analyzed' errors.json
# Expected: >= 3 files (good coverage)
```

### After generate_deliverables:
```bash
cd {case_directory}/deliverables
ls -lh customer_response.txt JIRA_DRAFT.txt
# Expected: All files exist, > 0 bytes
```

---

## Gate Thresholds by Phase

| Phase | Gate Threshold | What It Checks |
|-------|---|---|
| 2 | >= 90% | Triage confidence (minimum) |
| 3 | >= 99% | Evidence completeness |
| 4 | >= 90% | Hypothesis coherence |
| 5 | >= 99% | Can proceed to validation |
| 7 | >= 99% | Can deliver to customer |
| Delivery | All Verified | All critical claims resolved |

---

## Checkpoint Questions to Ask User

### After Phase 2 (Triage)
```
Found {N} patterns, confidence {X}%

Gate check (Phase 5 requires 99%): {X}%
  If X < 99: "Confidence is low. Still proceed to Phase 3?"
  If X >= 99: "Ready for Phase 3 evidence analysis?"
```

### After Phase 3 (Evidence)
```
Extracted {N} errors, {M} timeline events

Coverage: {X}% of files analyzed
  If X < 99%: "Only {X}% coverage. Request more logs?"
  If X >= 99%: "Ready for Phase 4 hypothesis?"
```

### After Phase 4 (Hypothesis)
```
Hypothesis: "{hypothesis}"
Confidence: {X}%

Gate check (Phase 5 requires 99%): {X}%
  If X < 99: CANNOT proceed. Must gather more evidence.
  If X >= 99: "Ready for Phase 5 validation?"
```

### Before Phase 7 (Deliverables)
```
Final confidence: {X}%

Gate check (Phase 7 requires 99%): {X}%
  If X < 99: CANNOT deliver. Gate BLOCKS.
  If X >= 99: "Ready to deliver to customer?"
```

---

## When to Use PTC (Advanced)

### Phase 1 Parallel Search (FASTEST)
```json
call_tool(workflow, "phase1-parallel", {...})
// Pattern + Case + JIRA in ~30s (instead of 90s)
```

### Timeline Correlation (COMPLEX ISSUES)
```json
call_tool(workflow, "timeline-correlation", {...})
// Cross-reference logs + HAR timestamps
```

### Custom Logic (SPECIAL CASES)
```json
call_tool(workflow, "custom", {code: "JavaScript here"})
// For investigations that don't fit standard templates
```

---

## Investigation Status Summary

| Phase | Output | Gate | Next |
|-------|--------|------|------|
| 0-1 | inv_context.json | None | Phase 2 |
| 2 | patterns.json | >= 90% | Phase 3 |
| 3 | errors.json + timeline.json | >= 99% coverage | Phase 4 |
| 4 | hypothesis.json + validation.json | >= 90% coherence | Phase 5 |
| 5 | validation results | >= 99% confidence | Phase 7 |
| 7 | customer_response.txt, JIRA_DRAFT.txt | >= 99%, all claims verified | DONE |

---

## Deliverable Citation Requirements (NON-NEGOTIABLE)

**ALL deliverables MUST include CITATIONS section at bottom.**

### Quick Format
```
Inline: Use [1], [2], [1][2] in body text

---
CITATIONS
[1] HAR:GET /endpoint - description
[2] Screenshot: "filename.png" - what it shows
[3] Log:file.log:line_N - specific entry
```

### Citation Types
| Source | Format |
|--------|--------|
| API call | `HAR:GET/POST/DELETE /endpoint - HTTP status, data` |
| Screenshot | `Screenshot: "filename.png"` |
| Log entry | `Log:filename.log:line_N` |
| Config | `Bundle:env.log - key=value` |
| Pattern | `Pattern:P-PRODUCT-NNN` |

### Before Finalizing Deliverable
```bash
# Check citations section exists
grep -A 20 "^CITATIONS" deliverables/customer_response.txt

# Count inline references
grep -oE '\[[0-9]+\]' deliverables/customer_response.txt | sort -u | wc -l
```

### Character Limit Check
```bash
# Must be < 2900 chars (AFTER citations)
wc -c deliverables/customer_response.txt
```

---

## Key Files to Check During Investigation

```
{case_directory}/
├── inv_context.json          # Current state, confidence score
├── analysis/
│   ├── patterns.json         # Phase 2 results
│   ├── errors.json           # Phase 3 results
│   ├── timeline.json         # Phase 3 timeline
│   ├── hypothesis.json       # Phase 4 results
│   └── validation.json       # Phase 5 results
└── deliverables/
    ├── customer_response.txt # Phase 7 output
    ├── JIRA_DRAFT.txt        # Phase 7 output
    └── investigation_summary.html # Phase 7 output
```

**To check current phase**:
```bash
jq '.current_phase' {case_directory}/inv_context.json
```

**To check current confidence**:
```bash
jq '.confidence_score' {case_directory}/inv_context.json
```

---

## When to Escalate

1. **Investigation stuck** despite troubleshooting → Read escalation.md, present options
2. **Contradictory evidence** in Phase 4 → Read sherlock.md for coherence rules
3. **Multiple root causes** found → Read recon.md for environment validation
4. **Confidence blocked** (< 99% despite evidence) → Escalate to TAC expert
5. **Custom automation** needed → Use PTC custom template

---

## Common Gotchas

| Gotcha | Why | Fix |
|--------|-----|-----|
| Tool runs but says "verify" | Post-hook checking output | Read verification results, decide if OK to use |
| Phase 5 blocks with "confidence 95%" | Phase 5 gate is >= 99%, not >= 90% | Gather more evidence, re-run Phase 3-4 |
| Delivery blocks with "claim unverified" | Delivery gate requires ALL claims verified | Update claim status to Verified |
| Files missing in extracted/ | Bundle extraction failed silently | Check bundles, re-extract, verify permissions |
| Gates seem random | Different gates for different phases | See gate thresholds table above |

---

## Skill Sections by Use Case

| Use Case | Section | Lines |
|----------|---------|-------|
| Understand hook status | "How to Read Hook Status" | 59-237 |
| Verify tool output | "Manual Verification After Tool Execution" | 730-796 |
| Speed up analysis | "Programmatic Tool Calling (PTC)" | 797-892 |
| Deal with blocks | "Troubleshooting Common Hook Failures" | 972-1070 |
| Check phase gates | "Checkpoints & Decision Gates" | 929-972 |
| Understand directory | "Case Directory Structure" | 894-927 |
| Track investigation | "inv_context.json Schema" | 1068-1098 |

---

## Version & Compatibility

- **Version**: 8.1.0-v1.7
- **Compatible with**: CAPPY v1.7.0 (single call_tool entry point)
- **Last Updated**: 2026-02-14
- **Status**: Production Ready
- **Changes**: Added Deliverable Citation Requirements mandate

For full documentation, see `SKILL.md`

---

*Quick Reference Card v8.0.0-v1.7*
*Use SKILL.md for detailed explanations*
