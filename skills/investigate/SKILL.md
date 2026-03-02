---
name: investigate
description: TAC case investigation using 8-phase methodology with human-in-the-loop checkpoints, inline verification, and ICFP v2.0 protocol for Cortex XSOAR/XSIAM/XDR troubleshooting.
---

# /investigate - TAC Investigation Skill

**Version**: 8.5.0-v1.7
**Purpose**: Interactive TAC investigation with human-in-the-loop checkpoints, inline verification, and CAPPY v1.7 single entry point architecture.
**Architecture**: call_tool v1.7 (single MCP entry point), ICFP v2.0, PTC v1.2, verification-mandatory, Scribe v1.0, TACO Spot v1.6
**Schema**: inv_context.json v3.5.0 ([full schema](../../docs/inv_context_v3.5_schema.md))
**Template**: [inv_context_template.json](../../templates/inv_context_template.json)

**v8.5.0 Additions** (2026-02-19):
- ✅ JIRA Internal References Format: ALL JIRA_DRAFT.txt files MUST include Internal References section
- ✅ IEEE Citation Format: Standardized citation format by source type (screenshot, HAR, SF case, etc.)
- ✅ Evidence-to-Section Mapping: Required table linking JIRA sections to supporting evidence
- ✅ Audit Trail: Internal references enable traceability and QA review
- ✅ Updated: SKILL.md with JIRA Internal References Format section

**v8.4.0 Additions** (2026-02-19):
- ✅ SF Case Lookup Mandate: NON-NEGOTIABLE requirement to use case tools via call_tool
- ✅ Required Tools: `case-get`, `case-comments`, `case-search`, `case-similar`
- ✅ Quick Reference Guide: Tool names lookup table added at end of skill file
- ✅ Enforcement: DO NOT skip SF lookup, DO NOT use TACO Spot/Vision as primary source
- ✅ Updated: SKILL.md with mandate section, STEP 0, and quick reference

**v8.3.0 Additions** (2026-02-18):
- ✅ SF Case Lookup First: Phase 0 now fetches Salesforce case context BEFORE evidence processing
- ✅ Tenant About Info Auto-Extraction: Parses auto-posted comment for version, fqdn, project_id, lcaas_id
- ✅ Related Cases Discovery: Extracts linked cases from SF description for cross-case correlation
- ✅ Updated Step Order: SF lookup → Bundle extraction → Triage (provides full context for better pattern matching)

**v8.2.0 Additions** (2026-02-14):
- ✅ Deliverable Citation Requirements: ALL deliverables MUST include CITATIONS section at bottom
- ✅ Citation Format: Inline `[1]`, `[2]` references with `---\nCITATIONS` section
- ✅ Enforcement: Phase 7 BLOCKED if citations missing, character limits apply AFTER citations
- ✅ Updated: SKILL.md, QUICK_REFERENCE.md, curator.md with citation mandate

**v8.1.0 Additions** (2026-02-13):
- ✅ Schema v3.5.0: 17 sections for comprehensive investigation tracking
- ✅ Automation Triggers: Claims/evidence auto-labeled UNVERIFIED, verification sweeps
- ✅ New Sections: case, environment, symptom, evidence, hypotheses, sessions, customer_interactions, related_cases, deliverables, feedback, audit_trail
- ✅ Metrics: time_to_resolution_days, engineer_hours_saved, pattern_reuse tracking
- ✅ Template: inv_context_template.json for new investigations

**v8.0.0-v1.7 Additions** (2026-02-13):
- ✅ CAPPY v1.7 Integration: Single call_tool entry point with guaranteed hook execution
- ✅ Hook Status Interpretation: Post-tool hook checking (error, requires_review, verifications, success)
- ✅ Gate Enforcement: Phase 5 (≥99%), Phase 7 (≥99%), Delivery gate checks
- ✅ Manual Verification: jq/grep commands after each MCP tool execution
- ✅ Preserved Capabilities: All v7.10.0 features (Scribe banners, TACO Spot, Vision Direct, PTC workflows)
- ✅ No Capability Loss: Only gains from v1.7 architecture

**v7.10.0 Additions**:
- Vision Direct Case Lookup: Quick case research via Vision URL pattern
- URL format: `${CAPPY_DASHBOARD_URL}/dashboard/{case_number}`
- Workflow: Open URL → Reprocess if stale → Save as single HTML to case evidence/
- Pattern database now includes Vision URLs in case references
- Standard practice for precedent case research before deep investigation

**v7.9.0 Additions**:
- Bug B-002 Workaround: Phases 5-7 now use Main Claude directly instead of CORTEX-CAPPY Task agent
- Reason: Claude Code bug 'classifyHandoffIfNeeded is not defined' crashes custom subagents on permission issues
- CORTEX-CAPPY still used for Phases 2-4 (triage, evidence, hypothesis) where MCP tool permissions work correctly
- GitHub Issues: #22098, #22312

**v7.8.0 Additions**:
- TACO Spot Integration: case-helper, topic-researcher, jira-aid, tac-review at key phases
- Low-confidence triage → TACO PILOT guidance escape hatch
- Phase 7 JIRA enhancement with jira-aid insights
- Investigation stuck → case-helper rescue option

**v7.7.0 Additions**:
- Templates Location: Canonical path to MCP prod templates
- Combined JIRA Draft: Single `JIRA_DRAFT.txt` with plaintext + wiki markup sections
- Plain Text Customer Response: `customer_response.txt` (no markdown)
- Consolidated RCA: RCA content merged into `living_note.md` (no separate file)

**v7.6.0 Additions**:
- First-Response Templates: 5 category-specific evidence request templates (plain text)
- Evidence Requirements: triage_case returns required evidence when confidence < 70%
- Template Index: JSON mapping of symptoms → categories → required evidence

**v7.5.0 Additions**:
- WLA Priority Mapping: Maps TAC P1-P4 priorities to CAPPY SEV-1 to SEV-4 severity routing
- Agent strategy by priority: 3 parallel agents for P1, standard for P3, async for P4
- Investigation depth guidance by priority level

**v7.4.0 Additions**:
- SCRIBE INTEGRATION: Visual banners for investigation progress (START, PROGRESS, RESUME)
- Full infographic generation on case resolution (ASCII + HTML)
- JIRA templates (wiki markup + plaintext)
- Customer response templates
- Case resume detection with context restoration banner

**v7.3.0 Additions**:
- 4 new verification functions: `verifyTimestampRange`, `verifyContentPattern`, `verifyFileSize`, `verifyEncoding`
- Error Recovery Guidance: decision tree + PTC recovery patterns for verification failures
- Edge Case Handlers: malformed HAR, bundle extraction failures, insufficient evidence detection
- Metrics & Telemetry: comprehensive schema for tracking verification rates over time

**v7.2.0 Additions**:
- STEP 0: Pre-Phase Preparation (auto-extract, auto-detect, auto-context)
- Inline Verification Hooks (claim verification at creation time)
- Checkpoint Summaries (verification reports before phase transitions)

---

## SALESFORCE CASE LOOKUP MANDATE (NON-NEGOTIABLE)

**Whenever SF, Salesforce, or case context is needed, you MUST use the case tools via call_tool.**

This applies to:
- Case number provided (e.g., `DEMO-007`, `SF-DEMO-007`)
- User requests "get case details", "fetch SF context", "check Salesforce"
- Phase 0 Pre-Flight (mandatory SF lookup before triage)
- Any request to verify JIRA content against SF case

### Required Tools

| Tool | Purpose | call_tool Invocation |
|------|---------|---------------------|
| `case-get` | Get case details (customer, product, description, status) | `call_tool({ operation: "execute", tool: "case-get", params: { case_number: "DEMO-007" } })` |
| `case-comments` | Get case comments (customer dialogue, TAC responses, Tenant About Info) | `call_tool({ operation: "execute", tool: "case-comments", params: { case_number: "DEMO-007" } })` |
| `case-search` | Search cases by keyword | `call_tool({ operation: "execute", tool: "case-search", params: { query: "EDL error" } })` |
| `case-similar` | Find similar cases | `call_tool({ operation: "execute", tool: "case-similar", params: { case_number: "DEMO-007" } })` |

### When to Use

| Trigger | Action |
|---------|--------|
| `/investigate DEMO-007` | Call `case-get` + `case-comments` in Phase 0 |
| "Check SF case for context" | Call `case-get` + `case-comments` |
| "Verify JIRA against case" | Call `case-get` to compare details |
| "What did customer report?" | Call `case-comments` for customer dialogue |
| Phase 0 (always) | Mandatory `case-get` + `case-comments` |

### Example Usage

```javascript
// Phase 0: Mandatory SF lookup
const caseDetails = await call_tool({
  operation: "execute",
  tool: "case-get",
  params: { case_number: "DEMO-007" }
});

const caseComments = await call_tool({
  operation: "execute",
  tool: "case-comments",
  params: { case_number: "DEMO-007" }
});

// Extract customer, product, Tenant About Info from response
const { account_name, product, description, status, priority } = caseDetails;
const tenantAboutInfo = caseComments.comments.find(c =>
  c.body.includes('=== Tenant About Info ===')
);
```

### Enforcement

- **DO NOT** use TACO Spot, Vision URLs, or manual browser lookups as primary SF source
- **DO NOT** skip SF lookup in Phase 0
- **DO NOT** generate JIRA without comparing against SF case content
- **ALWAYS** call case tools via `call_tool` orchestrator

---

## Usage

```
/investigate <symptom or case description>
```

**Examples**:
- `/investigate XSOAR 8.5 Docker container OOM killed`
- `/investigate XSIAM correlation rules not matching bind variables`
- `/investigate SF-DEMO-006 automation file read failures`

---

## What This Skill Does

Interactive investigation workflow with human-in-the-loop checkpoints.
Main Claude orchestrates phases, user verifies at checkpoints.

### Agent Usage by Phase (v8.0.0-v1.7)

**v1.7 Architecture Change**: All MCP tool calls now route through single `call_tool` entry point with guaranteed hook execution. Single CAPPY agent orchestrates Phases 2-7.

| Phase | Agent | Tool Call |
|-------|-------|-----------|
| 0-1: Pre-Flight, Discovery | Main Claude | `call_tool(execute, case-get)` + `call_tool(execute, case-comments)` + local file ops |
| 2: Triage | CAPPY Agent | `call_tool(execute, triage_case, params)` |
| 3: Evidence | CAPPY Agent | `call_tool(execute, analyze_evidence, params)` |
| 4: Hypothesis | CAPPY Agent | `call_tool(execute, cappy_synthesis, params)` + `call_tool(execute, validate_solution, params)` |
| 5-6: Validation, Solution | CAPPY Agent | `call_tool(execute, validate_solution, params)` |
| 7: Deliverables | CAPPY Agent | `call_tool(execute, generate_deliverables, params)` |

**v1.7 Benefits**:
- ✅ Single `call_tool` entry point (no multiple MPC tools)
- ✅ Guaranteed hook execution on every call (pre-hooks + post-hooks)
- ✅ Atomic quality gates enforced at phase boundaries (Phase 5 ≥99%, Phase 7 ≥99%)
- ✅ Single CAPPY agent orchestrates all phases (eliminates Bug B-002 permission issues)
- ✅ All v7.10.0 capabilities preserved (Scribe banners, TACO Spot, Vision Direct, PTC workflows)

---

## inv_context.json v3.5 Schema (v8.1.0)

**Schema Reference**: [inv_context_v3.5_schema.md](../../docs/inv_context_v3.5_schema.md)
**Template**: [inv_context_template.json](../../templates/inv_context_template.json)

### Schema Sections (17 total)

| Section | Purpose | Key Fields |
|---------|---------|------------|
| `case` | Identification | id, jira_id, customer, severity, SLA |
| `environment` | Product context | product, version, deployment, integrations |
| `symptom` | Problem statement | summary, error_messages, timeline, impact |
| `evidence` | Files registry | files[], processing status, pending_requests |
| `hypotheses` | Decision tracking | list[], confidence, status, rationale |
| `sessions` | Multi-session | list[], handoffs, active/wait minutes |
| `customer_interactions` | Communication | questions[], calls[], sentiment |
| `related_cases` | Cross-case | linked[], similar_found[], duplicates |
| `claims` | Verification | id, statement, verification.status, citations[] |
| `phases` | Progress | phase_N.status, started_at, completed_at |
| `outcome` | Resolution | classification, novelty, pattern_creation |
| `deliverables` | Outputs | items[], sent status, acknowledgment |
| `metrics` | Value | time_to_resolution_days, engineer_hours_saved |
| `feedback` | Learnings | what_worked, tool_feedback, pattern_suggestions |
| `audit_trail` | History | entries[] with timestamp, actor, action |
| `tool_usage` | Tools | token_budget, toolFallbacks |
| `escalation_attempts` | Escalation | trigger, severity, status |

### Automation Triggers & Verification Sweeps

**Principle: Everything starts UNVERIFIED**

```
New Claim Created    → verification.status = "UNVERIFIED"
New Evidence Added   → processing.status = "PENDING"
New Hypothesis       → status = "PROPOSED"
```

**Trigger Events**:

| Event | Action |
|-------|--------|
| Claim Created | Add to claims[] with status=UNVERIFIED |
| Evidence Uploaded | Add to evidence.files[] with status=PENDING, trigger extraction |
| Phase Completed | Trigger verification sweep |
| Session Started | Review unverified items |
| Deliverable Requested | **BLOCK if unverified claims exist** |

**Verification Sweep (runs after each phase)**:
```
1. Scan claims[] for verification.status != "VERIFIED"
2. Scan evidence.files[] for processing.status == "PENDING"
3. Scan hypotheses.list[] for status == "PROPOSED"
4. Attempt auto-verification (file lookup, pattern match)
5. Flag failures for Main Claude attention
6. Report: "Verification Sweep: 12 claims (10 verified, 2 pending)"
```

**Blocking Rules**:
- Deliverables **BLOCKED** if any claim is UNVERIFIED
- Phase gates **BLOCKED** if verification rate < threshold
- Forces verification before customer-facing output

### Hook Execution Points

```
Evidence Added ──→ [EvidenceRegistration] ──→ [AuditLog]
Tool Executed  ──→ [ClaimExtraction]      ──→ [AuditLog]
Phase Gate     ──→ [VerificationSweep]    ──→ [GateEnforcement]
Deliverable    ──→ [UnverifiedBlocker]    ──→ [DeliverableGeneration]
Session Resume ──→ [SessionResumeReview]
```

### inv_context.json Initialization

**On Phase 0 start**:
```bash
# Copy template to case directory
cp mcp-cappy-prod/templates/inv_context_template.json {case_dir}/inv_context.json

# Or create programmatically with schema v3.5 structure
```

**Claim Creation Pattern**:
```json
{
  "id": "claim-XXX",
  "phase": 3,
  "created_at": "2026-02-13T14:30:00Z",
  "statement": "HTTP 429 errors occurring every 30 seconds",
  "category": "OBSERVATION",
  "verification": {
    "status": "UNVERIFIED",
    "verified_at": null,
    "verified_by": null,
    "confidence": null,
    "method": null
  },
  "citations": [],
  "supporting_links": []
}
```

**Claim Verification Pattern**:
```json
{
  "verification": {
    "status": "VERIFIED",
    "verified_at": "2026-02-13T14:35:00Z",
    "verified_by": "CAPPY",
    "confidence": 0.95,
    "method": "FILE_INSPECTION"
  },
  "citations": [
    {
      "type": "EVIDENCE",
      "source": "server.log",
      "location": "line-4521",
      "excerpt": "Rate limit exceeded",
      "verified": true
    }
  ]
}
```

---

## CAPPY v1.7 Architecture Integration (v8.0.0)

### Single Entry Point: call_tool

All MCP operations now route through a single `call_tool` entry point:

```
Main Claude
  └─ call_tool(operation: "execute", tool: "triage_case", params: {...})
       └─ Routes to: mcp.rs::execute_tool()
            ├─ Pre-hooks (parameter validation, caching)
            ├─ Execute triage_case()
            ├─ Post-hooks (verification, review checks)
            └─ Return: {tool_output, hook_status, verifications}
```

**Hook Status Returned** (CRITICAL):
```json
{
  "error": "...",                    // Pre-hook blocked execution
  "requires_review": true,           // Execution prevented, needs review
  "verifications": [{...}],          // Post-hook verification failures
  "tool_output": {...}               // Actual tool result (if success)
}
```

### Hook Status Interpretation (Mandatory After Every call_tool)

**1. Execution Blocked** (error field present):
```
Response: {"error": "Blocked by pre-hook: Symptom required"}
Action: Ask user to provide missing info, retry
```

**2. Review Required** (requires_review: true, no error):
```
Response: {"requires_review": true, "review_prompt": "Confidence 62% < 99% gate"}
Action: Present options to user (gather evidence, retry, escalate)
```

**3. Verification Failures** (tool_output exists but verifications failed):
```
Response: {"tool_output": {...}, "verifications": [{function, passed, expected, actual, severity}]}
Action: Check severity (Error vs Warning), decide if usable
```

**4. Success** (no error, no requires_review, tool_output returned):
```
Response: {"tool_output": {...}}
Action: Use result, proceed to next phase
```

### Phase Gates (v1.7 Enforced)

**Phase 5 Gate**: `confidence >= 99%`
```
Before: call_tool(execute, validate_solution)
Check: Is confidence_score >= 99%?
  NO  → Phase 5 blocked, show recovery options
  YES → Proceed to Phase 5
```

**Phase 7 Gate**: `confidence >= 99%` AND `all claims verified`
```
Before: call_tool(execute, generate_deliverables)
Check: Is confidence_score >= 99%?
Check: Are all claims status "Verified" or "Excluded"?
  NO  → Phase 7 blocked, show recovery options
  YES → Proceed to Phase 7 (delivery)
```

### Manual Verification After Each Tool Call

After every `call_tool` execution, verify results locally:

```bash
# After triage_case
cd {case_directory}/analysis
jq '.confidence_score' patterns.json
jq '.patterns | length' patterns.json

# After analyze_evidence
jq '.errors | length' errors.json
jq '.events | length' timeline.json

# After generate_deliverables
ls -lh {case_directory}/deliverables/
head -20 {case_directory}/deliverables/customer_response.txt
```

### Tool Fallback & Manual Command Logging (v2.1.0)

**When MCP tools fail or don't fit the use case**, fall back to manual commands and LOG:

**Fallback Flow**:
```
1. Try MCP tool (triage_case, analyze_evidence, etc.)
2. If tool fails OR output insufficient:
   └─ Fall back to manual commands (jq, grep, tar, etc.)
3. LOG to inv_context.json
4. Continue investigation with manual results
```

**Common Fallback Scenarios**:
| Scenario | Tool Attempted | Manual Fallback |
|----------|---------------|-----------------|
| Unsupported bundle format | analyze_evidence | `tar -tzf`, manual extraction |
| HAR too large | analyze_evidence | `jq` with filters |
| Pattern not in database | triage_case | `grep` on log files |
| Custom log format | analyze_evidence | `awk`/`sed` parsing |

**LOG to `inv_context.json`**:
```json
{
  "tool_usage": {
    "toolFallbacks": [{
      "timestamp": "2026-02-13T14:30:00Z",
      "phase": 3,
      "toolAttempted": "analyze_evidence",
      "failureReason": "Bundle format not supported",
      "manualCommandsUsed": [
        "tar -tzf bundle.tar.gz | head -20",
        "jq '.log.entries[] | select(.response.status >= 400)' file.har"
      ],
      "result": "Extracted 45 errors manually",
      "newToolRequired": "analyze_custom_bundle"
    }]
  }
}
```

**Fields**:
| Field | Description |
|-------|-------------|
| `toolAttempted` | MCP tool that was tried |
| `failureReason` | Why tool failed or didn't fit |
| `manualCommandsUsed` | Exact commands used as fallback |
| `result` | What was extracted manually |
| `newToolRequired` | (Optional) Suggested new tool for this use case |

**This feedback loop helps identify gaps in CAPPY tools.**

### CAPPY Agent Strategy

**CAPPY Agent** handles all MCP tool orchestration for Phases 2-7:

**When to Spawn CAPPY Agent**:
```
After Phase 0-1 (environment detected and user confirmed)
  → Main Claude spawns CAPPY Task Agent
  → CAPPY invokes call_tool for each phase
  → Checks hook status after each tool call
  → Enforces gates and displays Scribe banners
  → Returns phase results to Main Claude
  → Checkpoints at phase boundaries
```

**CAPPY Agent Responsibilities**:
- Execute MCP tools via `call_tool(execute, tool, params)`
- Interpret hook status (error, requires_review, verifications, success)
- Check phase gates (Phase 5 ≥99%, Phase 7 ≥99%)
- Register and validate claims via CAPPY-CURATOR capability
- Display Scribe banners between phases
- Manage human-in-the-loop checkpoints
- Escalate to Main Claude if gates fail or user input needed

---

## CAPPY Orchestrator Spawning (v7.10.1+)

**Phase 2 Onwards**: Main Claude spawns CAPPY task agent at start of Phase 2.

### When to Spawn CAPPY

CAPPY is spawned after Phase 1 completion when ready to proceed to triage:

```
Phase 1 Complete → Main Claude says "Ready to proceed to Phase 2"
   ↓
Main Claude spawns CAPPY task agent with Phase 2 context
   ↓
CAPPY reads sub-skills, executes triage_case via call_tool, registers claims
   ↓
CAPPY returns Phase 2 findings + gate status to Main Claude
```

### How to Spawn CAPPY at Phase 2

**Main Claude invokes**:

```
Task spawn agent named "CAPPY" with this specification:

{
  "agent_name": "CAPPY",
  "agent_type": "task_agent",
  "task": "Orchestrate Phase 2 triage investigation",
  "inputs": {
    "investigation_id": "SF-12345678",
    "case_directory": "/case/SF-12345678",
    "symptom": "XSOAR integration timeout every 5 minutes",
    "product": "XSOAR",
    "version": "8.5.0",
    "environment": {
      "deployment_type": "Docker",
      "hostname": "xsoar-prod-01",
      "java_version": "11.0.10"
    }
  },
  "instructions": "Read /agents/CAPPY.md for orchestration procedures. Read sub-skills (curator.md, gate.md) for guidance. Execute triage_case via call_tool, register claims per curator.md, check gate per gate.md. Return Phase 2 results to Main Claude."
}
```

### CAPPY Lifecycle (Phases 2-7)

1. **Spawned at Phase 2**: Main Claude creates CAPPY task agent
2. **Phase 2 Execution**: Orchestrates triage (reads sub-skills, executes triage_case, registers claims, checks gate)
3. **Return to Main Claude**: Reports Phase 2 results and gate status
4. **Phase 3 Cycle**: Main Claude re-invokes CAPPY with Phase 3 context (if gate passed)
5. **Phase 4 Cycle**: Main Claude re-invokes CAPPY with Phase 4 context
6. **Phase 5+**: Bug B-002 forces Main Claude to handle directly (see workaround below)
7. **Escalation**: CAPPY can be re-invoked mid-investigation for emergency escalation (budget exceeded, deadlock)
8. **Terminated**: Investigation complete OR escalated to Praetorian

### CAPPY Invocation Pattern

Each phase follows this pattern:

```
Main Claude (Ready to proceed to Phase 3):
  ANNOUNCE: "Proceeding to Phase 3 evidence analysis with CAPPY..."
  SPAWN: CAPPY task agent with Phase 3 context
    ↓ (CAPPY executes Phase 3 orchestration)
  WAIT: Collect CAPPY response
  RECEIVE: Phase 3 findings + gate status + recommendations
  CHECKPOINT: "Phase 3 gate status: {status}, {recommendations}"
  USER APPROVAL: User says "Proceed to Phase 4" OR "Gather more evidence"
  SPAWN: CAPPY with Phase 4 context (if proceeding)
```

---

## Sub-Skill Integration (v2.0.0)

CAPPY directly reads sub-skill files for guidance and validation logic. No thin agents.

### Sub-Skill Reference

| Sub-Skill | Purpose | When Used |
|-----------|---------|-----------|
| `curator.md` | Evidence management, claim registration | Phase 2-3 |
| `gate.md` | Phase gate thresholds, recovery options | All phases |
| `sherlock.md` | Hypothesis coherence validation | Phase 4 |
| `recon.md` | Environment/architecture validation | Phase 4 |
| `synthesis.md` | Narrative generation | Phase 4 |
| `validate.md` | Solution validation rules | Phase 5 |
| `escalation.md` | Escalation decision trees | When blocked |
| `initialize.md` | Phase 0 setup | Phase 0 |
| `logging.md` | Forensics logging | All phases |

### Sub-Skill Usage by Phase

**Phase 2 (Post-Triage)**:
- Read `curator.md` → Register all claims in inv_context.json
- Read `gate.md` → Check confidence gate (≥90%)
  - PASS → Proceed to Phase 3
  - FAIL → Present recovery options (gather more evidence, lower threshold, escalate)

**Phase 3 (Post-Evidence)**:
- Read `curator.md` → Register extracted evidence claims
- Validate citations (file:line references must exist)
- Read `gate.md` → Check completeness gate (≥99%)
  - PASS → Proceed to Phase 4
  - FAIL → Recovery options (analyze more files, increase depth, skip weak evidence)

**Phase 4 (Post-Hypothesis)**:
- Read `sherlock.md` → Validate hypothesis coherence (assumptions, causality, contradictions)
  - Alignment score <90% → Refine hypothesis or gather more evidence
- Read `recon.md` → Validate environment compatibility (hypothesis matches product/version/build)
  - Mismatch → Adjust hypothesis or note architectural limitations
- Read `synthesis.md` → Generate investigation narrative
- Read `gate.md` → Check coherence gate (≥90%)
  - PASS → Proceed to Phase 5
  - FAIL → Refine hypothesis + evidence chain

**Phase 5-7 (Workaround - Main Claude Direct)**:
- Due to Bug B-002, Phases 5-7 use Main Claude directly
- Main Claude calls: validate_solution → generate_deliverables
- Read `escalation.md` if blocked (budget exceeded, investigation deadlock)

### How CAPPY Uses Sub-Skills

```
CAPPY does:

1. Read curator.md for claim registration guidance
2. Execute triage_case via call_tool
3. Register claims in inv_context.json per curator.md rules
4. Read gate.md for confidence gate logic
5. Check gate: confidence >= 90%?
   - PASS → Return findings to Main Claude
   - FAIL → Return recovery options per gate.md
```

### Mandate Enforcement (Non-Negotiable)

CAPPY enforces these mandates at each phase (logic in sub-skills):

1. **Technical Prowess**: Solutions must be technically sound, not workarounds
2. **Precision**: All claims must be cited (file:line, entry number, or pattern ID)
3. **Credibility**: No unverified assumptions, no fabricated evidence

**If violations detected**:
- Claims uncited → Read curator.md for citation fix guidance
- Hypothesis incoherent → Read sherlock.md for refinement options
- Environment mismatch → Read recon.md for architecture adjustments
- Citation gaps → Block Phase 7 deliverables generation until resolved

---

## CANONICAL CASE DIRECTORY STRUCTURE (v7.7.0)

Every case MUST use this standard structure. All tools read from and write to these canonical paths.

**Templates Location (Canonical)**: `gs://cappy-cache/templates/`
**Templates Location (Local Mirror)**: `/Users/kevtan/Desktop/Builders/01_Projects/cappys-rusty-toolkit/mcp-cappy-prod/templates`

To sync templates:
```bash
# Download from GCS (canonical → local)
gsutil -m rsync -r gs://cappy-cache/templates/ /path/to/local/templates/

# Upload to GCS (local → canonical)
gsutil -m rsync -r /path/to/local/templates/ gs://cappy-cache/templates/
```

```
{case_directory}/
├── inv_context.json          # CANONICAL: Investigation state (auto-created)
├── living_note.md            # CANONICAL: Running notes + RCA (consolidated)
│
├── evidence/                 # Raw customer evidence (READ-ONLY after intake)
│   ├── *.har                 # HAR files
│   ├── *.tar.gz              # Log bundles
│   └── *.png, *.jpg          # Screenshots
│
├── extracted/                # Auto-extracted bundle contents
│   └── {bundle_name}/        # Extracted per-bundle
│
├── analysis/                 # Tool output and intermediate analysis
│   ├── errors.json           # Extracted errors from analyze_evidence
│   ├── timeline.json         # Event timeline
│   └── patterns.json         # Matched patterns from triage_case
│
└── deliverables/             # CANONICAL: Final customer-facing outputs
    ├── customer_response.txt # Customer email (PLAIN TEXT only)
    ├── JIRA_DRAFT.txt        # Combined: plaintext + wiki markup
    └── investigation_summary.html  # HTML archival summary
```

### Canonical File Mandate

**CRITICAL**: These rules are NON-NEGOTIABLE.

1. **Single Source of Truth**: Each deliverable type has ONE canonical file path
2. **No Copies**: Never create `customer_response_v2.md` or `jira_update_draft.md`
3. **Edit In Place**: All revisions go to the canonical file
4. **Git-Style History**: Use `inv_context.json` to track revision history

### Deliverable Paths (Canonical)

| Deliverable Type | Canonical Path | Format | Created By |
|------------------|----------------|--------|------------|
| Customer Response | `{case_dir}/deliverables/customer_response.txt` | Plain text (no markdown) | `generate_deliverables` |
| JIRA Draft | `{case_dir}/deliverables/JIRA_DRAFT.txt` | Combined: plaintext + wiki markup | `generate_deliverables` |
| HTML Summary | `{case_dir}/deliverables/investigation_summary.html` | HTML | `generate_deliverables` |
| Living Note | `{case_dir}/living_note.md` | Markdown (includes RCA) | Phase 0 (auto-created) |
| Context | `{case_dir}/inv_context.json` | JSON | Phase 0 (auto-created) |

### JIRA_DRAFT.txt Format

Combined file with both versions separated by clear headers:

```
================================================================================
                           PLAINTEXT VERSION
              (Copy this for email, Slack, or case notes)
================================================================================

[plaintext content]

================================================================================
                          JIRA WIKI MARKUP
                    (Copy this to paste into JIRA)
================================================================================

{panel:title=...}
[wiki markup content]
{panel}
```

### Customer Response Format

**Plain text only** - no markdown formatting:
- No `##` headers, `**bold**`, or `- bullets`
- Simple line breaks and indentation only
- Ready to paste directly into email or case notes

### Deliverable Citation Requirements (NON-NEGOTIABLE)

**ALL deliverables MUST include citations using technical writing format.**

**Citation Format**:
- Inline references: `[1]`, `[2]`, `[1][2]` (multiple sources for same claim)
- Citations section at bottom of every deliverable
- Each citation must include: source type, file/entry reference, and brief description

**Required Citation Section Format**:
```
---
CITATIONS
[1] HAR:GET /api/endpoint response - description of what this proves
[2] Screenshot: "filename.png" - what the screenshot shows
[3] Log:app-server.log:line_123 - specific log entry
[4] inv_context.json:claim_001 - registered claim reference
```

**What MUST be cited**:
| Claim Type | Citation Example |
|------------|------------------|
| API response data | `[1] HAR:GET /xsoar/incidentfields - field count, status codes` |
| Error messages | `[2] HAR:DELETE /endpoint - HTTP 400, error code 100001` |
| Timestamps | `[3] HAR:entry_145 - 2026-01-14T10:30:45Z` |
| UI observations | `[4] Screenshot: "error_dialog.png"` |
| Configuration values | `[5] Bundle:env.log - XSOAR version 8.7.0` |
| Pattern matches | `[6] Pattern:P-XSOAR-001 - memory leak pattern` |

**Enforcement**:
- Phase 7 (generate_deliverables) MUST add citations to all outputs
- Deliverables without citations section are INVALID
- Main Claude MUST verify citations exist before finalizing
- Character limits apply AFTER citations are added (not before)

**Example Customer Response with Citations**:
```
Your tenant shows error 100001 when attempting deletion [1].
The API returns HTTP 400 because two fields share the same ID [2][3].

---
CITATIONS
[1] HAR:DELETE /xsoar/incidentfield/incident_findings - HTTP 400
[2] HAR:GET /xsoar/incidentfields - two objects with id="incident_findings"
[3] Screenshot: "Case03911746 - XSIAM - Issue Fields - Findings.png"
```

### JIRA Internal References Format (NON-NEGOTIABLE)

**ALL JIRA_DRAFT.txt files MUST include an Internal References section at the bottom.**

This section is for audit trail purposes and MUST NOT be copied to JIRA. It provides:
1. IEEE-formatted citations for traceability
2. Evidence-to-Section mapping for verification

**Required Format**:
```
---
INTERNAL REFERENCES (DO NOT COPY TO JIRA)

IEEE Citations for Audit Trail:
[1] Customer screenshot, "filename.png," SF-XXXXXXXX case files, YYYY-MM-DD.
[2] Video frame extraction, "frames/frame_XXX.png," SF-XXXXXXXX case files, YYYY-MM-DD.
[3] Customer log bundle, "bundle.tar.gz," SF-XXXXXXXX case files, YYYY-MM-DD.
[4] Tenant data export, "tenant_and_license_data.json," SF-XXXXXXXX case files, YYYY-MM-DD.
[5] Salesforce Case, "SF-XXXXXXXX," Palo Alto Networks CRM, YYYY-MM-DD.
[6] JIRA Precedent, "XSUP-XXXXX," Palo Alto Networks Internal, YYYY.

Evidence-to-Section Mapping:
| JIRA Section              | Supporting Evidence                    | Citation |
|---------------------------|----------------------------------------|----------|
| Issue Description         | SF case description, tenant data       | [4], [5] |
| Observed Behavior         | Screenshots, logs, SF case             | [1], [2], [5] |
| Similar Issues            | JIRA precedent                         | [6] |
| Troubleshooting Steps     | Screenshots, verification output       | [1], [3], [4] |
| Tenant info               | tenant_and_license_data.json           | [4] |
| Attachments               | All evidence files                     | [1]-[4] |
```

**IEEE Citation Format by Source Type**:
| Source Type | Format |
|-------------|--------|
| Screenshot | `Customer screenshot, "filename.png," SF-XXXXXXXX case files, YYYY-MM-DD.` |
| Video/Frame | `Video frame extraction, "frames/frame_XXX.png," SF-XXXXXXXX case files, YYYY-MM-DD.` |
| Log bundle | `Customer log bundle, "bundle.tar.gz," SF-XXXXXXXX case files, YYYY-MM-DD.` |
| HAR file | `Network trace, "filename.har," SF-XXXXXXXX case files, YYYY-MM-DD.` |
| SF Case | `Salesforce Case, "SF-XXXXXXXX," Palo Alto Networks CRM, YYYY-MM-DD.` |
| JIRA | `JIRA Issue, "XSUP-XXXXX," Palo Alto Networks Internal, YYYY.` |
| Confluence | `Palo Alto Networks, "Page Title," Confluence Page XXXXXX. [Online]. Available: URL.` |
| Cortex Docs | `Palo Alto Networks, "Topic," Cortex Documentation. [Online]. Available: URL.` |

**Why This Matters**:
1. **Audit Trail**: If Engineer asks "where did you get this info?", point to specific evidence
2. **QA Review**: Peer reviewers can verify claims against cited sources
3. **Escalation Defense**: If customer disputes, documented evidence chain exists
4. **Pattern Learning**: Track which evidence sources are most useful

### Automatic Directory Creation

At Phase 0 (Pre-Flight), create the full structure:

```bash
mkdir -p "{case_directory}/evidence"
mkdir -p "{case_directory}/extracted"
mkdir -p "{case_directory}/analysis"
mkdir -p "{case_directory}/deliverables"
```

### Revision Tracking

When a deliverable is updated, record in `inv_context.json`:

```json
{
  "deliverables": {
    "customer_response": {
      "path": "deliverables/customer_response.txt",
      "created_at": "2026-01-29T10:00:00Z",
      "updated_at": "2026-01-29T14:30:00Z",
      "revision_count": 2,
      "last_updated_by": "Claude",
      "status": "draft"
    },
    "jira_draft": {
      "path": "deliverables/JIRA_DRAFT.txt",
      "created_at": "2026-01-29T10:00:00Z",
      "status": "draft"
    }
  }
}
```

### Status Workflow

```
draft → pending_review → approved → sent
         ↓
       revision_requested (back to draft)
```

### Enforcement

- `generate_deliverables` ALWAYS writes to canonical paths
- Claude MUST check for existing deliverable before regenerating
- If deliverable exists, ask user: "Update existing or regenerate?"
- Never leave orphaned files

---

## SCRIBE INTEGRATION (NEW in v7.4.0 - Updated for v8.0.0-v1.7)

The Scribe library generates visual banners and professional deliverables throughout the investigation.

**Library Location**: `~/.cappy/scribe/`

**v1.7 Integration**: Scribe banners display automatically after CAPPY agent completes each phase and checks hooks/gates. No changes to banner format or content - all v7.4.0 features preserved.

**Banner Display Flow** (v1.7):
```
CAPPY Agent
  → Executes Phase 2: call_tool(execute, triage_case)
  → Checks hook status (error, requires_review, success)
  → Verifies Phase 2 gate (confidence >= 90%)
  → Displays PROGRESS banner with findings
  → Checkpoint: Ask user before Phase 3
```

### Banner Generation Points

| Trigger | Banner Type | Function |
|---------|-------------|----------|
| `/investigate` invoked | START | Display CAPPY logo + case info + problem + phase checklist |
| Phase complete | PROGRESS | Update progress bar + show key findings |
| Investigation resumed | RESUME | Show last session context + next action |
| Case resolved | COMPLETE | Generate full infographic |

### START Banner (Display at Skill Invocation)

After extracting case info and before Phase 0, display:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║     ██████╗ █████╗ ██████╗ ██████╗ ██╗   ██╗                                  ║
║    ██╔════╝██╔══██╗██╔══██╗██╔══██╗╚██╗ ██╔╝                                  ║
║    ██║     ███████║██████╔╝██████╔╝ ╚████╔╝                                   ║
║    ██║     ██╔══██║██╔═══╝ ██╔═══╝   ╚██╔╝                                    ║
║    ╚██████╗██║  ██║██║     ██║        ██║                                     ║
║     ╚═════╝╚═╝  ╚═╝╚═╝     ╚═╝        ╚═╝                                     ║
║                       TOOLKIT                                                 ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   CASE: {{CASE_ID}} {{JIRA_ID}}                                               ║
║   CUSTOMER: {{CUSTOMER}}                                                      ║
║   PRODUCT: {{PRODUCT}}                                                        ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   ┌─────────────────────────────────────────────────────────────────────┐     ║
║   │                         THE PROBLEM                                  │     ║
║   └─────────────────────────────────────────────────────────────────────┘     ║
║                                                                               ║
║   SYMPTOM: {{SYMPTOM}}                                                        ║
║   ERROR: {{ERROR_MESSAGE}}                                                    ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   PHASE PROGRESS                                                              ║
║   ══════════════                                                              ║
║   [▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░] Phase 0: Pre-Flight                       ║
║   [ ] Phase 1: Discovery                                                      ║
║   [ ] Phase 2: Triage                                                         ║
║   [ ] Phase 3: Evidence                                                       ║
║   [ ] Phase 4: Hypothesis                                                     ║
║   [ ] Phase 5: Validation                                                     ║
║   [ ] Phase 6: Solution                                                       ║
║   [ ] Phase 7: Deliverables                                                   ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### PROGRESS Banner (Display After Each Phase)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║     ██████╗ █████╗ ██████╗ ██████╗ ██╗   ██╗                                  ║
║    ██╔════╝██╔══██╗██╔══██╗██╔══██╗╚██╗ ██╔╝    TOOLKIT                       ║
║    ██║     ███████║██████╔╝██████╔╝ ╚████╔╝     ═══════                       ║
║    ╚██████╗██║  ██║██║     ██║        ██║       {{CASE_ID}}                   ║
║     ╚═════╝╚═╝  ╚═╝╚═╝     ╚═╝        ╚═╝       {{CUSTOMER}}                  ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║   PROGRESS  [{{PROGRESS_BAR}}] {{PROGRESS_PCT}}% - {{PHASE_NAME}}             ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   THE PROBLEM                           CURRENT STATUS                        ║
║   ───────────                           ──────────────                        ║
║   {{SYMPTOM_SHORT}}                     {{STATUS_MESSAGE}}                    ║
║                                         Confidence: {{CONFIDENCE}}%           ║
║                                                                               ║
║   KEY FINDINGS                                                                ║
║   ────────────                                                                ║
{{FINDINGS_LIST}}
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### RESUME Banner (Display When Returning to Case)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║     ██████╗ █████╗ ██████╗ ██████╗ ██╗   ██╗                                  ║
║    ██╔════╝██╔══██╗██╔══██╗██╔══██╗╚██╗ ██╔╝    TOOLKIT                       ║
║    ██║     ███████║██████╔╝██████╔╝ ╚████╔╝     ═══════                       ║
║    ╚██████╗██║  ██║██║     ██║        ██║                                     ║
║     ╚═════╝╚═╝  ╚═╝╚═╝     ╚═╝        ╚═╝       ◀◀ RESUMING                   ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║   CASE: {{CASE_ID}}                    CUSTOMER: {{CUSTOMER}}                 ║
║   LAST SESSION: {{LAST_SESSION_DATE}}  DURATION: {{CASE_DURATION}}            ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   STOPPED AT: Phase {{PHASE}} - {{PHASE_NAME}}                                ║
║   CONFIDENCE: {{CONFIDENCE}}%                                                 ║
║   NEXT ACTION: {{NEXT_ACTION}}                                                ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   QUICK CONTEXT                                                               ║
║   ─────────────                                                               ║
║   • Product: {{PRODUCT}}                                                      ║
║   • Issue: {{SYMPTOM_SHORT}}                                                  ║
║   • Hypothesis: {{HYPOTHESIS_SHORT}}                                          ║
║   • Blocker: {{BLOCKER}}                                                      ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Progress Bar Calculation

| Phase | Percentage | Bar (30 chars) |
|-------|------------|----------------|
| 0 | 6% | `▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░` |
| 1 | 19% | `▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░░` |
| 2 | 31% | `▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░` |
| 3 | 44% | `▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░` |
| 4 | 56% | `▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░` |
| 5 | 69% | `▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░` |
| 6 | 81% | `▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░` |
| 7 | 94% | `▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░` |
| Done | 100% | `▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓` |

### Case Resume Detection

Check for existing `inv_context.json` in case directory:
1. If exists and has `current_phase` > 0 → Display RESUME banner
2. If exists but `current_phase` == 0 → Display START banner
3. If not exists → New investigation, display START banner after creating context

### Deliverable Templates

Full templates available in `~/.cappy/scribe/templates/`:
- `JIRA_WIKI_MARKUP.md` - JIRA wiki format
- `JIRA_PLAINTEXT.md` - Plain text for email/case
- `CUSTOMER_RESPONSE.md` - Customer communication templates

Infographic templates in `~/.cappy/scribe/infographics/`:
- `INFOGRAPHIC_TEMPLATE.md` - ASCII art full infographic
- `INFOGRAPHIC_HTML.md` - HTML version for web viewing

---

### STEP 0: PRE-PHASE PREPARATION (Autonomous - Updated v8.3.0)

**ANNOUNCE**: "Preparing case directory. Fetching Salesforce context and extracting evidence..."

**Purpose**: Fetch Salesforce case context FIRST (customer, product, tenant info, related cases), then extract bundles and detect environment. SF context enables better triage. This eliminates 10-15 minutes of manual setup per case.

**Autonomous Actions** (in order):
1. Create case directory structure
2. **SF Case Lookup** - `call_tool(execute, case-get, {case_number})` → customer, product, description, related cases
3. **SF Comments Lookup** - `call_tool(execute, case-comments, {case_number})` → Tenant About Info (auto-posted), customer comments
4. Extract Tenant About Info from auto-posted comment (version, fqdn, project_id, lcaas_id)
5. Find and extract all `.tar.gz` and `.zip` bundles
6. Detect file types from extensions and content
7. Parse `env.log` for additional environment context (if present in bundle)
8. Create `inv_context.json` with ALL collected data (SF + evidence)
9. Generate evidence file manifest

**Why SF First?**: The `case-comments` tool returns the **Tenant About Info** auto-posted by platform integration, providing version, FQDN, project_id, and tenant type WITHOUT asking the customer. This context enables more accurate triage.

**SF Case Lookup Commands**:

```javascript
// Step 1: Get case details
const caseDetails = await call_tool({
  operation: "execute",
  tool: "case-get",
  params: { case_number: "{case_number}" }
});
// Returns: account_name, product, description, status, priority, related cases in description

// Step 2: Get case comments (includes Tenant About Info)
const caseComments = await call_tool({
  operation: "execute",
  tool: "case-comments",
  params: { case_number: "{case_number}" }
});
// Returns: customer comments, TAC responses, TACO PILOT analysis link, Tenant About Info

// Step 3: Extract Tenant About Info from auto-posted comment
const tenantInfo = caseComments.comments.find(c =>
  c.body.includes('=== Tenant About Info ===')
);
// Parse: project_id, lcaas_id, cortex_id, product_type, tenant_type, fqdn, frontend_version
```

**Tenant About Info Fields** (from auto-posted comment):
| Field | Example | Use |
|-------|---------|-----|
| project_id | xdr-us-1004045247219 | Backend identification |
| lcaas_id | 1004045247219 | License/tenant ID |
| cortex_id | 4509875297742532997 | Cortex platform ID |
| product_type | XSIAM | Product identification |
| tenant_type | paying | Customer tier |
| fqdn | customer.product.us.your-vendor.example.com | Tenant URL |
| frontend_version | v3.15.0-4655613-gc0c160a3 | Version info |

**PTC Workflow**:

```javascript
call_tool({
  operation: "workflow",
  template: "custom",
  investigationId: "{case_id}",
  code: `
    const caseDir = '{case_directory}';
    const extractedDir = \`\${caseDir}/extracted\`;

    // Step 1: Create extraction directory
    await execSync(\`mkdir -p "\${extractedDir}"\`);

    // Step 2: Find and extract bundles
    const bundles = await execSync(\`find "\${caseDir}" -maxdepth 1 \\( -name "*.tar.gz" -o -name "*.zip" \\) 2>/dev/null\`);
    const bundleList = bundles.split('\\n').filter(f => f.trim());
    const extractedFiles = [];

    for (const bundle of bundleList) {
      try {
        if (bundle.endsWith('.tar.gz')) {
          await execSync(\`tar -xzf "\${bundle}" -C "\${extractedDir}" 2>&1\`);
        } else if (bundle.endsWith('.zip')) {
          await execSync(\`unzip -o "\${bundle}" -d "\${extractedDir}" 2>&1\`);
        }
        extractedFiles.push(bundle);
      } catch (e) {
        store('extractionErrors', [...(load('extractionErrors') || []), { file: bundle, error: e.message }]);
      }
    }

    // Step 3: Parse env.log for environment context
    let envContext = { product: 'UNKNOWN', version: 'UNKNOWN', build: 'UNKNOWN', hostname: 'UNKNOWN' };
    try {
      const envLogPath = await execSync(\`find "\${caseDir}" -name "env.log" -type f 2>/dev/null | head -1\`);
      if (envLogPath.trim()) {
        const envContent = await execSync(\`cat "\${envLogPath.trim()}"\`);
        envContext = {
          product: (envContent.match(/Product:\\s*(.+)/)?.[1] || 'UNKNOWN').trim(),
          version: (envContent.match(/Version:\\s*(.+)/)?.[1] || 'UNKNOWN').trim(),
          build: (envContent.match(/Build:\\s*(.+)/)?.[1] || 'UNKNOWN').trim(),
          hostname: (envContent.match(/Hostname:\\s*(.+)/)?.[1] || 'UNKNOWN').trim(),
          envLogPath: envLogPath.trim()
        };
      }
    } catch (e) {
      store('envLogError', e.message);
    }

    // Step 4: Build evidence manifest
    const evidenceFiles = [];
    const fileTypes = { har: [], log: [], json: [], screenshot: [], bundle: bundleList };

    const allFiles = await execSync(\`find "\${caseDir}" -type f 2>/dev/null\`);
    allFiles.split('\\n').forEach(f => {
      if (!f || f.includes('inv_context.json')) return;
      const relativePath = f.replace(caseDir + '/', '');
      let type = 'OTHER';
      if (f.endsWith('.har')) { type = 'HAR'; fileTypes.har.push(relativePath); }
      else if (f.endsWith('.log')) { type = 'LOG'; fileTypes.log.push(relativePath); }
      else if (f.endsWith('.json')) { type = 'JSON'; fileTypes.json.push(relativePath); }
      else if (f.match(/\\.(png|jpg|jpeg)$/i)) { type = 'SCREENSHOT'; fileTypes.screenshot.push(relativePath); }
      else if (f.match(/\\.(tar\\.gz|zip)$/)) { type = 'BUNDLE'; }
      evidenceFiles.push({ path: relativePath, type, detected_confidence: 'HIGH' });
    });

    // Step 5: Create inv_context.json
    const invContext = {
      investigation_id: '{case_id}',
      created_at: new Date().toISOString(),
      environment: { ...envContext, detected_at: new Date().toISOString(), confirmed_by_human: false },
      evidence_files: evidenceFiles,
      claims: [],
      verification: { enabled: false, verifiers: [] },
      errors: load('extractionErrors') || []
    };

    const fs = require('fs');
    fs.writeFileSync(\`\${caseDir}/inv_context.json\`, JSON.stringify(invContext, null, 2));

    return {
      success: true,
      bundlesExtracted: extractedFiles.length,
      envContext,
      evidenceCount: evidenceFiles.length,
      fileTypes: {
        har: fileTypes.har.length,
        log: fileTypes.log.length,
        json: fileTypes.json.length,
        screenshot: fileTypes.screenshot.length
      }
    };
  `
})
```

**Fallback Manual Commands**:

```bash
# Create extraction directory
mkdir -p "{case_directory}/extracted"

# Extract bundles
for bundle in "{case_directory}"/*.tar.gz; do
  [ -f "$bundle" ] && tar -xzf "$bundle" -C "{case_directory}/extracted/" 2>&1
done
for zipfile in "{case_directory}"/*.zip; do
  [ -f "$zipfile" ] && unzip -o "$zipfile" -d "{case_directory}/extracted/" 2>&1
done

# Parse env.log
env_log=$(find "{case_directory}" -name "env.log" -type f 2>/dev/null | head -1)
if [ -n "$env_log" ]; then
  echo "Product: $(grep -E '^Product:' "$env_log" | cut -d: -f2- | xargs)"
  echo "Version: $(grep -E '^Version:' "$env_log" | cut -d: -f2- | xargs)"
  echo "Build: $(grep -E '^Build:' "$env_log" | cut -d: -f2- | xargs)"
fi
```

**After STEP 0 completes**, proceed to Context Confirmation:

```
AskUserQuestion({
  questions: [{
    question: `Auto-detected: Product=${product}, Version=${version}. Extracted ${bundleCount} bundles, found ${fileCount} evidence files. Is this correct?`,
    header: "Confirm",
    options: [
      { label: "Correct", description: "Proceed with investigation" },
      { label: "Wrong product/version", description: "I'll provide correct values" },
      { label: "Missing files", description: "I have additional evidence" }
    ],
    multiSelect: false
  }]
})
```

---

### STEP 1: EVIDENCE DISCOVERY (Main Claude)

**ANNOUNCE**: "Starting investigation for [case_id]. Let me identify evidence files."

1. Run `ls -la` on case directory
2. For EACH file found, show the user and ask for context:

```
AskUserQuestion({
  questions: [{
    question: "I found '[filename]' ([size]). What is this file?",
    header: "File Context",
    options: [
      { label: "Log bundle", description: "Server/engine diagnostic logs" },
      { label: "HAR file", description: "Network/API trace" },
      { label: "Data export", description: "Query results, TSV, CSV, JSON" },
      { label: "Screenshot", description: "Visual documentation" }
    ],
    multiSelect: false
  }]
})
```

3. After all files identified, confirm which to analyze:

```
AskUserQuestion({
  questions: [{
    question: "Which files should I analyze?",
    header: "Selection",
    options: [
      { label: "All files", description: "Analyze everything" },
      { label: "Specific files", description: "I'll specify which ones" },
      { label: "Need more files", description: "Evidence is incomplete" }
    ],
    multiSelect: false
  }]
})
```

---

### STEP 2: TRIAGE (Task → CORTEX-CAPPY)

**ANNOUNCE**: "Launching triage phase to find patterns and precedent..."

```
Task({
  subagent_type: "CORTEX-CAPPY",
  prompt: "Phase 1-2 ONLY: Run Pre-Flight and Triage for symptom: '[symptom]'. Files: [file_list]. Return patterns, JIRA matches, confidence.",
  description: "Pre-Flight + Triage"
})
```

**After Task returns**, show results and ask:

```
AskUserQuestion({
  questions: [{
    question: "Triage complete. Confidence: [X]%. How deep should I investigate?",
    header: "Depth",
    options: [
      { label: "QUICK", description: "High confidence match, known issue" },
      { label: "STANDARD (Recommended)", description: "Full analysis for most cases" },
      { label: "DEEP", description: "Novel issue, SEV-1/2, needs research" },
      { label: "Stop here", description: "I have what I need" }
    ],
    multiSelect: false
  }]
})
```

---

### STEP 3: EVIDENCE ANALYSIS (Task → CORTEX-CAPPY)

**ANNOUNCE**: "Analyzing evidence files at [DEPTH] depth..."

```
Task({
  subagent_type: "CORTEX-CAPPY",
  prompt: "Phase 3 ONLY: Analyze files [file_list] at [DEPTH] depth. Extract errors, timeline, findings. Return with file:line citations.",
  description: "Evidence Analysis"
})
```

**After Task returns** (execute ICFP v2.0 inter-phase process - see section below):
1. **MANUALLY VERIFY** key claims by reading actual files
2. **SHOW USER** what agent found vs what you verified
3. **ASK**:

```
AskUserQuestion({
  questions: [{
    question: "Evidence analysis complete. Do the findings match what you see?",
    header: "Verify",
    options: [
      { label: "Looks correct", description: "Proceed to hypothesis" },
      { label: "Some errors", description: "Let me provide corrections" },
      { label: "Major issues", description: "Re-analyze with corrections" },
      { label: "Add more context", description: "I have additional information" }
    ],
    multiSelect: false
  }]
})
```

---

### STEP 4: HYPOTHESIS (Task → CORTEX-CAPPY)

**ANNOUNCE**: "Generating root cause hypothesis..."

```
Task({
  subagent_type: "CORTEX-CAPPY",
  prompt: "Phase 4 ONLY: Generate hypothesis from: [evidence_summary]. Include confidence and supporting evidence.",
  description: "Hypothesis Generation"
})
```

**After Task returns**, show hypothesis and ask:

```
AskUserQuestion({
  questions: [{
    question: "Hypothesis: '[hypothesis]' (Confidence: [X]%). Proceed?",
    header: "Review",
    options: [
      { label: "Proceed", description: "Validate and design solution" },
      { label: "Refine", description: "Adjust hypothesis based on feedback" },
      { label: "More evidence", description: "Need additional data" },
      { label: "Alternative", description: "Consider different root cause" }
    ],
    multiSelect: false
  }]
})
```

---

### STEP 5-6: VALIDATION + SOLUTION (Main Claude - Direct)

**ANNOUNCE**: "Validating hypothesis and designing solution..."

**IMPORTANT**: Do NOT use CORTEX-CAPPY Task agent for validation phase.
See Bug B-002: Claude Code crashes with 'classifyHandoffIfNeeded is not defined' when custom subagents hit permission issues.

**Workaround**: Main Claude performs validation directly using MCP tools:

```
# 1. Research validation using MCP tools directly
validate_solution({
  hypothesis: "[hypothesis]",
  deepResearch: true,
  caseDir: "[case_directory]"
})

# 2. Cross-reference with JIRA/Confluence
research_topic({
  query: "[hypothesis keywords]",
  sources: ["jira", "confluence", "cortex_docs"],
  depth: "thorough"
})

# 3. Manual verification against evidence files
# Use Read, Grep, Bash tools to verify claims
```

**After validation complete**, show solution and ask:

```
AskUserQuestion({
  questions: [{
    question: "Solution designed. Which deliverables do you need?",
    header: "Deliverables",
    options: [
      { label: "Customer response", description: "Email/case update" },
      { label: "JIRA ticket", description: "Engineering escalation" },
      { label: "RCA document", description: "Full root cause analysis" },
      { label: "All of the above", description: "Complete documentation" }
    ],
    multiSelect: true
  }]
})
```

---

### STEP 7: DELIVERABLES (Main Claude - Direct)

**ANNOUNCE**: "Generating [selected_deliverables]..."

**IMPORTANT**: Do NOT use CORTEX-CAPPY Task agent for deliverables phase.
See Bug B-002: Claude Code crashes with 'classifyHandoffIfNeeded is not defined' when custom subagents hit permission issues.

**Workaround**: Main Claude generates deliverables directly using MCP tools:

```
# Use generate_deliverables MCP tool directly
generate_deliverables({
  caseDir: "[case_directory]",
  deliverables: ["customer_response", "jira_update"],
  provider: "claude"
})

# OR manually create deliverables using Write tool
# following templates in deliverables/ directory
```

**After generation complete**:
1. **MANUALLY VERIFY** all facts against evidence files
2. **SHOW USER** generated deliverables
3. **ASK**:

```
AskUserQuestion({
  questions: [{
    question: "Deliverables ready. Are they accurate?",
    header: "Final",
    options: [
      { label: "Finalize", description: "Save to case directory" },
      { label: "Edit needed", description: "Let me make corrections" },
      { label: "Regenerate", description: "Try again with feedback" }
    ],
    multiSelect: false
  }]
})
```

---

### TRANSPARENCY MANDATE

Before ANY action, tell the user what you're doing:
- "I'm listing files in [directory]..."
- "I'm reading [file] to verify [claim]..."
- "I'm launching CORTEX-CAPPY for Phase [X]..."
- "I'm comparing agent output to actual file content..."

**NEVER operate silently.** The user should always know what's happening.

---

### ERROR HANDLING

If user selects "Some errors" or "Major issues":
1. Ask what's wrong
2. Incorporate corrections
3. Re-run phase with corrections

If Task agent fails:
1. Show error to user
2. Ask whether to retry or proceed manually

---

## v4.0.0 7-Tool Architecture

| Tool | Phase | Purpose |
|------|-------|---------|
| `triage_case` | 1-2 | Pattern DB + case search + JIRA lookup |
| `analyze_evidence` | 3 | Bundle extraction, HAR analysis, timeline |
| `research_topic` | Cross | Multi-source research (JIRA, Confluence, Docs, Patterns, **+GitHub**) |
| `cappy_synthesis` | 4 | AI-powered hypothesis, synthesis, summarization |
| `validate_solution` | 5-6 | Multi-source validation + solution design |
| `generate_deliverables` | 7 | Customer response, JIRA content, RCA |
| `call_tool` | Meta | PTC workflows + tool discovery |

**72% reduction** from 25+ tools to 7 tools.

---

## WLA Priority Mapping (TAC Case Prioritization)

Maps TAC Work Level Agreement (WLA) priorities to CAPPY investigation strategy.

### Priority Definitions

| Priority | Definition | Key Criteria |
|----------|------------|--------------|
| **P1 (Critical)** | Total loss or continuous instability of mission-critical functionality | No workaround, revenue/reference impact |
| **P2 (High)** | Impaired but not total loss, or loss of redundancy of critical components | No workaround available |
| **P3 (Medium)** | Function failed but production not affected, or workaround in place | Test env only, or workaround exists for P1/P2 |
| **P4 (Low)** | No impact on customer business | Informational, enhancement requests |

### CAPPY Severity Routing

| WLA Priority | CAPPY Severity | Agent Strategy | Depth |
|--------------|----------------|----------------|-------|
| P1 | SEV-1 | 3 parallel agents, immediate escalation | DEEP |
| P2 | SEV-2 | 2 agents, expedited workflow | DEEP |
| P3 | SEV-3 | Standard single-agent workflow | STANDARD |
| P4 | SEV-4 | Async/batch processing | QUICK |

### Investigation Depth by Priority

```
P1/SEV-1 (Critical):
  ├─ Immediately spawn CORTEX-CAPPY (investigation)
  ├─ Parallel: Pattern + Case + JIRA search
  ├─ Parallel: Bundle + HAR analysis (if available)
  ├─ Auto-escalate if no pattern match in 15 min
  └─ Checkpoint every phase (human-in-the-loop mandatory)

P2/SEV-2 (High):
  ├─ Spawn CORTEX-CAPPY (investigation)
  ├─ Sequential: Triage → Evidence → Hypothesis
  ├─ Research external sources if no match
  └─ Checkpoint at Phase 4 (hypothesis review)

P3/SEV-3 (Medium):
  ├─ Standard workflow
  ├─ Single-agent investigation
  └─ Checkpoint at deliverables

P4/SEV-4 (Low):
  ├─ Quick triage only
  ├─ Pattern match sufficient
  └─ Async response acceptable
```

### Usage in triage_case

The `severity` parameter maps directly to WLA priority:

```json
{
  "symptom": "XSOAR Docker container OOM killed",
  "product": "XSOAR",
  "severity": "SEV-1"  // Maps to P1 (Critical)
}
```

### Reference

- Source: TAC WLA Priorities (Dan Yoffe, 2025-10-08)
- Confluence: Engineering Troubleshooting > Priorities Definition

---

## Programmatic Tool Calling (PTC) Integration

PTC enables **sandboxed multi-tool orchestration** for complex investigations requiring precision and efficiency. Use PTC when multiple tools need parallel execution, conditional logic, or custom data filtering.

### When to Use PTC

| Scenario | Approach | PTC? |
|----------|----------|------|
| Simple pattern lookup | Direct `triage_case` call | ❌ |
| Single HAR analysis | Direct `analyze_evidence` call | ❌ |
| **Parallel triage + evidence** | `call_tool({ operation: "workflow", template: "phase1-parallel" })` | ✅ |
| **Multi-product cascade** | `call_tool({ operation: "workflow", template: "multi-product-cascade" })` | ✅ |
| **Custom investigation logic** | `call_tool({ operation: "workflow", template: "custom", code: "..." })` | ✅ |

### Available PTC Templates

| Template | Use Case |
|----------|----------|
| `phase1-parallel` | Parallel pattern + case + JIRA search |
| `pattern-enrichment` | Enrich patterns with historical cases |
| `har-analysis` | HAR analysis + hypothesis generation |
| `multi-product-cascade` | Cross-product investigation (XSOAR+XSIAM+XDR) |
| `evidence-validation` | Phase 4 multi-source validation |
| `timeline-correlation` | Correlate events across bundle + HAR |
| `full-investigation` | Complete Phases 1-4 automated pipeline |
| `custom` | Dynamic code for unique investigation needs |

### Dynamic Custom Code Generation

**Generate custom PTC code dynamically** based on investigation needs for **precision** and **efficiency**:

#### Key Factors (NOT severity-based)

1. **Investigation Context** (what files are available)
   - Bundle + HAR → Parallel analysis with timeline correlation
   - Bundle only → Focus on log analysis + env extraction
   - HAR only → Network/API focus
   - Neither → Pattern/case/JIRA search only

2. **Symptom Complexity**
   - Single product → Standard triage
   - Multi-product → Cross-product parallel search
   - Intermittent → Timeline-focused analysis

3. **Evidence Type**
   - Log bundle → `analyze_evidence`, extract exit codes, check env.log
   - HAR file → API status codes, timing, rate limits
   - Screenshots → Use observations from previous phases
   - Config files → Diff against working configs

#### Example: Custom Investigation Workflow

When you have a complex investigation with bundle + HAR + multi-product symptoms:

```javascript
call_tool({
  operation: "workflow",
  template: "custom",
  investigationId: "SF-DEMO-003",
  code: `
    const symptom = load('symptom');
    const bundlePath = load('bundlePath');
    const harPath = load('harPath');

    // Parallel Phase 1-2: Triage + Evidence
    const [triage, bundleEvidence, harEvidence] = await parallel([
      triage_case({ symptom, product: 'XSOAR', includePatterns: true, includeCases: true, includeJira: true }),
      bundlePath ? analyze_evidence({ bundlePath, depth: 'deep' }) : Promise.resolve(null),
      harPath ? analyze_evidence({ harPath, depth: 'deep' }) : Promise.resolve(null)
    ]);
    store('phase1-2', { triage, bundleEvidence, harEvidence });

    // Merge timelines for correlation
    const timeline = [...(bundleEvidence?.timeline || []), ...(harEvidence?.timeline || [])]
      .sort((a, b) => new Date(a.timestamp) - new Date(b.timestamp));

    // Phase 3: Hypothesis based on top pattern + evidence
    const hypothesis = await cappy_synthesis({
      task: 'hypothesize',
      context: { patterns: triage.patterns, errors: [...(bundleEvidence?.errors || []), ...(harEvidence?.errors || [])], timeline },
      tier: 'cloud'
    });
    store('phase3', hypothesis);

    // Phase 4: Validate if confidence >= 60
    if (triage.overallConfidence >= 60) {
      const validation = await validate_solution({
        hypothesis: hypothesis.hypothesis || triage.patterns[0]?.symptom,
        deepResearch: true
      });
      store('phase4', validation);
      return { hypothesis: hypothesis.hypothesis, confidence: validation.confidence, validated: validation.validated };
    }

    return { hypothesis: hypothesis.hypothesis, confidence: triage.overallConfidence, validated: false };
  `
})
```

### PTC Best Practices

1. **Use templates for common patterns** - Faster, tested, reliable
2. **Custom code when templates don't fit** - Unique investigation needs
3. **Parallel for independent operations** - Triage + evidence analysis
4. **Sequential for dependent operations** - Analysis → hypothesis → validation
5. **Store intermediate results** - Enables debugging and phase recovery
6. **Return structured data** - Easy to verify and display

---

## ICFP v2.0 Protocol Reference

This skill implements ICFP v2.0 (Inter-Phase Context-Driven Forensics Protocol).

**7-Step Inter-Phase Process**:
1. **Tool Output Verification** - Manually verify MCP tool claims against source files
2. **Context Extraction** - Extract P1/P2/P3 identifiers from inv_context
3. **Case File Forensics** - Search customer files for each identifier
4. **Documentation Research** - Fetch Cortex Docs via `research_topic`, compare documented vs observed
5. **Cross-File Correlation** - Build timeline, identify causal relationships
6. **Context Finalization** - Verify findings recorded, recalculate confidence
7. **Gate Check (BLOCKING)** - Validate before proceeding to next phase

**Key Principles**:
1. **Context-Driven**: Every search traces to identifier in inv_context.json
2. **Continuous Enrichment**: Update context after EVERY action, not just transitions
3. **Agent Autonomy**: Choose appropriate tools (jq, grep, python, etc.)
4. **Drift Prevention**: Every action ties back to original symptom
5. **Documentation as Evidence**: All research recorded with citations

**Gate Check Commands**:
| Check | Command |
|-------|---------|
| P1 identifiers | `jq '[.evidence.items[] \| select(.type=="pattern")] \| length' inv_context.json` |
| Discrepancies | `jq '[.evidence.items[] \| .data.discrepancy \| select(. != null and .resolved != true)] \| length' inv_context.json` |
| PRIMARY evidence | `jq '[.evidence.items[] \| select(.source=="PRIMARY" and .type=="manual_forensics")] \| length' inv_context.json` |
| Confidence score | `jq '.evidence.aggregatedScore' inv_context.json` |
| Research queries | `jq '.research.queries \| length' inv_context.json` |

**Gate Thresholds**:
- Phase 4+: Documentation research required
- Phase 5+: Confidence >= 99%
- Phase 7: Final ICFP must pass (confidence >= 99% for customer response)

---

## Core Mandate: Manual Verification

**EVERY claim about evidence MUST be manually verified. No exceptions.**

### After EVERY Tool Use

1. **Verify** - Use Bash/Read/Grep to parse actual evidence files
2. **Expand** - Call additional MCP tools with specific parameters
3. **Fill gaps** - Investigate what tools missed
4. **Iterate** - Keep calling tools until phase is complete

**Example After `triage_case`**:
```
1. READ pattern descriptions - do symptoms actually match?
2. Use research_topic with cortex_content source for integration source code
3. GREP log files for error codes mentioned in patterns
4. If pattern mentions version X, verify customer is on version X
```

### Evidence Requirements Check (v1.3.9+)

**MANDATORY**: When `triage_case` returns `evidenceRequirements` (confidence < 70%), you MUST:

1. **Check the response** for `evidenceRequirements` field
2. **Load the template** from `~/.../mcp-cappy-prod/templates/first-response/{template}`
3. **Present to user** or include in customer first-response

```typescript
// triage_case response includes:
{
  "overallConfidence": 45,  // < 70% triggers evidence requirements
  "evidenceRequirements": {
    "category": "ui-frontend",
    "template": "ui-frontend.txt",
    "requiredEvidence": ["har", "console_errors", "screenshot", "server_version"],
    "reason": "UI/Frontend issue detected - HAR file with DevTools open before refresh is critical"
  }
}
```

**Action**: When you see `evidenceRequirements`:
- Read the template file
- Present to user: "Before proceeding, we need specific evidence. Here's what to request from the customer:"
- Include the template content

**Template Location**: `mcp-cappy-prod/templates/first-response/`

| Category | Template |
|----------|----------|
| UI/Frontend | `ui-frontend.txt` |
| Integration | `integration-error.txt` |
| Playbook | `playbook-error.txt` |
| Performance | `performance.txt` |
| Content Pack | `content-pack.txt` |

### Verification Commands

| Evidence Type | Command |
|---------------|---------|
| HAR HTTP codes | `jq '[.log.entries[].response.status] \| group_by(.) \| map({(.[0]\|tostring): length}) \| add' file.har` |
| HAR errors | `jq '.log.entries[] \| select(.response.status >= 400)' file.har` |
| Log errors | `grep -c "ERROR\|Exception\|FATAL" *.log` |
| Incident tasks | `jq '[.tasks[] \| select(.pushedToWorkersTime == "0001-01-01T00:00:00Z")] \| length' invPlaybook.json` |

---

## Cross-Verification Framework (v1.2.0)

**Purpose**: Validate AI-processed outputs against their sources of truth.

### When to Cross-Verify

| Output Type | Sources of Truth | When Required |
|-------------|-----------------|---------------|
| **Vision HTML exports** | Log bundles, TSF files, Broker VM logs | Always - Vision applies filtering/deduplication |
| **TACO Pilot exports** | SF cases, JIRA, Confluence, TOIs | Always - links may be stale |
| **AI summaries** | Original customer evidence files | Before Phase 7 deliverables |
| **Pattern matches** | Raw logs, HAR entries | When confidence < 80% |

### Cross-Verification Commands

| Claim Type | Vision/TACO Claim | Cross-Verify Command |
|------------|------------------|---------------------|
| Error count | "8255 errors" | `grep -c '\berror\b' syslog syslog.1` (current logs only) |
| UFW blocks | "4727 ufw_block" | `grep -c 'UFW BLOCK' syslog*` |
| HTTP status | "25 HTTP 500" | `jq '[.log.entries[] \| select(.response.status==500)] \| length' file.har` |
| Time window | "between X and Y" | Check if Vision only analyzed current logs vs archived `.gz` |

### Automatic Hook Triggering

The `CrossVerificationHook` (priority 39) automatically triggers when tool output contains:
- `Vision*.html` or `vision*.html` file references
- `TACO*.html` or `taco*.html` file references

When triggered, it logs:
```
⚠️  Cross-verification recommended: Validate claims against source of truth
```

### HTML Table Extraction

Vision HTML exports contain data in `<td>` table cells. Use these extraction methods:

```python
# Extract all table cell values
python3 -c "
import re
with open('Vision.html', 'r') as f:
    cells = re.findall(r'<td[^>]*>([^<]+)</td>', f.read())
    for c in cells:
        if c.strip() and c.strip().replace(',','').isdigit():
            print(c.strip())
"

# Find specific value in table
python3 -c "
import re
with open('Vision.html', 'r') as f:
    content = f.read()
    # Try with and without commas
    for val in ['8,255', '8255']:
        if val in content:
            idx = content.find(val)
            print(content[max(0,idx-50):idx+len(val)+50])
            break
"
```

### Number Normalization

Vision may display numbers differently than raw logs:

| Vision Display | Normalized | Log Format |
|----------------|------------|------------|
| `8,338` | `8338` | `8338` |
| `1.5K` | `1500` | `1500` |
| `2.3M` | `2300000` | `2300000` |

The `ClaimValidator` hook automatically normalizes numbers when comparing claims.

### Discrepancy Investigation

When cross-verification reveals discrepancies:

1. **Check time window**: Vision may only count current logs (not `.gz` archived)
2. **Check deduplication**: Vision may count unique events, not total occurrences
3. **Check file scope**: Vision may exclude certain log directories
4. **Log to context**: Add discrepancy to `inv_context.json` toolUsage.manualVerifications

```json
{
  "toolUsage": {
    "manualVerifications": [{
      "timestamp": "2026-02-02T16:00:00Z",
      "source": "Vision HTML",
      "toolClaimed": "8255 errors",
      "manualVerified": "6378 in current logs, 31006 total",
      "discrepancy": true,
      "resolution": "Vision counts current logs only"
    }]
  }
}
```

---

## cappy_synthesis Usage

Phase 4 AI-powered hypothesis and synthesis:

```typescript
cappy_synthesis({
  task: "hypothesize",  // or "synthesize", "summarize", "compare"
  context: { evidence, patterns, timeline },
  tier: "local",  // Use for sensitive customer data (FREE)
  // tier: "cloud"  // Use for general analysis
})
```

**Tasks**:
| Task | Purpose |
|------|---------|
| `hypothesize` | Generate root cause hypothesis from evidence |
| `synthesize` | Combine multiple evidence sources into coherent analysis |
| `summarize` | Compress context for next phase or customer |
| `compare` | Diff working vs failing configs to find differentiating variable |

---

## research_topic Sources

| Source | Purpose |
|--------|---------|
| `cortex_docs` | Official Cortex documentation |
| `confluence` | TAC playbooks, KB articles |
| `jira` | Bug lookup, known issues |
| `patterns` | 400-pattern database |
| `cortex_content` | **NEW** GitHub platform org (integration source code) |

**Example**:
```typescript
research_topic({
  query: "ServiceNow incident_fields parser",
  sources: ["cortex_content"],  // Search integration source code
  depth: "thorough"
})
```

---

## Evidence Hierarchy

| Source | Weight | Examples |
|--------|--------|----------|
| **PRIMARY** | 100% | Raw logs, HAR, PCAP, configs, screenshots |
| **SECONDARY** | 80% | Parsed logs, TAC playbooks, JIRA |
| **TERTIARY** | 50% | AI suggestions, historical cases, patterns |

---

## Novel Issue Protocol (Zero Patterns)

**Zero patterns = Novel issue. Proceed with manual forensics. DO NOT BLOCK.**

When `triage_case` returns zero patterns:
1. Mark as `"novelIssue": true` in context
2. Proceed to Phase 3 (Evidence Analysis)
3. Use `research_topic` with `cortex_content` source for integration source code
4. Generate symptom-based hypothesis via `cappy_synthesis`

---

## 7-Phase Workflow Quick Reference

| Phase | Primary Tool | Fallback |
|-------|--------------|----------|
| 1: Pre-Flight | `analyze_evidence` (bundlePath) | Manual env.log parsing |
| 2: Triage | `triage_case` | `research_topic` |
| 3: Evidence | `analyze_evidence` | Manual jq/grep |
| 4: Hypothesis | `cappy_synthesis` | Manual analysis |
| 5: Validation | `validate_solution` | `research_topic` |
| 6: Solution | (from Phase 5) | `research_topic` |
| 7: Deliverables | `generate_deliverables` | Manual drafting |

---

## Tool Usage Logging

All tool calls and manual verifications should be logged to `inv_context.json`:

```json
{
  "toolUsage": {
    "mcpToolCalls": [{
      "timestamp": "2026-01-14T10:30:00Z",
      "tool": "triage_case",
      "phase": "TRIAGE",
      "result": { "success": true, "patternsMatched": 2 }
    }],
    "manualVerifications": [{
      "timestamp": "2026-01-14T10:32:00Z",
      "command": "jq '[.log.entries[].response.status]...' file.har",
      "discrepancy": {
        "toolClaimed": "25 HTTP 200",
        "actualFound": "24 HTTP 200, 1 HTTP 500"
      }
    }],
    "bugsIdentified": [{
      "tool": "analyze_evidence",
      "issue": "HAR analyzer missed HTTP 500",
      "severity": "HIGH"
    }]
  }
}
```

### METRICS & TELEMETRY (NEW in v7.3.0)

Track verification success rates and investigation quality over time.

**Schema** - Add to `inv_context.json`:

```json
{
  "metrics": {
    "investigation_id": "SF-DEMO-003",
    "started_at": "2026-01-19T09:00:00Z",
    "completed_at": "2026-01-19T12:30:00Z",
    "duration_minutes": 210,

    "verification": {
      "total_claims": 15,
      "verified_claims": 13,
      "unverified_claims": 2,
      "verification_rate": 86.7,
      "recovery_attempts": 3,
      "recovery_successes": 1,
      "recovery_rate": 33.3
    },

    "phases": {
      "pre_flight": { "duration_ms": 45000, "success": true },
      "triage": { "duration_ms": 12000, "patterns_found": 3, "confidence": 82 },
      "evidence": { "duration_ms": 180000, "files_analyzed": 5, "errors_found": 12 },
      "hypothesis": { "duration_ms": 25000, "confidence": 78 },
      "validation": { "duration_ms": 90000, "sources_checked": 4 },
      "deliverables": { "duration_ms": 60000, "types_generated": ["customer_response", "jira_update"] }
    },

    "edge_cases": {
      "encountered": ["HAR_MISSING_ENTRIES", "BUNDLE_WRONG_EXTENSION"],
      "recovered": ["BUNDLE_WRONG_EXTENSION"],
      "failed": ["HAR_MISSING_ENTRIES"]
    },

    "quality": {
      "final_confidence": 85,
      "recommendation": "APPROVE",
      "human_corrections": 1,
      "deliverable_revisions": 0
    }
  }
}
```

**PTC Workflow to Generate Metrics**:

```javascript
call_tool({
  operation: "workflow",
  template: "custom",
  investigationId: "{case_id}",
  code: `
    const caseDir = load('caseDir') || '{case_directory}';
    const fs = require('fs');

    let invContext = JSON.parse(fs.readFileSync(\`\${caseDir}/inv_context.json\`, 'utf8'));
    const claims = invContext.claims || [];
    const toolUsage = invContext.toolUsage || {};

    // Calculate verification metrics
    const verified = claims.filter(c => c.verified === true).length;
    const unverified = claims.filter(c => c.verified === false).length;
    const recoveryAttempts = claims.filter(c => c.recovery_attempted).length;
    const recoverySuccesses = claims.filter(c => c.recovery_applied).length;

    // Calculate phase durations from tool calls
    const phases = {};
    const phaseCalls = toolUsage.mcpToolCalls || [];
    const phaseMap = {
      'triage_case': 'triage',
      'analyze_evidence': 'evidence',
      'cappy_synthesis': 'hypothesis',
      'validate_solution': 'validation',
      'generate_deliverables': 'deliverables'
    };

    for (const call of phaseCalls) {
      const phase = phaseMap[call.tool] || 'other';
      if (!phases[phase]) phases[phase] = { duration_ms: 0, calls: 0, success: true };
      phases[phase].calls++;
      if (call.duration_ms) phases[phase].duration_ms += call.duration_ms;
      if (call.result?.success === false) phases[phase].success = false;
    }

    // Build metrics
    const startTime = new Date(invContext.created_at);
    const endTime = new Date();

    invContext.metrics = {
      investigation_id: invContext.investigation_id,
      started_at: invContext.created_at,
      completed_at: endTime.toISOString(),
      duration_minutes: Math.round((endTime - startTime) / 60000),

      verification: {
        total_claims: claims.length,
        verified_claims: verified,
        unverified_claims: unverified,
        verification_rate: claims.length > 0 ? Math.round((verified / claims.length) * 1000) / 10 : 0,
        recovery_attempts: recoveryAttempts,
        recovery_successes: recoverySuccesses,
        recovery_rate: recoveryAttempts > 0 ? Math.round((recoverySuccesses / recoveryAttempts) * 1000) / 10 : 0
      },

      phases,

      edge_cases: {
        encountered: invContext.errors?.map(e => e.error) || [],
        recovered: invContext.errors?.filter(e => e.recovery_applied)?.map(e => e.error) || [],
        failed: invContext.errors?.filter(e => e.recovery_failed)?.map(e => e.error) || []
      },

      quality: {
        final_confidence: invContext.verification_summary?.verified_percent || 0,
        recommendation: invContext.verification_summary?.recommendation || 'UNKNOWN',
        human_corrections: (toolUsage.manualVerifications || []).filter(v => v.discrepancy).length,
        deliverable_revisions: 0
      }
    };

    fs.writeFileSync(\`\${caseDir}/inv_context.json\`, JSON.stringify(invContext, null, 2));

    return { success: true, metrics: invContext.metrics };
  `
})
```

**Aggregate Metrics Over Time** (for skill improvement):

Store metrics from completed investigations to track:
- Average verification rate across cases
- Most common edge cases encountered
- Recovery success rate by error type
- Average investigation duration by product

```bash
# Export metrics from all completed investigations
find ~/Desktop/Investigations -name "inv_context.json" -exec jq '.metrics' {} \; > all_metrics.jsonl

# Calculate average verification rate
jq -s '[.[] | select(.verification.verification_rate != null) | .verification.verification_rate] | add / length' all_metrics.jsonl

# Most common edge cases
jq -s '[.[].edge_cases.encountered[]] | group_by(.) | map({error: .[0], count: length}) | sort_by(.count) | reverse' all_metrics.jsonl
```

---

## Context Recovery

If `inv_context.json` is corrupted:

1. Check for backups: `ls -la $CASE_DIR/inv_context.json.bak.*`
2. Restore: `cp inv_context.json.bak.PHASE_2 inv_context.json`
3. Validate: `python3 -c "import json; json.load(open('inv_context.json'))"`

---

## If Evidence Is Insufficient

**NEVER fabricate facts.** Request targeted files:

| Symptom | Request |
|---------|---------|
| UI errors | HAR file from affected page |
| Playbook not running | Incident export + d1.log |
| Container OOM | docker logs + memory stats |
| Agent connectivity | Agent logs + proxy config |

---

---

## INLINE VERIFICATION HOOKS (NEW in v7.2.0)

Inline verification catches false claims at creation time before they reach deliverables. Every claim from tool output is verified against actual evidence files.

### Verification Setup (Run After STEP 0)

**ANNOUNCE**: "Configuring verification based on available evidence files..."

**PTC Workflow**:

```javascript
call_tool({
  operation: "workflow",
  template: "custom",
  investigationId: "{case_id}",
  code: `
    const caseDir = load('caseDir') || '{case_directory}';
    const fs = require('fs');

    // Load context
    let invContext = JSON.parse(fs.readFileSync(\`\${caseDir}/inv_context.json\`, 'utf8'));

    // Ensure environment is confirmed
    if (!invContext.environment?.confirmed_by_human) {
      return { success: false, error: 'Environment not confirmed', recommendation: 'Complete Context Confirmation first' };
    }

    const files = invContext.evidence_files || [];

    // Analyze available file types
    const fileTypes = {
      har: files.filter(f => f.path?.endsWith('.har') || f.type === 'HAR'),
      log: files.filter(f => f.path?.endsWith('.log') || f.type === 'LOG'),
      json: files.filter(f => f.path?.endsWith('.json') && !f.path?.includes('inv_context'))
    };

    // Define verifiers based on available files
    const verifiers = [];

    if (fileTypes.har.length > 0) {
      verifiers.push(
        { name: 'http_status_verifier', applies_to: ['*.har'], method: 'jq', claim_types: ['http_status'], priority: 1 },
        { name: 'har_count_verifier', applies_to: ['*.har'], method: 'jq', claim_types: ['count'], priority: 1 },
        { name: 'har_timing_verifier', applies_to: ['*.har'], method: 'jq', claim_types: ['timing'], priority: 2 }
      );
    }

    if (fileTypes.log.length > 0) {
      verifiers.push(
        { name: 'error_count_verifier', applies_to: ['*.log'], method: 'grep', claim_types: ['count', 'error_count'], priority: 1 },
        { name: 'log_pattern_verifier', applies_to: ['*.log'], method: 'grep', claim_types: ['pattern_exists'], priority: 1 }
      );
    }

    if (fileTypes.json.length > 0) {
      verifiers.push(
        { name: 'field_existence_verifier', applies_to: ['*.json'], method: 'jq', claim_types: ['field_exists'], priority: 1 },
        { name: 'field_value_verifier', applies_to: ['*.json'], method: 'jq', claim_types: ['field_value'], priority: 1 }
      );
    }

    // Determine verification depth from pattern confidence
    const triageResult = load('triageResult');
    let depth = 'STANDARD';
    if (triageResult?.patterns?.[0]?.confidence === 'DEFINITIVE') depth = 'LIGHT';
    else if (triageResult?.patterns?.[0]?.confidence === 'MODERATE') depth = 'DEEP';

    // Update context with verification config
    invContext.verification = {
      enabled: true,
      depth,
      configured_at: new Date().toISOString(),
      verifiers: verifiers.filter(v => depth === 'DEEP' || v.priority <= (depth === 'LIGHT' ? 1 : 2)),
      file_coverage: { har: fileTypes.har.length, log: fileTypes.log.length, json: fileTypes.json.length }
    };

    fs.writeFileSync(\`\${caseDir}/inv_context.json\`, JSON.stringify(invContext, null, 2));

    return {
      success: true,
      depth,
      enabledVerifiers: invContext.verification.verifiers.length,
      fileCoverage: invContext.verification.file_coverage
    };
  `
})
```

**Fallback Manual Commands**:

```bash
# Count HAR files
har_count=$(jq '[.evidence_files[] | select(.path | endswith(".har"))] | length' "{case_directory}/inv_context.json")

# Count log files
log_count=$(jq '[.evidence_files[] | select(.path | endswith(".log"))] | length' "{case_directory}/inv_context.json")

# Add verification section based on file types
jq '.verification = { "enabled": true, "depth": "STANDARD", "configured_at": (now | todate) }' \
  "{case_directory}/inv_context.json" > tmp.json && mv tmp.json "{case_directory}/inv_context.json"
```

### Verification Depth Levels

| Depth | Description | Verifiers Active | Use When |
|-------|-------------|------------------|----------|
| LIGHT | Count verification only | 2-3 | DEFINITIVE patterns (95%+ confidence) |
| STANDARD | Count + type verification | 5-7 | STRONG patterns (80-95% confidence) |
| DEEP | All verifiers | 10+ | MODERATE patterns (<80% confidence) |

---

### Claim Verification Functions

Use these verification functions during investigation to verify claims inline:

**PTC Claim Verification**:

```javascript
// Verification helper functions (use within PTC workflows)
const verifyCount = async (claimedCount, subject, harFile) => {
  let jqQuery;
  if (subject.includes('HTTP 200')) jqQuery = '[.log.entries[] | select(.response.status == 200)] | length';
  else if (subject.includes('HTTP 4')) jqQuery = '[.log.entries[] | select(.response.status >= 400 and .response.status < 500)] | length';
  else if (subject.includes('HTTP 5')) jqQuery = '[.log.entries[] | select(.response.status >= 500)] | length';
  else jqQuery = '.log.entries | length';

  const result = await execSync(`jq '${jqQuery}' "${harFile}"`);
  const actualCount = parseInt(result.trim());
  return {
    verified: claimedCount === actualCount,
    actual: actualCount,
    discrepancy: claimedCount !== actualCount ? `Claimed ${claimedCount}, actual ${actualCount}` : null
  };
};

const verifyHttpStatus = async (claimedStatus, entryIndex, harFile) => {
  const result = await execSync(`jq '.log.entries[${entryIndex}].response.status' "${harFile}"`);
  const actualStatus = parseInt(result.trim());
  return {
    verified: claimedStatus === actualStatus,
    actual: actualStatus,
    discrepancy: claimedStatus !== actualStatus ? `Claimed ${claimedStatus}, actual ${actualStatus}` : null
  };
};

const verifyFieldExists = async (fieldName, jsonFile) => {
  const result = await execSync(`jq 'has("${fieldName}")' "${jsonFile}"`);
  const exists = result.trim() === 'true';
  return { verified: exists, actual: exists, discrepancy: !exists ? `Field "${fieldName}" not found` : null };
};

const verifyErrorCount = async (claimedCount, pattern, logFile) => {
  const result = await execSync(`grep -c "${pattern}" "${logFile}" || echo 0`);
  const actualCount = parseInt(result.trim());
  return {
    verified: claimedCount === actualCount,
    actual: actualCount,
    discrepancy: claimedCount !== actualCount ? `Claimed ${claimedCount}, actual ${actualCount}` : null
  };
};

// NEW v7.3.0: Additional verification functions
const verifyTimestampRange = async (claimedStart, claimedEnd, harFile) => {
  const result = await execSync(`jq '[.log.entries[].startedDateTime] | sort | [first, last]' "${harFile}"`);
  const [actualStart, actualEnd] = JSON.parse(result.trim());
  const startMatch = new Date(claimedStart).getTime() === new Date(actualStart).getTime();
  const endMatch = new Date(claimedEnd).getTime() === new Date(actualEnd).getTime();
  return {
    verified: startMatch && endMatch,
    actual: { start: actualStart, end: actualEnd },
    discrepancy: (!startMatch || !endMatch) ? `Claimed ${claimedStart} to ${claimedEnd}, actual ${actualStart} to ${actualEnd}` : null
  };
};

const verifyContentPattern = async (pattern, shouldExist, file) => {
  try {
    const result = await execSync(`grep -l "${pattern}" "${file}" 2>/dev/null`);
    const exists = result.trim().length > 0;
    return {
      verified: exists === shouldExist,
      actual: exists,
      discrepancy: (exists !== shouldExist) ? `Pattern "${pattern}" ${exists ? 'found' : 'not found'}, expected ${shouldExist ? 'to exist' : 'to not exist'}` : null
    };
  } catch (e) {
    return { verified: !shouldExist, actual: false, discrepancy: shouldExist ? `Pattern "${pattern}" not found` : null };
  }
};

const verifyFileSize = async (claimedSize, file, tolerance = 0.1) => {
  const result = await execSync(`stat -f%z "${file}" 2>/dev/null || stat --format=%s "${file}"`);
  const actualSize = parseInt(result.trim());
  const lowerBound = claimedSize * (1 - tolerance);
  const upperBound = claimedSize * (1 + tolerance);
  const withinTolerance = actualSize >= lowerBound && actualSize <= upperBound;
  return {
    verified: withinTolerance,
    actual: actualSize,
    discrepancy: !withinTolerance ? `Claimed ${claimedSize} bytes, actual ${actualSize} bytes (${Math.round((actualSize - claimedSize) / claimedSize * 100)}% difference)` : null
  };
};

const verifyEncoding = async (expectedEncoding, file) => {
  const result = await execSync(`file -b --mime-encoding "${file}"`);
  const actualEncoding = result.trim().toLowerCase();
  const expected = expectedEncoding.toLowerCase();
  const match = actualEncoding === expected || (expected === 'utf-8' && actualEncoding === 'us-ascii');
  return {
    verified: match,
    actual: actualEncoding,
    discrepancy: !match ? `Expected encoding ${expectedEncoding}, actual ${actualEncoding}` : null
  };
};
```

**Fallback Manual Verification Commands**:

```bash
# HTTP 200 count in HAR
actual=$(jq '[.log.entries[] | select(.response.status == 200)] | length' "{har_file}")
echo "Claimed: {claimed}, Actual: $actual"

# HTTP 4xx count in HAR
actual=$(jq '[.log.entries[] | select(.response.status >= 400 and .response.status < 500)] | length' "{har_file}")

# HTTP 5xx count in HAR
actual=$(jq '[.log.entries[] | select(.response.status >= 500)] | length' "{har_file}")

# Error count in log file
actual=$(grep -c "ERROR" "{log_file}" || echo 0)

# Field existence in JSON
exists=$(jq 'has("{field_name}")' "{json_file}")
echo "Field '{field_name}' exists: $exists"

# Verify specific entry status
actual=$(jq '.log.entries[{entry_index}].response.status' "{har_file}")
```

### Claim Storage Format

Store verified claims in `inv_context.json`:

```json
{
  "claims": [
    {
      "id": "claim_001",
      "tool": "analyze_evidence",
      "claim": "Found 25 HTTP 200 responses",
      "type": "count",
      "claimed_value": 25,
      "verified_value": 24,
      "verified": false,
      "discrepancy": "Claimed 25, actual 24",
      "evidence_ref": "network_trace.har",
      "verification_method": "jq '[.entries[] | select(.response.status == 200)] | length'",
      "timestamp": "2026-01-19T10:35:00Z"
    }
  ]
}
```

### Discrepancy Handling

If `verified: false`:
1. Log discrepancy to claims array
2. Flag for human review at checkpoint
3. Continue with investigation (don't block)
4. **NEVER use unverified claims in deliverables**

---

### ERROR RECOVERY GUIDANCE (NEW in v7.3.0)

When verification fails, don't just stop - try recovery actions:

| Failure Type | Recovery Action | PTC Recovery |
|--------------|-----------------|--------------|
| **Count mismatch** | Re-run with corrected jq filter | `await verifyCount(newCount, subject, file)` |
| **HTTP status mismatch** | Check specific entry, verify index | `jq '.log.entries[N].response.status'` |
| **Field not found** | Check alternate field names, nested paths | `jq 'paths \| select(.[-1] == "fieldname")'` |
| **Timestamp range wrong** | Verify timezone, check for filtered entries | `jq '.log.entries \| length'` to confirm count |
| **Pattern not found** | Try case-insensitive, partial match | `grep -i` or `grep -E` |
| **File size mismatch** | Verify correct file path, check compression | `ls -la` and `file` commands |
| **Encoding mismatch** | Convert encoding or note difference | `iconv -f X -t utf-8` |

**Recovery Decision Tree**:

```
Verification Failed
    │
    ├─ Is this a CRITICAL claim (used in customer deliverable)?
    │   ├─ YES → Attempt recovery action → Re-verify
    │   │         └─ Still fails → Flag for MANUAL review
    │   └─ NO → Log discrepancy → Continue investigation
    │
    └─ Is discrepancy > 20%?
        ├─ YES → Re-analyze source file → Check for parsing errors
        └─ NO → Accept with warning → Log for pattern improvement
```

**PTC Recovery Example**:

```javascript
// Recovery workflow for count mismatch
const recoverCountMismatch = async (claim, caseDir) => {
  const fs = require('fs');

  // Try alternative jq queries
  const alternativeQueries = [
    '[.log.entries[] | select(.response.status == 200)] | length',
    '[.log.entries[] | select(.response.status >= 200 and .response.status < 300)] | length',
    '.log.entries | map(select(.response.status == 200)) | length'
  ];

  for (const query of alternativeQueries) {
    try {
      const result = await execSync(`jq '${query}' "${claim.evidence_ref}"`);
      const count = parseInt(result.trim());
      if (count === claim.claimed_value) {
        // Found matching query - update verification method
        claim.verification_method = query;
        claim.verified = true;
        claim.recovery_applied = true;
        return { recovered: true, claim };
      }
    } catch (e) { continue; }
  }

  // All recovery attempts failed
  claim.recovery_attempted = true;
  claim.recovery_failed = true;
  return { recovered: false, claim, recommendation: 'MANUAL_REVIEW' };
};
```

---

### EDGE CASE HANDLERS (NEW in v7.3.0)

Handle common failure scenarios gracefully during investigation.

#### Malformed HAR File

**Detection**: jq parse error, missing `.log.entries`, unexpected structure

```javascript
// PTC Edge Case Handler: Malformed HAR
const handleMalformedHar = async (harFile, caseDir) => {
  const fs = require('fs');

  // Step 1: Check if file is valid JSON
  try {
    await execSync(`jq empty "${harFile}" 2>&1`);
  } catch (e) {
    // Not valid JSON - try to identify issue
    const preview = await execSync(`head -c 500 "${harFile}"`);
    if (preview.includes('<!DOCTYPE') || preview.includes('<html')) {
      return { error: 'HAR_IS_HTML', message: 'File is HTML, not HAR JSON. Customer may have saved webpage instead of exporting HAR.', recommendation: 'Request HAR export via: Chrome DevTools → Network → Right-click → Save all as HAR' };
    }
    if (preview.startsWith('PK')) {
      return { error: 'HAR_IS_ZIP', message: 'File is ZIP archive. Extract and re-analyze.', recommendation: `unzip "${harFile}" -d extracted/` };
    }
    return { error: 'HAR_INVALID_JSON', message: `Invalid JSON: ${e.message}`, recommendation: 'Request fresh HAR export from customer' };
  }

  // Step 2: Check for required HAR structure
  const hasLog = await execSync(`jq 'has("log")' "${harFile}"`);
  if (hasLog.trim() !== 'true') {
    return { error: 'HAR_MISSING_LOG', message: 'HAR missing .log property', recommendation: 'Non-standard HAR format. Try: jq \'.entries // .log.entries\' to find entries.' };
  }

  const hasEntries = await execSync(`jq '.log | has("entries")' "${harFile}"`);
  if (hasEntries.trim() !== 'true') {
    return { error: 'HAR_MISSING_ENTRIES', message: 'HAR missing .log.entries array', recommendation: 'HAR may be empty or truncated. Check file size and request new export.' };
  }

  return { success: true, message: 'HAR structure valid' };
};
```

**Fallback Manual Commands**:

```bash
# Check if HAR is valid JSON
jq empty "{har_file}" 2>&1 || echo "Invalid JSON"

# Check for HTML (wrong file type)
head -c 100 "{har_file}" | grep -q "<!DOCTYPE\|<html" && echo "File is HTML, not HAR"

# Check for ZIP
file "{har_file}" | grep -q "Zip archive" && echo "File is ZIP, extract first"

# Verify HAR structure
jq 'has("log") and (.log | has("entries"))' "{har_file}"
```

#### Bundle Extraction Failure

**Detection**: tar/unzip errors, corrupted archives, password-protected files

```javascript
// PTC Edge Case Handler: Bundle Extraction Failure
const handleBundleExtractionFailure = async (bundlePath, caseDir, error) => {
  const fs = require('fs');

  // Identify failure type
  const errorMsg = error.toLowerCase();

  if (errorMsg.includes('unexpected end of file') || errorMsg.includes('truncated')) {
    return {
      error: 'BUNDLE_TRUNCATED',
      message: 'Bundle file is truncated/incomplete',
      recommendation: 'File was cut off during transfer. Request re-upload.',
      recovery: null
    };
  }

  if (errorMsg.includes('password') || errorMsg.includes('encrypted')) {
    return {
      error: 'BUNDLE_ENCRYPTED',
      message: 'Bundle is password-protected',
      recommendation: 'Request password or unencrypted bundle',
      recovery: null
    };
  }

  if (errorMsg.includes('permission denied') || errorMsg.includes('cannot open')) {
    return {
      error: 'BUNDLE_PERMISSION',
      message: 'Permission denied accessing bundle',
      recommendation: `Run: chmod 644 "${bundlePath}"`,
      recovery: `await execSync(\`chmod 644 "${bundlePath}"\`)`
    };
  }

  if (errorMsg.includes('not in gzip format') || errorMsg.includes('invalid magic')) {
    // Try alternate extraction methods
    const fileType = await execSync(`file -b "${bundlePath}"`);
    if (fileType.includes('Zip archive')) {
      return {
        error: 'BUNDLE_WRONG_EXTENSION',
        message: 'File is ZIP but has .tar.gz extension',
        recommendation: `Use: unzip "${bundlePath}" instead of tar`,
        recovery: `await execSync(\`unzip -o "${bundlePath}" -d "${caseDir}/extracted/"\`)`
      };
    }
    return {
      error: 'BUNDLE_WRONG_FORMAT',
      message: `File is ${fileType.trim()}, not gzip`,
      recommendation: 'Check file format and use appropriate extraction tool',
      recovery: null
    };
  }

  // Unknown error
  return {
    error: 'BUNDLE_UNKNOWN_ERROR',
    message: error,
    recommendation: 'Check bundle integrity with: gzip -t file.tar.gz',
    recovery: null
  };
};
```

**Fallback Manual Commands**:

```bash
# Test bundle integrity
gzip -t "{bundle_file}" 2>&1 && echo "Valid gzip" || echo "Corrupted gzip"

# Check actual file type
file -b "{bundle_file}"

# Try alternate extraction
unzip -t "{bundle_file}" 2>&1  # Test as ZIP
tar -tzf "{bundle_file}" 2>&1  # List contents without extracting

# Fix permissions
chmod 644 "{bundle_file}"
```

#### Empty or Minimal Evidence

**Detection**: No log entries, empty HAR, insufficient data for analysis

```javascript
// PTC Edge Case Handler: Insufficient Evidence
const handleInsufficientEvidence = async (caseDir) => {
  const fs = require('fs');
  const invContext = JSON.parse(fs.readFileSync(`${caseDir}/inv_context.json`, 'utf8'));

  const issues = [];
  const recommendations = [];

  // Check HAR entries
  for (const file of invContext.evidence_files.filter(f => f.type === 'HAR')) {
    try {
      const count = await execSync(`jq '.log.entries | length' "${caseDir}/${file.path}"`);
      if (parseInt(count.trim()) < 5) {
        issues.push({ file: file.path, issue: 'HAR has < 5 entries', severity: 'HIGH' });
        recommendations.push('Request HAR with more activity. Instruct: reproduce issue with DevTools Network tab open.');
      }
    } catch (e) { /* skip */ }
  }

  // Check log files
  for (const file of invContext.evidence_files.filter(f => f.type === 'LOG')) {
    try {
      const lines = await execSync(`wc -l < "${caseDir}/${file.path}"`);
      if (parseInt(lines.trim()) < 10) {
        issues.push({ file: file.path, issue: 'Log has < 10 lines', severity: 'MEDIUM' });
        recommendations.push('Request full log bundle, not excerpts.');
      }
    } catch (e) { /* skip */ }
  }

  // No evidence at all
  if (invContext.evidence_files.length === 0) {
    issues.push({ issue: 'No evidence files found', severity: 'CRITICAL' });
    recommendations.push('Cannot proceed without evidence. Request: log bundle, HAR file, or screenshots.');
  }

  return {
    sufficient: issues.filter(i => i.severity === 'CRITICAL' || i.severity === 'HIGH').length === 0,
    issues,
    recommendations
  };
};
```

---

## CHECKPOINT SUMMARY (NEW in v7.2.0)

Generate verification report for human review before proceeding to deliverables. This is the final quality gate.

**ANNOUNCE**: "Generating verification summary for human review..."

### PTC Workflow for Checkpoint Summary

```javascript
call_tool({
  operation: "workflow",
  template: "custom",
  investigationId: "{case_id}",
  code: `
    const caseDir = load('caseDir') || '{case_directory}';
    const fs = require('fs');

    let invContext = JSON.parse(fs.readFileSync(\`\${caseDir}/inv_context.json\`, 'utf8'));
    const claims = invContext.claims || [];

    if (claims.length === 0) {
      return {
        success: true,
        summary: { total: 0, verified: 0, recommendation: 'NO_CLAIMS' },
        approval_required: true
      };
    }

    // Calculate statistics
    const verified = claims.filter(c => c.verified === true);
    const unverified = claims.filter(c => c.verified === false);
    const total = claims.length;
    const verifiedPercent = Math.round((verified.length / total) * 100);

    // Determine recommendation
    let recommendation, severity;
    if (verifiedPercent >= 90) { recommendation = 'APPROVE'; severity = 'LOW'; }
    else if (verifiedPercent >= 70) { recommendation = 'REVIEW_DISCREPANCIES'; severity = 'MEDIUM'; }
    else if (verifiedPercent >= 50) { recommendation = 'MANUAL_REVIEW_REQUIRED'; severity = 'HIGH'; }
    else { recommendation = 'STOP_INVESTIGATION'; severity = 'CRITICAL'; }

    // Generate markdown report
    let report = \`# Verification Summary\\n\\n\`;
    report += \`**Investigation ID:** \${invContext.investigation_id || 'Unknown'}\\n\`;
    report += \`**Generated:** \${new Date().toISOString()}\\n\\n\`;
    report += \`## Statistics\\n\\n\`;
    report += \`| Metric | Count | Percentage |\\n|--------|-------|------------|\\n\`;
    report += \`| Total Claims | \${total} | 100% |\\n\`;
    report += \`| ✅ Verified | \${verified.length} | \${verifiedPercent}% |\\n\`;
    report += \`| ❌ Unverified | \${unverified.length} | \${100 - verifiedPercent}% |\\n\\n\`;
    report += \`## Recommendation: \${recommendation}\\n**Severity:** \${severity}\\n\\n\`;

    if (unverified.length > 0) {
      report += \`## ❌ Unverified Claims - REQUIRES REVIEW\\n\\n\`;
      unverified.forEach((c, i) => {
        report += \`\${i + 1}. **[\${c.id}]** Claimed: \${c.claimed_value}, Actual: \${c.verified_value}\\n\`;
        report += \`   - Discrepancy: \${c.discrepancy}\\n\`;
        report += \`   - Evidence: \${c.evidence_ref || c.evidence_file}\\n\\n\`;
      });
    }

    // Save report
    fs.writeFileSync(\`\${caseDir}/verification_report.md\`, report);

    // Update context
    invContext.verification_summary = {
      generated_at: new Date().toISOString(),
      total, verified: verified.length, unverified: unverified.length,
      verified_percent: verifiedPercent, recommendation, severity
    };
    fs.writeFileSync(\`\${caseDir}/inv_context.json\`, JSON.stringify(invContext, null, 2));

    return {
      success: true,
      summary: invContext.verification_summary,
      approval_required: recommendation !== 'APPROVE'
    };
  `
})
```

### Fallback Manual Commands

```bash
# Count claims
total=$(jq '.claims | length' "{case_directory}/inv_context.json")

# Count verified claims
verified=$(jq '[.claims[] | select(.verified == true)] | length' "{case_directory}/inv_context.json")

# Count unverified claims
unverified=$(jq '[.claims[] | select(.verified == false)] | length' "{case_directory}/inv_context.json")

# List discrepancies
jq '.claims[] | select(.verified == false) | {id, claimed: .claimed_value, actual: .verified_value, discrepancy}' \
  "{case_directory}/inv_context.json"

# Calculate verification percentage
echo "scale=2; $verified * 100 / $total" | bc
```

### Recommendation Matrix

| Verified % | Recommendation | Severity | Action |
|------------|----------------|----------|--------|
| ≥ 90% | APPROVE | LOW | Proceed to deliverables |
| 70-89% | REVIEW_DISCREPANCIES | MEDIUM | Review and correct unverified claims |
| 50-69% | MANUAL_REVIEW_REQUIRED | HIGH | Manual verification required |
| < 50% | STOP_INVESTIGATION | CRITICAL | Do not proceed, re-analyze |

### After Checkpoint Summary

```
AskUserQuestion({
  questions: [{
    question: `Verification: ${verifiedPercent}% of claims verified. ${unverifiedCount} discrepancies found. Proceed?`,
    header: "Checkpoint",
    options: [
      { label: "Proceed", description: "Accept verified claims, proceed to deliverables" },
      { label: "Review discrepancies", description: "I'll review and correct unverified claims" },
      { label: "Re-analyze", description: "Re-run evidence analysis with corrections" },
      { label: "Stop", description: "Stop investigation, need more evidence" }
    ],
    multiSelect: false
  }]
})
```

---

## Tool Input Best Practices

Tool input validation and best practices are enforced by the `ParameterValidator` PreToolUse hook in `mcp-cappy-dev/src/hooks/builtin/parameter_validator_hook.rs`.

### Key Validations

| Tool | Parameter | Guidance |
|------|-----------|----------|
| `triage_case` | symptom | 10-100 chars recommended. Use concise keywords (3-7 words) for better pattern matching. Verbose inputs often return 0 patterns. |
| `research_topic` | query | Include product name for relevance. |
| `research_topic` | sources | Available: cortex_docs, confluence, jira, patterns, web |
| `validate_solution` | hypothesis | Be specific. Include product, version, and suspected root cause. |

### Discovery

Use `call_tool({ operation: "discover", query: "..." })` to get tool schemas with:
- **bestPractices**: Parameter-specific guidance
- **configuredSources**: Active JIRA projects and Confluence spaces from `~/.cappy/mcp-gateway/user-config.json`

### Hook Architecture

```
mcp-cappy-dev/src/hooks/
├── builtin/
│   ├── parameter_validator_hook.rs  # Validates params, warns on verbose input
│   ├── pii_guard_hook.rs            # Redacts sensitive data
│   ├── phase_gate_hook.rs           # Confidence thresholds (99%/99%)
│   └── ...
└── context.rs                       # Severity enum (Error, Warning, Info)
```

---

## Pattern Enrichment Workflow (v7.6.0)

When an investigation resolves successfully, enrich the pattern database for future cases.

### When to Enrich

After Phase 7 (Deliverables), if:
1. A pattern was matched and confirmed correct
2. Root cause was identified with PRIMARY evidence
3. Solution was validated and worked

### What to Add

For each validated pattern, add `required_evidence` field:

```json
{
  "id": "P123",
  "error_pattern": "...",
  "required_evidence": ["log_bundle", "integration_config", "test_output"],
  ...
}
```

### Evidence Type Reference

| Evidence Key | Description |
|--------------|-------------|
| `har` | HAR file from browser DevTools |
| `console_errors` | Browser console errors |
| `screenshot` | Visual capture of issue |
| `server_version` | Full version string |
| `server_logs` | Log bundle with debug enabled |
| `log_bundle` | Complete tar.gz log bundle |
| `integration_config` | Integration settings (redacted) |
| `test_output` | !test-module or command output |
| `playbook_export` | Playbook JSON/YAML export |
| `work_plan_screenshot` | Work plan with failing task |
| `incident_context` | !getContext output |
| `task_error` | Full task error with stack trace |
| `resource_metrics` | docker stats, kubectl top |
| `incident_count` | Incident volume metrics |

### How to Enrich

1. Edit `mcp-cappy-prod/databases/cappy-cache_latest.json`
2. Find the pattern by ID
3. Add `required_evidence` array
4. Update `metadata.updated_at` timestamp
5. Increment `validation_count`

**Example**:
```bash
# Quick edit with jq (backup first)
jq '.patterns |= map(if .id == "P123" then . + {"required_evidence": ["log_bundle", "server_logs"]} else . end)' \
  cappy-cache_latest.json > temp.json && mv temp.json cappy-cache_latest.json
```

### Curation Reminder

**After every successful investigation**, ask:
- Was a pattern used? → Enrich it
- Was no pattern found but should exist? → Create new pattern
- Was evidence collection difficult? → Add to `required_evidence`

Use `/curator` for pattern database maintenance tasks.

---

## TACO Spot Integration (v7.8.0)

TACO Spot tools provide supplementary guidance from TACO PILOT (Jose Talavera's TAC AI assistant) at key investigation phases.

### Integration Points

| Phase | Trigger | TACO Tool | Purpose |
|-------|---------|-----------|---------|
| 2 (Triage) | Confidence < 70% | `case-helper` | Get TACO PILOT next steps when patterns are weak |
| 4 (Hypothesis) | Unfamiliar topic | `topic-researcher` | Research background on integration/product |
| 7 (Deliverables) | JIRA draft | `jira-aid` | Supplement JIRA with TACO PILOT insights |
| 7 (Deliverables) | Case review | `tac-review` | Generate TAC case review summary |
| Any | Investigation stuck | `case-helper` | Escape hatch for blocked investigations |

### Phase 2: Low Confidence Triage

When `triage_case` returns `overallConfidence < 70`:

```
AskUserQuestion({
  questions: [{
    question: `Triage confidence is ${confidence}% (low). Would you like TACO PILOT guidance?`,
    header: "TACO Spot",
    options: [
      { label: "Yes, get TACO PILOT help", description: "Query TACO PILOT for next steps" },
      { label: "No, proceed with evidence", description: "Continue with manual analysis" },
      { label: "Research topic first", description: "Use topic-researcher for background" }
    ],
    multiSelect: false
  }]
})
```

If user selects "Yes":
```bash
# Auto-detects case from directory, product from env.log
cappy-core taco-spot case-helper
```

### Phase 4: Hypothesis Stuck

When hypothesis generation needs more context on unfamiliar topics:

```bash
# Research a specific topic
cappy-core taco-spot topic-researcher --topic "XSOAR 8.x Docker memory limits"
```

### Phase 7: Deliverables Enhancement

**JIRA Draft Enhancement:**
```bash
# Get TACO PILOT insights for JIRA
cappy-core taco-spot jira-aid
```

Merge TACO PILOT findings with `generate_deliverables` output:
1. Run `generate_deliverables` for structured JIRA content
2. Run `jira-aid` for similar cases and precedent
3. Combine: CAPPY analysis + TACO PILOT historical context

**TAC Case Review:**
```bash
# Generate case review summary
cappy-core taco-spot tac-review --name "Kevin Tan" --followup 2026-02-09
```

### Escape Hatch: Investigation Stuck

At ANY point if investigation is blocked:

```
AskUserQuestion({
  questions: [{
    question: "Investigation seems stuck. Options:",
    header: "Next Steps",
    options: [
      { label: "TACO PILOT guidance", description: "Get case-helper suggestions" },
      { label: "Research topic", description: "Deep dive on specific topic" },
      { label: "Request more evidence", description: "Ask customer for files" },
      { label: "Escalate to team", description: "Get human TAC help" }
    ],
    multiSelect: false
  }]
})
```

### TACO Spot Tool Summary

| Tool | Command | Auto-Detects | Output |
|------|---------|--------------|--------|
| `info` | `cappy-core taco-spot info` | case | **Quick case lookup** (fastest, direct URL) |
| `case-helper` | `cappy-core taco-spot case-helper` | case, product | Next steps, needed items, JIRA recommendation |
| `topic-researcher` | `cappy-core taco-spot topic-researcher --topic "..."` | product | Topic research from TACO PILOT |
| `jira-aid` | `cappy-core taco-spot jira-aid` | case, product | Similar JIRAs, escalation guidance |
| `tac-review` | `cappy-core taco-spot tac-review` | case, name, product, followup | TAC case review summary |
| `kb-helper` | `cappy-core taco-spot kb-helper --topic "..."` | case, product | KB article outline |

### Vision Direct Case Lookup (v7.10.0)

**Fastest method for case research** - opens TACO PILOT Vision directly with case context.

**URL Format**: `${CAPPY_DASHBOARD_URL}/dashboard/{case_number}`

**Examples**:
- `http://localhost:3002/dashboard/CASE-0001`
- `http://localhost:3002/dashboard/CASE-0002`

**Workflow**:

```
╔════════════════════════════════════════════════════════════════════════════╗
║                    VISION DIRECT CASE LOOKUP                               ║
╠════════════════════════════════════════════════════════════════════════════╣
║                                                                            ║
║  1. OPEN URL                                                               ║
║     ${CAPPY_DASHBOARD_URL}/dashboard/{case_number}      ║
║     (Replace {case_number} with SF case number, e.g., DEMO-005)            ║
║                                                                            ║
║  2. AUTHENTICATE                                                           ║
║     Complete Okta login if prompted                                        ║
║                                                                            ║
║  3. REPROCESS IF NEEDED                                                    ║
║     If case shows stale data or "Not processed":                           ║
║     • Click "Reprocess Case" button                                        ║
║     • Wait for processing to complete (may take 30-60 seconds)             ║
║                                                                            ║
║  4. SAVE RESULTS                                                           ║
║     • Wait for page to fully load                                          ║
║     • Press Cmd+S (Mac) or Ctrl+S (Windows)                                ║
║     • Select "Webpage, Complete" or "Webpage, Single File (.mhtml)"        ║
║     • Save to: {case_directory}/evidence/Vision-{case_id}.html             ║
║                                                                            ║
║  5. PARSE SAVED HTML                                                       ║
║     cappy-core taco-spot read {case_dir}/evidence/Vision-*.html            ║
║                                                                            ║
╚════════════════════════════════════════════════════════════════════════════╝
```

**Pattern Database Integration**:

All CAPPY patterns now include Vision URLs in their case references:

```json
{
  "references": {
    "cases": [{
      "case_id": "SF-DEMO-004",
      "url": "http://localhost:3002/dashboard/CASE-0002"
    }]
  }
}
```

When a pattern matches, you can directly open the Vision URL for precedent case research.

**Standard Practice**: Use Vision URLs for quick case context before deep investigation.

### Post-TACO PILOT Submission

After any TACO Spot command that opens browser:

```
╔════════════════════════════════════════════════════════════════════════════╗
║                         TACO PILOT                                         ║
╠════════════════════════════════════════════════════════════════════════════╣
║                                                                            ║
║  FOR FORM-BASED TOOLS (tac-review, kb-helper, case-helper, etc.):          ║
║  1. Review the query in the form                                           ║
║  2. Click the green "Submit to TACO PILOT" button                          ║
║  3. Results open in a NEW TAB                                              ║
║                                                                            ║
║  FOR INFO COMMAND (direct URL):                                            ║
║  • Results display immediately after Okta login                            ║
║                                                                            ║
╠════════════════════════════════════════════════════════════════════════════╣
║                         SAVING RESULTS                                     ║
╠════════════════════════════════════════════════════════════════════════════╣
║                                                                            ║
║  1. Wait for page to fully load                                            ║
║  2. Press Cmd+S (Mac) or Ctrl+S (Windows)                                  ║
║  3. Save to ~/Downloads/ (default filename is fine)                        ║
║  4. Parse with: cappy-core taco-spot read ~/Downloads/Vision*.html         ║
║                                                                            ║
╚════════════════════════════════════════════════════════════════════════════╝
```

### Reading TACO PILOT Results

After user submits and gets results, they can save the HTML and parse it:

```bash
# Parse saved TACO PILOT export
cappy-core taco-spot read ~/Downloads/tacopilot-results.html --format summary
```

Incorporate parsed results into `inv_context.json`:
```json
{
  "taco_pilot": {
    "queried_at": "2026-02-02T11:30:00Z",
    "tool": "case-helper",
    "results": {
      "current_status": "...",
      "next_steps": ["..."],
      "jira_recommendation": "..."
    }
  }
}
```

---

## CAPPY Agent Orchestration

CAPPY Agent is the sole helper and orchestrator for Main Claude during investigation phases 2-7. It handles all orchestration directly without intermediate knowledge modules:

**CAPPY Agent Responsibilities** (Phases 2-7):
- ✅ Invoke `call_tool(operation: "execute", tool: "triage_case", ...)` and interpret results
- ✅ Check hook status after each tool execution (error, requires_review, verifications, success)
- ✅ Manage phase gates (Phase 5 ≥99%, Phase 7 ≥99% + all claims verified)
- ✅ Register and verify claims using ICFP v2.0 protocol in `inv_context.json`
- ✅ Display Scribe v1.0 progress banners between phases
- ✅ Manage checkpoints with Main Claude for user confirmation
- ✅ Handle escalation decisions when gates block
- ✅ Orchestrate parallel evidence analysis for complex cases

**Main Claude Responsibilities** (Phases 0-1 & All Checkpoints):
- ✅ Initialize case directory and extract environment from `env.log`
- ✅ Review CAPPY Agent results at phase boundaries
- ✅ Make checkpoint decisions (proceed, gather more evidence, retry, escalate)
- ✅ Provide Scribe banners and progress updates to user
- ✅ Invoke final deliverable generation when confidence gates pass

**Architecture** (v1.7.0):
```
Main Claude (Phase 0-1, Checkpoints, Final Output)
    ↓
/investigate skill called
    ↓
CAPPY Agent spawned (Phases 2-7 orchestration)
    ├─ Phase 2: call_tool(execute, triage_case, ...)
    │          → Interpret hooks → Register claims → Check gate (≥90%)
    ├─ Phase 3: call_tool(execute, analyze_evidence, ...)
    │          → Interpret hooks → Register claims → Check coverage
    ├─ Phase 4: call_tool(execute, cappy_synthesis, ...)
    │          → call_tool(execute, validate_solution, ...)
    │          → Interpret hooks → Validate hypothesis → Check gate (≥90%)
    ├─ Phase 5-6: call_tool(execute, validate_solution, ...)
    │          → Deep validation → Check gate (≥99%)
    └─ Phase 7: call_tool(execute, generate_deliverables, ...)
               → Verify claims → Generate outputs → Check gate (≥99%)
    ↓
Results returned to Main Claude
    ↓
Main Claude displays deliverables and checkpoint decisions to user
```

**Wiring Required**:
- CAPPY Agent is configured in MCP server settings
- CAPPY Agent has access to all `call_tool` operations
- CAPPY Agent returns structured results to Main Claude with hook status
- Main Claude interprets CAPPY Agent outputs and makes checkpoint decisions

---

## Related Skills

- `/curator` - Pattern database maintenance and update management
- TACO Spot tools - case-helper, jira-aid, tac-review (integrated at phase checkpoints)

---

## Canonical Documentation

| Document | Purpose |
|----------|---------|
| `docs/FUTURE_FEATURES.md` | Feature requests (P-003: GCS, P-004: Feedback System) |
| `docs/BUG_BACKLOG.md` | Report workflow issues and tool bugs discovered during investigations |
| `docs/cookbooks/08-wiring.md` | Component wiring patterns for CAPPY development |

**Reporting Investigation Issues:**

When workflow issues are encountered during investigation (tool failures, pattern mismatches, missing features), document them in `inv_context.json` under `workflow_issues[]` and consider adding to `docs/BUG_BACKLOG.md` if they represent reproducible bugs.

---

**Full Reference**: See CORTEX-CAPPY.md agent documentation for detailed phase documentation.

---

## Phase 6.5: Final Verification Checkpoint (P-007)

**Added**: 2026-02-06
**Purpose**: Autonomous verification that all claims meet 99%+ standard before customer delivery
**Executor**: CAPPY (reads gate.md for verification logic)
**Gate**: VerificationCheckpointHook (≥99% verification rate)

### When to Invoke Phase 6.5

After Phase 6 (solution validation) completes, before starting Phase 7 deliverable generation:

```
Claude: "Phase 6 complete. Ready to verify before deliverables."
  ↓
CAPPY (Phase 7 checkpoint per gate.md)
  ├─ Load inv_context.json["claims"] - all claims from Phases 2-6
  ├─ Calculate verification_rate:
  │   ├─ Count: claims with Phase 3 evidence citations
  │   ├─ Count: total claims registered
  │   └─ Rate = verified / total
  ├─ Check for contradictions (zero tolerance)
  ├─ Trigger VerificationCheckpointHook
  │   ├─ If rate >= 0.99: PASSED → proceed to Phase 7
  │   └─ If rate < 0.99: BLOCKED → recovery options
  └─ Update inv_context.json["verification_checkpoint"]
```

### Verification Checkpoint Output

inv_context.json["verification_checkpoint"]:
```json
{
  "timestamp": "2026-02-06T15:30:00Z",
  "phase": 7,
  "checkpoint_type": "pre_delivery",
  "claims_total": 12,
  "claims_verified": 12,
  "claims_unverified": 0,
  "verification_rate": 1.0,
  "threshold": 0.99,
  "gate_passed": true,
  "unverified_list": [],
  "contradictions": [],
  "message": "Verification rate 100% (12/12 claims). Passes 99% threshold. Ready for Phase 7 delivery.",
  "verified_by": "CAPPY",
  "recovery_options": []
}
```

### Mandatory Citation Validation

**All claims must have citations** in one of 7 acceptable formats:

| Format | Example | When to Use |
|--------|---------|------------|
| **File:Line** | `server.log:4521` | Text files (logs, configs) |
| **HAR:Entry** | `HAR:145` | HAR file entries |
| **JSON Path** | `response.metadata.timeout_ms` | JSON responses |
| **Timestamp** | `2025-12-16T20:51:56.061Z` | Timeline events |
| **Pattern ID** | `P-127` | Pattern database match |
| **JIRA Ref** | `SF-DEMO-005` | JIRA tickets |
| **Environment** | `env.log:version=8.5.0` | env.log metadata |

### Recovery Paths if Verification Fails

If verification_rate < 0.99:

**Option 1: Return to Phase 5 (Additional Research)**
- Target unverified claims from unverified_list
- Search documentation/JIRA/KB for citations
- Add new citations to claims
- Re-run Phase 7 checkpoint
- Expected improvement: +10-30% verification rate

**Option 2: Deliver with Caveats (Document Unverified)**
- Mark each unverified claim in deliverables
- Label as "Based on analysis but not explicitly verified in evidence"
- Customer knows which parts are verified vs assumption
- Proceed to Phase 7 deliverable generation
- Verification rate remains same but documented

**Option 3: Get Kevin Approval (Override Gate)**
- Escalate with verification_rate and unverified_list
- Kevin decides: deliver as-is or request changes
- If approved: proceed to Phase 7
- If rejected: return to Phase 5

### Process Flow

```
Phase 6 Complete
    │
    ├─ CAPPY: Collect all claims (per gate.md)
    ├─ Calculate verification_rate
    ├─ Trigger VerificationCheckpointHook
    │
    └─ Gate Decision:
        │
        ├─ ✓ PASSED (rate >= 0.99)
        │   └─ Proceed to Phase 7: Deliverables
        │
        └─ ✗ BLOCKED (rate < 0.99)
            ├─ Option 1: Phase 5 Return
            │   └─ Additional research for unverified claims
            ├─ Option 2: Delivery with Caveats
            │   └─ Mark unverified, proceed to Phase 7
            └─ Option 3: Kevin Approval
                └─ Escalate for override decision
```

### Implementation Guidelines

1. **Automatic Invocation**: CAPPY runs Phase 6.5 verification automatically after Phase 6
2. **No User Interaction Required**: Checkpoint runs silently unless gate blocks
3. **Clear Reporting**: If blocked, present recovery options with effort estimates
4. **Citation Tracking**: All claims tracked with verification status in inv_context.json
5. **Audit Trail**: All checkpoints logged to inv_context.json["audit_trail"]

### Success Criteria

Phase 6.5 is working correctly when:

✅ All claims from Phases 2-6 are collected
✅ Verification rate calculated correctly
✅ Gate passes if verification_rate >= 0.99 and no contradictions
✅ Gate blocks if verification_rate < 0.99
✅ Recovery options presented (Phase 5 return, caveats, override)
✅ inv_context.json["verification_checkpoint"] populated accurately
✅ Can proceed to Phase 7 on PASSED gate
✅ Cannot proceed to delivery on BLOCKED gate without recovery

### Related Components

- **gate.md**: Phase 7 verification_checkpoint_gate specification
- **CAPPY agent**: Phase 7 verification logic (reads gate.md)
- **VerificationCheckpointHook**: Rust hook enforcing 99% threshold
- **inv_context.json**: verification_checkpoint section (schema below)

### inv_context.json Schema Update

```rust
/// Phase 7 Verification Checkpoint (P-007)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationCheckpoint {
    /// Checkpoint timestamp
    pub timestamp: DateTime<Utc>,

    /// Always 7 for verification checkpoint
    pub phase: u8,

    /// Type: "pre_delivery"
    pub checkpoint_type: String,

    /// Total claims from all phases
    pub claims_total: usize,

    /// Claims with Phase 3 evidence citations
    pub claims_verified: usize,

    /// Claims without citations
    pub claims_unverified: usize,

    /// Verification rate: claims_verified / claims_total
    pub verification_rate: f32,

    /// Gate threshold (0.99 = 99%)
    pub threshold: f32,

    /// Whether gate passed
    pub gate_passed: bool,

    /// List of unverified claims with reasons
    pub unverified_list: Vec<UnverifiedClaim>,

    /// Any contradictions found (should be empty)
    pub contradictions: Vec<String>,

    /// Human-readable message
    pub message: String,

    /// Which agent verified
    pub verified_by: String,

    /// Recovery options if gate blocked
    pub recovery_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnverifiedClaim {
    /// Unique claim ID
    pub id: String,

    /// The assertion
    pub assertion: String,

    /// Why it's unverified
    pub reason: String,
}
```

---



## QUICK REFERENCE: call_tool Tool Names

**Use this reference when invoking tools via `call_tool({ operation: "execute", tool: "...", params: {...} })`**

### Salesforce Case Tools
| Tool Name | Purpose | Example Params |
|-----------|---------|----------------|
| `case-get` | Get case details | `{ case_number: "DEMO-007" }` |
| `case-comments` | Get case comments & Tenant About Info | `{ case_number: "DEMO-007" }` |
| `case-search` | Search cases by keyword | `{ query: "EDL error XSIAM" }` |
| `case-similar` | Find similar cases | `{ case_number: "DEMO-007" }` |
| `case-my` | Get my assigned cases | `{}` |
| `case-field` | Get specific case field | `{ case_number: "DEMO-007", field: "Description" }` |
| `case-private-comment` | Add private comment to case | `{ case_number: "DEMO-007", comment: "..." }` |

### Investigation Tools
| Tool Name | Purpose | Example Params |
|-----------|---------|----------------|
| `triage-case` | Pattern match + similar cases | `{ symptom: "EDL 500 error", product: "XSIAM" }` |
| `analyze-evidence` | Analyze HAR/bundle/logs | `{ harPath: "/path/to/file.har" }` |
| `cappy-synthesis` | AI hypothesis generation | `{ task: "hypothesize", context: {...} }` |
| `validate-solution` | Validate hypothesis | `{ hypothesis: "...", deepResearch: true }` |
| `generate-deliverables` | Generate JIRA/customer response | `{ caseDir: "/path/to/case", deliverables: ["jira_update"] }` |

### Knowledge Tools
| Tool Name | Purpose | Example Params |
|-----------|---------|----------------|
| `pattern-match` | Search pattern database | `{ symptom: "memory leak", product: "XSOAR" }` |
| `confluence-search` | Search Confluence | `{ query: "Broker VM troubleshooting" }` |
| `confluence-get-page` | Get Confluence page | `{ page_id: "120718855" }` |
| `cortex-docs` | Search Cortex documentation | `{ query: "EDL integration" }` |

### Forensics Tools
| Tool Name | Purpose | Example Params |
|-----------|---------|----------------|
| `har-forensics` | Deep HAR analysis | `{ harPath: "/path/to/file.har" }` |
| `log-analytics` | Analyze log files | `{ logPath: "/path/to/server.log" }` |
| `json-utils` | JSON parsing utilities | `{ jsonPath: "/path/to/file.json", query: ".field" }` |

### Utility Tools
| Tool Name | Purpose | Example Params |
|-----------|---------|----------------|
| `smart-viewer` | View images/PDFs/videos | `{ path: "/path/to/file.png" }` |
| `taco-reader` | Parse TACO PILOT HTML exports | `{ htmlPath: "/path/to/Vision.html" }` |

### Tool Discovery
```javascript
// Find tools by capability
call_tool({ operation: "discover", query: "analyze HAR network trace" })

// Returns: tool name, description, inputSchema, examples
```

---

**Version**: 8.4.0-v1.7 | **Updated**: 2026-02-19
