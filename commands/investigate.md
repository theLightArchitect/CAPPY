---
description: Launch 8-phase TAC investigation for Cortex XSOAR/XSIAM/XDR cases
argument-hint: <symptom-or-case-id> [evidence-path]
allowed-tools:
  - mcp__plugin_cappy-tac-toolkit_cappy__call-tool
  - Task
  - AskUserQuestion
  - Read
  - Glob
  - Grep
  - Bash
  - Write
  - Edit
  - WebSearch
  - WebFetch
  - mcp__mcp-gateway__mcp_jira__jira_search
  - mcp__mcp-gateway__mcp_jira__jira_get_issue
  - mcp__mcp-gateway__mcp_confluence__confluence_search
  - mcp__mcp-gateway__mcp_confluence__confluence_get_page
---

# /investigate Command

**Version**: 2.3.0
**Architecture**: Main Claude orchestrates + HITL checkpoints, CAPPY Task agent executes heavy phases
**Full Spec**: Read `skills/investigate/SKILL.md` for complete documentation

### Changelog (2.3.0) - SF-Wide Search (THE BIGGEST ADVANTAGE)
- **NEW**: Check 1 - Salesforce-Wide Search across ALL customers
  - 1a. **Exact error message search** - Same error = same fix (highest value, +40 confidence)
  - 1b. **Symptom keywords search** - Broader net for similar issues
  - 1c. **Recently closed search** - What resolution worked in last 30 days?
  - 1d. **Currently open search** - Who else is working on this? Collaborate!
  - 1e. **Customer history** - Returning issue? Previous context?
- **ENHANCED**: Confidence scoring - SF exact error match now worth +40 points
- **NEW**: For each closed SF match, pull comments to extract resolution
- **NEW**: Collaboration prompt when same issue is currently open elsewhere
- **Source**: Automating what top closers do through experience - they've seen the same issue 50+ times

### Changelog (2.2.0) - Multi-Source Knowledge Search
- Multi-source knowledge search (Pattern DB, JIRA, Confluence, Cortex Docs)
- Cross-verification with confidence scoring
- Engineering escalation paths (Iltiaz pattern)

### Changelog (2.1.0)
- Initial Quick-Win Triage
- "I already know the answer" option
- Close-Loop Language (Step 7.4)

---

## ARCHITECTURE OVERVIEW

```
Main Claude (YOU)
  ├─ Phases 0-1: Direct execution (pre-flight, discovery)
  ├─ ALL Checkpoints: AskUserQuestion at every phase boundary
  ├─ Phases 2-4: Spawn CAPPY Task agent for heavy lifting
  ├─ Phases 5-7: Direct execution (Bug B-002 workaround)
  └─ Fallbacks: If orchestrators fail, use 3-tier fallback
```

**Key Principle**: YOU handle all user interaction. CAPPY does computation. YOU verify and present.

---

## SUB-SKILLS DIRECTORY

CAPPY Task agent reads these sub-skills for guidance. Location: `skills/investigate/sub-skills/`

| Sub-Skill | Purpose | Used In |
|-----------|---------|---------|
| `curator.md` | Claim registration, citation rules, ICFP v2.0 | Phase 2-7 |
| `gate.md` | Phase gate thresholds and recovery options | All phases |
| `sherlock.md` | Hypothesis coherence validation | Phase 4 |
| `recon.md` | Environment/architecture compatibility | Phase 4 |
| `synthesis.md` | Root cause narrative patterns | Phase 4 |
| `validate.md` | Solution validation rules | Phase 5 |
| `escalation.md` | Escalation decision trees | When blocked |
| `initialize.md` | Case setup and context initialization | Phase 0 |
| `logging.md` | inv_context.json update rules | All phases |

**CAPPY Agent Access**: When spawning CAPPY Task agent, the prompt MUST include:
```
SUB-SKILLS (Read these FIRST for guidance):
- skills/investigate/sub-skills/{skill}.md - {purpose}
```

---

## MANDATORY HITL CHECKPOINTS

**After EVERY phase**, you MUST use AskUserQuestion. Never auto-proceed.

---

## PHASE 0: PRE-FLIGHT (Main Claude Direct)

### Step 0.1: Parse Input
```
Input: /investigate {symptom-or-case-id} [evidence-path]

Parse:
- If starts with "SF-" or is 8 digits → Case ID
- If path exists → Evidence directory
- Otherwise → Symptom description
```

### Step 0.2: SF Case Lookup (MANDATORY if case ID provided)

```javascript
// PRIMARY: Salesforce case lookup
call_tool({
  operation: "execute",
  tool: "case-get",
  params: { case_number: "{case_number}" }
})

call_tool({
  operation: "execute",
  tool: "case-comments",
  params: { case_number: "{case_number}" }
})
```

**OPTIONAL: Taco-spot tools** - Use only when:
- User explicitly requests taco-spot data
- SF data is incomplete and user confirms taco-spot usage
- There's a clear opportunity (e.g., need AI-generated case insights)

---

### Step 0.2.1: TOOL FAILURE HANDLING (HITL Required)

**On ANY tool failure, ALWAYS use HITL before fallback:**

```javascript
AskUserQuestion({
  questions: [{
    question: "Tool failure: {tool_name} returned error: {error_message}",
    header: "Tool Failure - Confirm Fallback",
    options: [
      {
        label: "Use Vision Direct fallback",
        description: "Open ${CAPPY_DASHBOARD_URL}/dashboard/{case_number} for manual lookup"
      },
      {
        label: "Use taco-spot tools",
        description: "Fetch case context from taco-spot API instead"
      },
      {
        label: "Retry SF with re-authentication",
        description: "Launch SF browser auth and retry"
      },
      {
        label: "Skip case lookup",
        description: "Proceed with local evidence only"
      },
      {
        label: "Report tool failure",
        description: "Generate bug report and send to #recon_cappy Slack channel"
      }
    ],
    multiSelect: false
  }]
})
```

**Fallback Rationale Display**: When presenting options, include:
- **Vision Direct**: Best for manual inspection, always available on internal network
- **Taco-spot**: Good for cached/indexed data, AI insights, but may have stale data
- **SF Re-auth**: Required if token expired, opens browser for OAuth flow
- **Skip**: Use when local evidence is sufficient

---

### Step 0.2.2: Tool Failure Reporting

**If user selects "Report tool failure":**

Use the `report-tool-failure` tool to send a comprehensive bug report to #recon_cappy:

```javascript
call_tool({
  operation: "execute",
  tool: "report-tool-failure",
  params: {
    toolName: "{tool_name}",
    operation: "{operation}",
    params: {exact_params_as_json},
    errorType: "{error_type}",
    errorMessage: "{error_message}",
    traceback: "{full_traceback_if_available}",
    fallbackUsed: "{fallback_option_selected}",
    workaround: "{description_of_workaround}",
    caseNumber: "{case_number}",
    phase: "Phase 0 - Pre-Flight",
    skill: "/investigate"
  }
})
```

The tool automatically:
- Gets reporter email via gcloud auth (your-org.example.com)
- Generates UUID for bug tracking
- Formats and sends report to #recon_cappy via Slack

**Confirm to user**: "Bug report {bugId} sent to #recon_cappy by {reporterEmail}"

---

### Step 0.2.3: SF Authentication Recovery

**If user selects "Retry SF with re-authentication":**

```bash
sf org login web -a ${SF_ORG_ALIAS} --instance-url https://your-org.my.salesforce.com
```

This opens a browser window for Salesforce login. After authentication completes, retry the SF case lookup.

### Step 0.2.4: QUICK-WIN TRIAGE (Multi-Source Verification)

**PURPOSE**: Before deep investigation, perform automated multi-source search to identify if a DEFINITIVE resolution exists. Implements patterns from top quick-closers (Gregory Durkin, Iltiaz Nagaria, James Voong).

**TIME BUDGET**: 2 minutes maximum. If no definitive resolution found, proceed to full investigation.

**EXECUTION**: Run Checks 1-3 in parallel, then Check 4 (knowledge search), then Check 5 (verification).

---

#### Check 1: Salesforce-Wide Search (THE BIGGEST ADVANTAGE)

**PURPOSE**: Search ALL of Salesforce for similar issues across ALL customers. This is what top closers have built through years of experience - we automate it.

**Execute ALL searches in parallel**:

```javascript
// 1a. EXACT ERROR MESSAGE search (highest value - same error = same fix)
call_tool({
  operation: "execute",
  tool: "case-search",
  params: {
    query: '"{exact_error_message}"',  // Quoted for exact match
    limit: 20
  }
})

// 1b. SYMPTOM KEYWORDS search (broader net)
call_tool({
  operation: "execute",
  tool: "case-search",
  params: {
    query: "{product} {primary_symptom_keyword} {secondary_keyword}",
    limit: 20
  }
})

// 1c. RECENTLY CLOSED with same pattern (what worked?)
call_tool({
  operation: "execute",
  tool: "case-search",
  params: {
    query: "{symptom_keyword} status:Closed closed:last_30_days",
    limit: 20
  }
})

// 1d. CURRENTLY OPEN with same pattern (who else is working on this?)
call_tool({
  operation: "execute",
  tool: "case-search",
  params: {
    query: "{symptom_keyword} status:Open",
    limit: 20
  }
})

// 1e. CUSTOMER HISTORY (Iltiaz pattern: remember returning customers)
call_tool({
  operation: "execute",
  tool: "case-search",
  params: {
    query: "account:{customer_account_id}",
    limit: 10
  }
})
```

**What We Extract From Each Result**:

| Search Type | If Found | Extract |
|-------------|----------|---------|
| Exact error match | HIGH VALUE | Resolution steps, linked JIRA, owner |
| Symptom match (closed) | Resolution that worked | Comments, what fixed it |
| Symptom match (open) | Ongoing work | Owner (collaborate), linked JIRA |
| Customer history | Context | Previous issues, patterns |

**SF-Wide Search Analysis**:

```javascript
// For each matching case, pull comments to find resolution
for (match of sf_matches.filter(m => m.status === "Closed")) {
  call_tool({
    operation: "execute",
    tool: "case-comments",
    params: { case_number: match.case_number }
  })
  // Extract: What was the fix? Was there a JIRA? What workaround?
}
```

**High-Value Signals**:
| Signal | Meaning | Action |
|--------|---------|--------|
| 5+ cases with same error | Known issue | Find the JIRA, provide known fix |
| Recently closed + JIRA linked | Engineering fix available | Reference JIRA, apply fix |
| Multiple open cases | Ongoing investigation | Collaborate with owner |
| Same customer + same symptom | Returning issue | Check what changed |

**If exact error match found with resolution**:
```javascript
AskUserQuestion({
  questions: [{
    question: `Found ${match_count} cases with same error message.

Case ${best_match.case_number} (closed ${days_ago} days ago):
- Customer: ${best_match.customer}
- Resolution: ${resolution_summary}
- JIRA: ${linked_jira || "None"}

Apply this resolution?`,
    header: "SF-Wide Match Found",
    options: [
      { label: "Yes, apply resolution", description: "Use this fix for current case" },
      { label: "Review case details", description: "See full comments from matched case" },
      { label: "Contact case owner", description: "Reach out to ${best_match.owner}" },
      { label: "Different issue", description: "Continue investigation" }
    ]
  }]
})
```

---

#### Check 2: Platform Issue Detection (PARALLEL)

```javascript
// Check TACOPILOT for platform-wide issues (Greg's pattern)
// TACOPILOT often flags ongoing platform issues
call_tool({
  operation: "execute",
  tool: "taco-reader",
  params: {
    case_number: "{case_number}",
    sections: ["platform_status", "similar_cases"]
  }
})
```

**Platform Issue Keywords** (in symptom or case comments):
- "can't access", "unable to login", "down", "500", "503"
- "maintenance", "unavailable", "all users affected"
- "since this morning", "since the upgrade"

**If platform issue detected**:
1. Check status.your-vendor.example.com
2. If resolved: Confirm with customer → Close
3. If ongoing: Set status to "Engineering Engaged", monitor

---

#### Check 3: CS Scope Assessment (PARALLEL)

**CS Scope Indicators** (NOT break-fix):
- "how to", "how can I", "is it possible", "best practice"
- "clarification", "documentation", "training"
- No error messages, no failures, no unexpected behavior

**If CS scope detected**:
1. Provide brief, helpful answer
2. Add: "For further guidance, please follow up with your CSA"
3. Close or redirect

---

#### Check 4: Multi-Source Knowledge Search (SEQUENTIAL)

**CRITICAL**: Search ALL sources for DEFINITIVE patterns with resolution steps and citations.

##### 4a. Pattern Database (DEFINITIVE patterns only)

```javascript
// Search for patterns with confidence_tier = "DEFINITIVE" and has resolution_steps
call_tool({
  operation: "execute",
  tool: "pattern-match",
  params: {
    symptom: "{symptom_summary}",
    product: "{product}",
    confidence_threshold: 85,
    require_resolution: true,
    require_citations: true
  }
})
```

**Pattern must have**:
- `confidence_tier`: "DEFINITIVE" or "STRONG" (≥85%)
- `resolution_steps`: Non-empty array of actionable steps
- `citations`: At least one source (Confluence, JIRA, Cortex Docs)

##### 4b. JIRA Search (Recent resolutions)

```javascript
// Search JIRA for similar issues with resolution
mcp__mcp-gateway__mcp_jira__jira_search({
  jql: 'project = XSUP AND text ~ "{primary_symptom_keyword}" AND status in (Resolved, Closed) AND resolved >= -90d ORDER BY resolved DESC',
  fields: "summary,status,resolution,resolutiondate,assignee,customfield_10100",
  limit: 10
})
```

**Extract from JIRA**:
- Resolution type (Fixed, Workaround, Won't Fix)
- Resolution steps from comments
- Assignee (for escalation path - Iltiaz pattern: know who to contact)

##### 4c. Confluence Search (TAC Playbooks & KBs)

```javascript
// Search Confluence for documented solutions
mcp__mcp-gateway__mcp_confluence__confluence_search({
  query: "{product} {symptom_keyword} troubleshooting",
  limit: 5
})

// If results found, get page content
mcp__mcp-gateway__mcp_confluence__confluence_get_page({
  page_id: "{confluence_page_id}"
})
```

**Extract from Confluence**:
- Root cause explanation
- Step-by-step resolution
- Known limitations or workarounds

##### 4d. Cortex Docs Search (Official documentation)

```javascript
// Search official Cortex documentation
call_tool({
  operation: "execute",
  tool: "cortex-docs",
  params: {
    query: "{product} {feature} {symptom_keyword}",
    product: "{product}"
  }
})
```

**Extract from Cortex Docs**:
- Official configuration guidance
- Known behaviors/limitations
- Recommended troubleshooting steps

##### 4e. Team Recent Resolutions (Team knowledge)

```javascript
// Search cases closed by team in last 14 days with similar symptom
// (James Voong pattern: leverage team's recent wins)
call_tool({
  operation: "execute",
  tool: "case-search",
  params: {
    query: "{symptom_keyword} status:Closed closed:last_14_days team:FS-CORTEX-NAM",
    limit: 10
  }
})
```

---

#### Check 5: Cross-Verification (MANDATORY before solution-first)

**PURPOSE**: Verify discovered resolution matches reported symptoms.

**Verification Matrix**:

| Source | Found | Symptom Match | Resolution Available | Cited |
|--------|-------|---------------|---------------------|-------|
| Pattern DB | Y/N | Y/N | Y/N | Y/N |
| JIRA | Y/N | Y/N | Y/N | Y/N |
| Confluence | Y/N | Y/N | Y/N | Y/N |
| Cortex Docs | Y/N | Y/N | Y/N | Y/N |
| Team Cases | Y/N | Y/N | Y/N | Y/N |

**Confidence Scoring**:
```
confidence = 0

# Each source that matches adds to confidence
if pattern_db_match: confidence += 30
if jira_match: confidence += 25
if confluence_match: confidence += 20
if cortex_docs_match: confidence += 15
if team_case_match: confidence += 10

# Verification bonuses
if symptom_exact_match: confidence += 10
if error_message_exact_match: confidence += 15
if multiple_sources_agree: confidence += 10
```

**Confidence Thresholds**:
| Score | Tier | Action |
|-------|------|--------|
| ≥85% | DEFINITIVE | Solution-first, skip deep investigation |
| 70-84% | STRONG | Solution-first with verification ask |
| 50-69% | MODERATE | Present findings, user decides |
| <50% | LOW | Proceed to full investigation |

---

#### Check 6: Solution-First Decision Point

**If confidence ≥70%**:

```javascript
AskUserQuestion({
  questions: [{
    question: `I found a ${confidence_tier} match (${confidence}%) from ${sources_count} sources.

Pattern: ${pattern_name}
Sources: ${source_list}

Symptom Match Verification:
- Reported: "${customer_symptom}"
- Pattern: "${pattern_symptom}"
- Match: ${symptom_match_percent}%

Resolution Steps:
${resolution_steps_summary}

Provide solution now?`,
    header: "Quick-Win Resolution Found",
    options: [
      { label: "Yes, provide solution now", description: "Send resolution to customer, close loop" },
      { label: "Review sources first", description: "Let me see the source documents" },
      { label: "Verify with evidence", description: "Check customer's evidence against resolution" },
      { label: "Pattern doesn't match", description: "Continue full investigation" }
    ]
  }]
})
```

**If user selects "provide solution now"**:
1. Generate customer response with:
   - Root cause explanation (cited)
   - Step-by-step resolution
   - Verification steps for customer
   - Close-loop language: "Let me know if ok to close, or if issue persists"
2. Log resolution source to inv_context.json
3. Skip to Phase 7 for closure

**If user selects "Review sources first"**:
- Display JIRA ticket content
- Display Confluence page content
- Display Pattern DB entry
- Return to decision point

---

#### Check 7: Engineering Escalation Path (Iltiaz Pattern)

**If resolution requires Engineering**:

```javascript
// Identify owner for this issue type (Iltiaz's network approach)
const escalation_paths = {
  "Podman/Engine": ["Srinivas Karavadi", "Jon Crawford"],
  "SLS/Logging": ["Adi Daud", "Hamid Mahvidi"],
  "XSOAR Jobs": ["XSOAR Backend Team"],
  "XSIAM Performance": ["XSIAM Platform Team"],
  "Agent Issues": ["XDR Agent Team"]
};

// Find existing JIRA from team member
mcp__mcp-gateway__mcp_jira__jira_search({
  jql: `project = XSUP AND text ~ "{symptom}" AND assignee in (${team_members}) AND status != Closed`,
  limit: 5
})
```

**If existing Engineering engagement found**:
- Link to existing JIRA
- Provide update from Engineering
- Close SF case if customer can track via JIRA

---

#### Quick-Win Triage Summary

| Check | Source | Time | Outcome if Positive |
|-------|--------|------|---------------------|
| **1. SF-WIDE SEARCH** | **ALL of Salesforce** | **30s** | **HIGHEST VALUE - same error = same fix** |
| 1a. Exact error match | SF full-text search | 10s | Direct resolution from precedent |
| 1b. Symptom keywords | SF keyword search | 10s | Similar cases, linked JIRAs |
| 1c. Recently closed | SF closed last 30d | 5s | What resolution worked? |
| 1d. Currently open | SF open cases | 5s | Who's working on this? Collaborate |
| 1e. Customer history | SF account search | 5s | Returning issue? Context |
| 2. Platform Issue | TACOPILOT + Status Page | 10s | Confirm + close |
| 3. CS Scope | Symptom Analysis | 5s | Answer + redirect |
| 4a. Pattern DB | CAPPY Pattern Match | 10s | DEFINITIVE pattern with citations |
| 4b. JIRA | JIRA Search (resolved) | 10s | Engineering resolution |
| 4c. Confluence | Confluence Search | 10s | TAC Playbook steps |
| 4d. Cortex Docs | cortex-docs tool | 10s | Official guidance |
| 5. Verification | Cross-reference all | 15s | Confidence scoring |
| 6. Decision | User HITL | - | Solution-first or continue |
| 7. Eng Path | JIRA assignee lookup | 5s | Direct escalation contact |

**Total Time Budget**: 2 minutes maximum

**Confidence Scoring** (SF-Wide matches have highest weight):
| Source | Points | Rationale |
|--------|--------|-----------|
| SF exact error match (closed) | +40 | Same error + resolution = definitive |
| SF exact error match (open) | +25 | Ongoing work, collaborate |
| SF symptom match (closed) | +20 | Similar issue solved |
| JIRA match (resolved) | +20 | Engineering fix available |
| Pattern DB (DEFINITIVE) | +20 | Verified pattern with citations |
| Confluence match | +15 | Documented playbook |
| Cortex Docs match | +10 | Official guidance |
| Multiple sources agree | +15 | Cross-validated |

**Confidence Thresholds**:
- ≥85%: DEFINITIVE → Solution-first, skip investigation
- 70-84%: STRONG → Solution-first with verification
- 50-69%: MODERATE → User decides
- <50%: LOW → Full investigation

**If confidence <50%**: Proceed to Step 0.3 (standard investigation path).

---

#### Quick-Win Triage Logging

Log all quick-win checks to inv_context.json:
```json
{
  "quick_win_triage": {
    "timestamp": "ISO8601",
    "time_elapsed_ms": 45000,
    "sf_wide_search": {
      "exact_error_matches": {
        "count": 5,
        "closed": 3,
        "open": 2,
        "best_match": {
          "case_number": "03923935",
          "customer": "Other Corp",
          "owner": "Gregory Durkin",
          "resolution": "Podman lingering fix",
          "linked_jira": "XSUP-63638"
        }
      },
      "symptom_matches": {
        "count": 12,
        "closed_with_resolution": 8,
        "open_active": 4
      },
      "customer_history": {
        "previous_cases": 3,
        "related_to_current": 1
      }
    },
    "other_sources": {
      "platform_issue": { "found": false },
      "cs_scope": { "detected": false },
      "pattern_db": { "matches": 2, "top_confidence": 75 },
      "jira": { "matches": 3, "resolved_count": 2 },
      "confluence": { "matches": 1, "page_ids": ["12345"] }
    },
    "verification": {
      "symptom_match_percent": 85,
      "error_match_exact": true,
      "sources_agreeing": 4
    },
    "final_confidence": 88,
    "tier": "DEFINITIVE",
    "decision": "solution_first",
    "resolution_source": "SF:03923935 + JIRA:XSUP-63638",
    "resolution_applied": "Podman lingering fix from case 03923935"
  }
}
```

---

### Step 0.3: Locate/Create Case Directory
```bash
case_dir="$HOME/Desktop/Investigations/01_Cases/00_ACTIVE/SF-{case_number}"
mkdir -p "$case_dir"/{evidence,extracted,analysis,deliverables}
```

### Step 0.4: List Evidence
```bash
ls -la "$case_dir"
ls -la "$case_dir/evidence" 2>/dev/null
```

### Step 0.5: Extract Bundles & Detect Environment
```bash
# Extract bundles
for bundle in "$case_dir"/*.tar.gz "$case_dir/evidence"/*.tar.gz; do
  [ -f "$bundle" ] && tar -xzf "$bundle" -C "$case_dir/extracted/" 2>&1
done

# Find and parse env.log
env_log=$(find "$case_dir" -name "env.log" -type f 2>/dev/null | head -1)
if [ -n "$env_log" ]; then
  grep -E "^(Product|Version|Build|Server Name):" "$env_log"
fi
```

### Step 0.6: CHECKPOINT - Context Confirmation

```javascript
AskUserQuestion({
  questions: [{
    question: "Pre-flight complete. Product: {product}, Version: {version}. Found {N} evidence files. Proceed?",
    header: "Pre-Flight",
    options: [
      { label: "Correct, proceed", description: "Start investigation" },
      { label: "Wrong product/version", description: "I'll provide corrections" },
      { label: "Missing files", description: "I have more evidence" },
      { label: "Show me what you found", description: "List all details" }
    ],
    multiSelect: false
  }]
})
```

**WAIT for response. Do NOT auto-proceed.**

---

## PHASE 1: DISCOVERY (Main Claude Direct)

### Step 1.1: Identify Evidence Files
For each file, categorize:
- `.har` → HAR network trace
- `.tar.gz` → Log bundle
- `.png/.jpg` → Screenshot
- `.json` → Data export/config
- `.log` → Raw log file

### Step 1.2: CHECKPOINT - File Selection

```javascript
AskUserQuestion({
  questions: [{
    question: "Found {N} evidence files: {file_summary}. Which should I analyze?",
    header: "Evidence",
    options: [
      { label: "All files", description: "Analyze everything" },
      { label: "Let me specify", description: "I'll tell you which files" },
      { label: "Need context first", description: "Describe the issue to me" },
      { label: "Add more files", description: "I have additional evidence" },
      { label: "I already know the answer", description: "Skip to solution - I recognize this pattern" }
    ],
    multiSelect: false
  }]
})
```

**If user selects "I already know the answer"**:
1. User provides the known solution/pattern
2. Skip to Phase 7 (Deliverables) to generate customer response
3. Include standard verification steps for customer
4. Close loop: "Let me know if ok to close, or if issue persists"

**WAIT for response. Do NOT auto-proceed.**

---

## PHASE 2: TRIAGE (CAPPY Task Agent OR Fallback)

### Primary: Spawn CAPPY Task Agent

```javascript
Task({
  subagent_type: "cappy-tac-toolkit:CAPPY",
  description: "Phase 2 Triage",
  prompt: `Execute Phase 2 ONLY.

INPUTS:
- Symptom: {symptom}
- Product: {product}
- Version: {version}
- Evidence files: {file_list}

SUB-SKILLS (Read these FIRST for guidance):
- skills/investigate/sub-skills/curator.md - Claim registration rules
- skills/investigate/sub-skills/gate.md - Gate thresholds (Phase 2: ≥90%)

EXECUTE:
1. call_tool({ operation: "execute", tool: "triage_case", params: { symptom: "{symptom}", product: "{product}", severity: "{severity}" }})
2. Register all claims to inv_context.json following curator.md rules
3. Check confidence gate following gate.md (threshold: ≥90%)

RETURN (structured JSON):
{
  "phase": 2,
  "gate_status": "PASS|FAIL",
  "confidence_score": N,
  "threshold": 90,
  "patterns_found": [...],
  "jira_cases": [...],
  "claims_registered": N,
  "recommendation": "..."
}

Do NOT proceed to Phase 3. Return to Main Claude.`
})
```

### FALLBACK: If CAPPY Fails or Orchestrator Fails

**Fallback Tier 1**: Use individual MCP tools directly:
```javascript
// Step 1: Pattern search
call_tool({
  operation: "execute",
  tool: "pattern-match",
  params: { symptom: "{symptom}", product: "{product}" }
})

// Step 2: Similar case search
call_tool({
  operation: "execute",
  tool: "case-search",
  params: { query: "{symptom}", product: "{product}", limit: 5 }
})

// Step 3: JIRA search
call_tool({
  operation: "execute",
  tool: "jira-search",
  params: { query: "{symptom} {product}", project: "XSUP", limit: 5 }
})
```

**Fallback Tier 2**: Use MCP Gateway tools:
```javascript
// JIRA search via gateway
mcp__mcp-gateway__mcp_jira__jira_search({
  jql: "project = XSUP AND text ~ \"{symptom}\" AND component = \"{product}\"",
  fields: "summary,status,resolution,description",
  limit: 10
})

// Confluence search via gateway
mcp__mcp-gateway__mcp_confluence__confluence_search({
  query: "{symptom} {product}",
  limit: 10
})
```

**Fallback Tier 3**: Native tools (no MCP required):

**Pattern Database Search**:
```bash
# Search by symptom keywords (case-insensitive)
jq --arg symptom "{symptom}" '
  .patterns[] |
  select(
    (.symptom_keywords | any(test($symptom; "i"))) or
    (.title | test($symptom; "i")) or
    (.description | test($symptom; "i"))
  ) |
  {
    id: .pattern_id,
    title: .title,
    product: .product,
    confidence: .confidence_tier,
    keywords: .symptom_keywords,
    solution: .solution_summary
  }
' ~/.claude/tools/cappy-cache_latest.json | head -100

# Filter by product
jq --arg product "{product}" '
  .patterns[] |
  select(.product == $product) |
  {id: .pattern_id, title: .title, confidence: .confidence_tier}
' ~/.claude/tools/cappy-cache_latest.json | head -50

# Get full pattern details by ID
jq --arg pid "{pattern_id}" '
  .patterns[] | select(.pattern_id == $pid)
' ~/.claude/tools/cappy-cache_latest.json
```

**Evidence Error Search**:
```bash
# Search logs for common error patterns
grep -rni "error\|exception\|failed\|timeout\|refused\|denied" "$case_dir/extracted/" | head -100

# Search HAR for HTTP errors
jq '.log.entries[] | select(.response.status >= 400) | {url: .request.url, status: .response.status, time: .startedDateTime}' "$har_file" | head -50

# Search for specific error codes
grep -rni "HTTP 4\|HTTP 5\|status.*[45][0-9][0-9]" "$case_dir/" | head -50
```

**Native Confidence Calculation**:
```
| Factor | Points |
|--------|--------|
| Exact pattern keyword match | +30 |
| Partial pattern match | +15 |
| Multiple patterns match | +10 |
| Evidence errors correlate with pattern | +20 |
| Product/version exact match | +10 |
| JIRA precedent found | +15 |

Total >= 90% = PASS gate
```

**LOG FALLBACK** to inv_context.json:
```json
{
  "tool_usage": {
    "toolFallbacks": [{
      "timestamp": "ISO8601",
      "phase": 2,
      "toolAttempted": "triage_case",
      "failureReason": "{reason}",
      "manualCommandsUsed": ["{commands}"],
      "result": "{what was found}"
    }]
  }
}
```

### CHECKPOINT - Triage Results

```javascript
AskUserQuestion({
  questions: [{
    question: "Triage: {confidence}% confidence. Top match: '{pattern}'. Similar cases: {jira_list}. How to proceed?",
    header: "Triage",
    options: [
      { label: "Analyze evidence (Recommended)", description: "Deep dive into files" },
      { label: "Known issue - skip to solution", description: "Pattern is definitive" },
      { label: "Get TACO PILOT guidance", description: "Low confidence, need help" },
      { label: "Wrong direction", description: "Pattern doesn't fit" },
      { label: "Need more context", description: "Explain the matches" }
    ],
    multiSelect: false
  }]
})
```

**Low Confidence Escape Hatch** (if confidence < 90%):
```bash
# TACO PILOT case helper
open "${CAPPY_DASHBOARD_URL}/dashboard/{case_number}"
```

**WAIT for response. Do NOT auto-proceed.**

---

## PHASE 3: EVIDENCE ANALYSIS (CAPPY Task Agent OR Fallback)

### Primary: Spawn CAPPY Task Agent

```javascript
Task({
  subagent_type: "cappy-tac-toolkit:CAPPY",
  description: "Phase 3 Evidence",
  prompt: `Execute Phase 3 ONLY.

INPUTS:
- Evidence files: {file_list}
- Analysis depth: {depth}
- Pattern context: {pattern_from_phase_2}

SUB-SKILLS (Read these FIRST for guidance):
- skills/investigate/sub-skills/curator.md - Claim registration with citations
- skills/investigate/sub-skills/gate.md - Gate thresholds (Phase 3: ≥99%)

EXECUTE:
1. call_tool({ operation: "execute", tool: "analyze_evidence", params: { paths: [{file_list}], depth: "{depth}" }})
2. Extract: errors, timeline, key events
3. Register all claims with file:location citations following curator.md
4. Check completeness gate following gate.md (threshold: ≥99%)

RETURN (structured JSON):
{
  "phase": 3,
  "gate_status": "PASS|FAIL",
  "completeness_score": N,
  "threshold": 99,
  "errors_found": [...],
  "timeline_events": [...],
  "key_findings": [...],
  "claims_registered": N,
  "uncited_claims": N,
  "recommendation": "..."
}

Do NOT proceed to Phase 4. Return to Main Claude.`
})
```

### FALLBACK: If CAPPY Fails or analyze_evidence Fails

Main Claude handles evidence analysis manually using native tools:

---

**HAR Analysis (Comprehensive)**:

```bash
# 1. Entry count and basic stats
jq '.log.entries | length' "$har_file"

# 2. Status code distribution
jq '[.log.entries[].response.status] | group_by(.) | map({status: .[0], count: length}) | sort_by(-.count)' "$har_file"

# 3. Find all error responses (4xx, 5xx)
jq '.log.entries[] | select(.response.status >= 400) | {
  index: (input_line_number - 1),
  url: .request.url,
  method: .request.method,
  status: .response.status,
  statusText: .response.statusText,
  time: .startedDateTime,
  duration_ms: .time
}' "$har_file"

# 4. Find slow requests (>5 seconds)
jq '.log.entries[] | select(.time > 5000) | {
  url: .request.url,
  duration_ms: .time,
  status: .response.status,
  time: .startedDateTime
}' "$har_file"

# 5. Extract response bodies for errors (for error messages)
jq '.log.entries[] | select(.response.status >= 400) | {
  url: .request.url,
  status: .response.status,
  body: (.response.content.text // "no body")
}' "$har_file" | head -200

# 6. Find specific API endpoints
jq --arg endpoint "{endpoint}" '
  .log.entries[] | select(.request.url | contains($endpoint))
' "$har_file"

# 7. Timeline extraction (sorted)
jq '[.log.entries[] | {time: .startedDateTime, url: .request.url, status: .response.status}] | sort_by(.time)' "$har_file"

# 8. Request headers analysis
jq '.log.entries[0].request.headers' "$har_file"

# 9. Find authentication issues
jq '.log.entries[] | select(.response.status == 401 or .response.status == 403) | {url: .request.url, status: .response.status}' "$har_file"

# 10. Count entries by status code
jq '[.log.entries[].response.status] | group_by(.) | map({(.[0]|tostring): length}) | add' "$har_file"
```

---

**Log Bundle Analysis (Comprehensive)**:

```bash
# 1. Find errors with line numbers and context
grep -rn "error\|ERROR\|exception\|Exception\|CRITICAL\|FATAL" "$case_dir/extracted/" | head -100

# 2. Find errors with surrounding context (-B before, -A after)
grep -rn -B 2 -A 5 "error\|exception" "$case_dir/extracted/"*.log | head -200

# 3. Extract timestamps for timeline
grep -oP "\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}" "$case_dir/extracted/"*.log | sort -u | head -50

# 4. Find stack traces
grep -rn -A 10 "Traceback\|at .*\(.*:\d\+\)\|Exception:" "$case_dir/extracted/" | head -200

# 5. Search specific log files
grep -n "{pattern}" "$case_dir/extracted/app-server.log" | head -50

# 6. Find connection/timeout issues
grep -rni "timeout\|connection refused\|connection reset\|ETIMEDOUT\|ECONNREFUSED" "$case_dir/extracted/" | head -50

# 7. Find authentication/permission issues
grep -rni "unauthorized\|forbidden\|permission denied\|access denied\|401\|403" "$case_dir/extracted/" | head -50

# 8. Extract unique error types
grep -rhi "error" "$case_dir/extracted/"*.log | sort -u | head -50

# 9. Find configuration issues
grep -rni "config\|setting\|parameter\|invalid" "$case_dir/extracted/" | grep -i "error\|fail\|invalid" | head -50

# 10. Memory/resource issues
grep -rni "out of memory\|OOM\|heap\|memory limit\|resource exhausted" "$case_dir/extracted/" | head -50
```

---

**Screenshot Analysis** (Claude vision):
```javascript
// Read screenshot - Claude interprets visually
Read({ file_path: "{screenshot_path}" })

// Describe what you see:
// - Error messages visible
// - UI state (buttons, fields, values)
// - Configuration shown
// - Any warning/error indicators
```

---

**JSON/Config Analysis**:
```bash
# Validate JSON
jq '.' "$json_file" > /dev/null && echo "Valid JSON" || echo "Invalid JSON"

# Extract specific fields
jq '.{path}' "$json_file"

# Pretty print with line numbers (for citations)
jq -C '.' "$json_file" | cat -n

# Search for specific values
jq '.. | objects | select(.key == "value")' "$json_file"
```

---

**Timeline Correlation**:
```bash
# Merge HAR timestamps with log timestamps
echo "=== HAR Timeline ===" && jq -r '.log.entries[] | .startedDateTime' "$har_file" | sort
echo "=== Log Timeline ===" && grep -oP "\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}" "$case_dir/extracted/"*.log | sort -u

# Find events in same time window (±5 seconds)
# Manual correlation: compare timestamps from both sources
```

### VERIFY - After CAPPY or Manual Analysis

```bash
# Verify claim: "Found N HTTP 500 errors"
jq '[.log.entries[] | select(.response.status == 500)] | length' "$har_file"

# Verify claim: "Error X at line Y"
sed -n '{Y}p' "$log_file"
```

### CHECKPOINT - Evidence Findings

```javascript
AskUserQuestion({
  questions: [{
    question: "Evidence: Found {N} errors, {M} key events. Verified against files. Proceed?",
    header: "Evidence",
    options: [
      { label: "Generate hypothesis", description: "Create root cause theory" },
      { label: "Show details", description: "Review all findings" },
      { label: "Something's wrong", description: "Findings don't match my view" },
      { label: "Analyze more", description: "Additional evidence exists" }
    ],
    multiSelect: false
  }]
})
```

**WAIT for response. Do NOT auto-proceed.**

---

## PHASE 4: HYPOTHESIS (CAPPY Task Agent OR Fallback)

### Primary: Spawn CAPPY Task Agent

```javascript
Task({
  subagent_type: "cappy-tac-toolkit:CAPPY",
  description: "Phase 4 Hypothesis",
  prompt: `Execute Phase 4 ONLY.

INPUTS:
- Evidence findings: {evidence_summary}
- Pattern matches: {patterns}
- Product: {product}, Version: {version}

SUB-SKILLS (Read these FIRST for guidance):
- skills/investigate/sub-skills/sherlock.md - Hypothesis coherence validation
- skills/investigate/sub-skills/recon.md - Environment compatibility check
- skills/investigate/sub-skills/gate.md - Gate thresholds (Phase 4: ≥90%)

EXECUTE:
1. call_tool({ operation: "execute", tool: "cappy_synthesis", params: { task: "hypothesize", evidence: {evidence_summary} }})
2. Validate hypothesis coherence following sherlock.md rules
3. Check environment compatibility following recon.md
4. Check coherence gate following gate.md (threshold: ≥90%)

RETURN (structured JSON):
{
  "phase": 4,
  "gate_status": "PASS|FAIL",
  "coherence_score": N,
  "threshold": 90,
  "hypothesis": "...",
  "confidence_tier": "DEFINITIVE|STRONG|MODERATE|WEAK",
  "supporting_evidence": [...],
  "contradictions": [...],
  "weak_assumptions": [...],
  "recommendation": "..."
}

Do NOT proceed to Phase 5. Return to Main Claude.`
})
```

### FALLBACK: Manual Hypothesis Generation

**Fallback Tier 1**: Use individual MCP tools:
```javascript
// Research similar issues
call_tool({
  operation: "execute",
  tool: "cortex-docs",
  params: { query: "{symptom}", product: "{product}" }
})

// Search JIRA for precedent
call_tool({
  operation: "execute",
  tool: "jira-search",
  params: { query: "{error_message}", project: "XSUP" }
})
```

**Fallback Tier 2**: Use MCP Gateway + WebSearch:
```javascript
// JIRA search via gateway
mcp__mcp-gateway__mcp_jira__jira_search({
  jql: "project = XSUP AND text ~ \"{error_message}\"",
  fields: "summary,status,resolution,description,comment",
  limit: 10
})

// Confluence search via gateway
mcp__mcp-gateway__mcp_confluence__confluence_search({
  query: "{symptom} {product} troubleshooting",
  limit: 10
})

// Get specific Confluence page
mcp__mcp-gateway__mcp_confluence__confluence_get_page({
  page_id: "{page_id}",
  include_metadata: true,
  convert_to_markdown: true
})
```

**Fallback Tier 3**: WebSearch for documentation:
```javascript
// Search Cortex documentation
WebSearch({
  query: "{symptom} {product} site:docs.your-vendor.example.com 2026",
  allowed_domains: ["docs.your-vendor.example.com"]
})

// Fetch specific doc page
WebFetch({
  url: "{doc_url}",
  prompt: "Extract: 1) Known issues related to {symptom}, 2) Requirements/prerequisites, 3) Troubleshooting steps, 4) Error codes and meanings"
})

// Search for known bugs/limitations
WebSearch({
  query: "{product} {error_message} known issue OR bug OR limitation"
})

// Search Live community
WebSearch({
  query: "{symptom} {product} site:community.your-vendor.example.com",
  allowed_domains: ["community.your-vendor.example.com"]
})
```

---

**Manual Synthesis Process** (Claude reasoning):

1. **List Observed Symptoms**:
   - Primary symptom from customer
   - Errors found in HAR (HTTP status, error messages)
   - Errors found in logs (exceptions, failures)
   - Visual issues from screenshots

2. **Identify Common Thread**:
   - Do errors share a timestamp? (timing issue)
   - Do errors share an endpoint? (API issue)
   - Do errors share an error code? (specific bug)
   - Do errors share a component? (subsystem issue)

3. **Match Against Pattern Database**:
   ```bash
   # Get pattern details for best match
   jq --arg pid "{pattern_id}" '
     .patterns[] | select(.pattern_id == $pid) |
     {
       root_cause: .root_cause,
       solution: .solution_summary,
       jira_refs: .jira_references,
       verification_steps: .verification_steps
     }
   ' ~/.claude/tools/cappy-cache_latest.json
   ```

4. **Formulate Hypothesis**:
   ```
   ROOT CAUSE: [Clear statement of the root cause]

   EVIDENCE CHAIN:
   - [Evidence 1] [file:line citation]
   - [Evidence 2] [file:entry citation]
   - [Evidence 3] [documentation reference]

   CONFIDENCE: [X]% ([TIER])

   KEY ASSUMPTIONS (to verify in Phase 5):
   - [Assumption 1] - verification method
   - [Assumption 2] - verification method
   ```

5. **Check for Contradictions**:
   - Does any evidence contradict the hypothesis?
   - Are there timeline mismatches?
   - Does the hypothesis fit the product/version?

---

**Hypothesis Coherence Check** (per sherlock.md):
```
| Check | Pass Criteria |
|-------|---------------|
| Evidence alignment | All claims supported by Phase 3 findings |
| No contradictions | No evidence conflicts with hypothesis |
| Assumptions marked | Unverified claims clearly identified |
| Timeline matches | Event sequence makes logical sense |
| Architecture fits | Hypothesis fits customer's setup |

Coherence score >= 90% required for Phase 4 gate
```

### CHECKPOINT - Hypothesis Review

```javascript
AskUserQuestion({
  questions: [{
    question: "Hypothesis: '{root_cause}' ({confidence}% - {tier}). Evidence: {citations}. Accept?",
    header: "Root Cause",
    options: [
      { label: "Accept, validate (Recommended)", description: "Research and confirm" },
      { label: "Refine", description: "Adjust based on my input" },
      { label: "Alternative", description: "Consider different cause" },
      { label: "Need more evidence", description: "Insufficient data" },
      { label: "Research topic", description: "Use topic-researcher" }
    ],
    multiSelect: false
  }]
})
```

**WAIT for response. Do NOT auto-proceed.**

---

## PHASES 5-6: VALIDATION + SOLUTION (Main Claude Direct)

**Note**: Bug B-002 workaround - do NOT spawn CAPPY Task agent for these phases.

### Step 5.1: Validate Solution

```javascript
call_tool({
  operation: "execute",
  tool: "validate_solution",
  params: {
    hypothesis: "{hypothesis}",
    product: "{product}",
    version: "{version}",
    evidence_summary: "{evidence}"
  }
})
```

**FALLBACK** if validate_solution fails:

**Fallback Tier 1**: Use individual MCP tools:
```javascript
// Research documentation
call_tool({
  operation: "execute",
  tool: "cortex-docs",
  params: { query: "{hypothesis_topic}", product: "{product}" }
})

// Check Confluence
call_tool({
  operation: "execute",
  tool: "confluence-search",
  params: { query: "{symptom} {product}" }
})

// Check JIRA precedent
call_tool({
  operation: "execute",
  tool: "jira-search",
  params: { query: "{root_cause}", project: "XSUP" }
})
```

**Fallback Tier 2**: Use MCP Gateway:
```javascript
// JIRA search for similar resolved issues
mcp__mcp-gateway__mcp_jira__jira_search({
  jql: "project = XSUP AND text ~ \"{root_cause}\" AND status = Resolved",
  fields: "summary,status,resolution,description,comment",
  limit: 10
})

// Get specific JIRA issue details
mcp__mcp-gateway__mcp_jira__jira_get_issue({
  issue_key: "{XSUP-XXXXX}",
  fields: "*all",
  comment_limit: 20
})

// Confluence knowledge base search
mcp__mcp-gateway__mcp_confluence__confluence_search({
  query: "{root_cause} {product}",
  limit: 10
})

// Get specific Confluence article
mcp__mcp-gateway__mcp_confluence__confluence_get_page({
  page_id: "{page_id}",
  convert_to_markdown: true
})
```

**Fallback Tier 3**: Native WebSearch/WebFetch:
```javascript
// Search official documentation
WebSearch({
  query: "{hypothesis_topic} {product} site:docs.your-vendor.example.com 2026"
})

// Fetch and extract from doc page
WebFetch({
  url: "{doc_url}",
  prompt: "Verify: Does this documentation support the hypothesis '{root_cause}'? Extract: requirements, known issues, workarounds."
})

// Search for known bugs
WebSearch({
  query: "{product} {error_code} bug OR issue OR fix"
})

// Search release notes for fixes
WebSearch({
  query: "{product} release notes {version} fix {symptom}"
})

// Search community for solutions
WebSearch({
  query: "{symptom} {product} solution site:community.your-vendor.example.com"
})
```

---

**Manual Validation Process**:

1. **Verify Against Evidence** (re-check Phase 3):
   ```bash
   # Re-verify key claims against files
   jq '.log.entries[{N}]' "$har_file"  # Verify HAR entry
   sed -n '{N}p' "$log_file"           # Verify log line
   ```

2. **Cross-Reference Documentation**:
   - Does official docs describe this behavior?
   - Is there a known limitation?
   - What are the prerequisites?

3. **Check JIRA Precedent**:
   - Similar issues resolved how?
   - Was it a bug fix or configuration?
   - What version was it fixed in?

4. **Validate Assumptions** (from Phase 4):
   ```
   | Assumption | Verification Method | Result |
   |------------|---------------------|--------|
   | [Assumption 1] | [Search docs/JIRA] | VERIFIED/UNVERIFIED |
   | [Assumption 2] | [Check evidence] | VERIFIED/UNVERIFIED |
   ```

5. **Calculate Final Confidence**:
   ```
   | Factor | Weight | Score |
   |--------|--------|-------|
   | Evidence alignment | 40% | [0-100] |
   | Documentation support | 25% | [0-100] |
   | JIRA precedent | 20% | [0-100] |
   | No contradictions | 15% | [0-100] |

   Final = (Evidence × 0.4) + (Docs × 0.25) + (JIRA × 0.2) + (NoContradictions × 0.15)

   Gate: >= 99% for Phase 5
   ```

### Step 5.2: Check Confidence Gate (≥99%)

If confidence < 99%:
```javascript
AskUserQuestion({
  questions: [{
    question: "Validation confidence {confidence}% is below 99% threshold. Options:",
    header: "Low Confidence",
    options: [
      { label: "Research more", description: "Find additional validation" },
      { label: "Proceed with caveats", description: "Mark as uncertain" },
      { label: "Request override", description: "Kevin approval" },
      { label: "Re-analyze", description: "Go back to evidence" }
    ],
    multiSelect: false
  }]
})
```

### CHECKPOINT - Solution Ready

```javascript
AskUserQuestion({
  questions: [{
    question: "Solution validated ({confidence}%). Fix: '{solution}'. Generate deliverables?",
    header: "Solution",
    options: [
      { label: "JIRA + Customer response", description: "All deliverables" },
      { label: "JIRA only", description: "Engineering escalation" },
      { label: "Customer response only", description: "No escalation needed" },
      { label: "Refine solution", description: "Need adjustments" }
    ],
    multiSelect: false
  }]
})
```

**WAIT for response. Do NOT auto-proceed.**

---

## PHASE 7: DELIVERABLES (Main Claude Direct)

### Step 7.1: Check Verification Gate (≥99% claims verified)

Before generating deliverables, verify all claims:
```bash
# Count verified vs unverified claims
jq '[.claims[] | select(.verification.status == "VERIFIED")] | length' inv_context.json
jq '[.claims[] | select(.verification.status == "UNVERIFIED")] | length' inv_context.json
```

If verification_rate < 99%:
```javascript
AskUserQuestion({
  questions: [{
    question: "Verification rate {rate}% below 99%. {N} claims unverified. Options:",
    header: "Verification Gate",
    options: [
      { label: "Verify remaining claims", description: "Research citations" },
      { label: "Deliver with caveats", description: "Mark unverified" },
      { label: "Request override", description: "Kevin approval" }
    ],
    multiSelect: false
  }]
})
```

### Step 7.2: Generate Deliverables

```javascript
call_tool({
  operation: "execute",
  tool: "generate_deliverables",
  params: {
    case_id: "{case_id}",
    deliverables: ["{selected_deliverables}"],
    case_directory: "{case_directory}"
  }
})
```

**FALLBACK** if generate_deliverables fails:

**Step 1**: Read templates (use Glob to find plugin path first):
```bash
# Find plugin installation path dynamically
plugin_path=$(find ~/.claude/plugins -name "cappy-tac-toolkit" -type d 2>/dev/null | head -1)
```

```javascript
// Templates are in plugin's skills directory
// Use relative paths from plugin root: skills/investigate/scribe/templates/

// JIRA plaintext template
Read({ file_path: "{plugin_path}/skills/investigate/scribe/templates/JIRA_PLAINTEXT.md" })

// JIRA wiki markup template
Read({ file_path: "{plugin_path}/skills/investigate/scribe/templates/JIRA_WIKI_MARKUP.md" })

// Customer response template
Read({ file_path: "{plugin_path}/skills/investigate/scribe/templates/CUSTOMER_RESPONSE.md" })

// ALTERNATIVE: Use templates from deliverables directory
Read({ file_path: "{plugin_path}/templates/deliverables/JIRA_DRAFT.txt" })

// Case-specific templates (workspace-level)
Read({ file_path: "{workspace}/00_Templates/TEMPLATE_JIRA_DRAFT_REVIEW.md" })
Read({ file_path: "{workspace}/00_Templates/TEMPLATE_EVIDENCE_REPOSITORY.md" })
```

**Note**: Replace `{plugin_path}` with actual path found via Glob or use:
- `~/.claude/plugins/cache/cappy-local/cappy-tac-toolkit/*/` (wildcard version)
- Or use `templates/deliverables/` within plugin for portable templates

**Step 2**: Generate deliverables manually:

```javascript
// JIRA escalation (plaintext for email/case)
Write({
  file_path: "{case_dir}/JIRA_TICKET_PLAINTEXT.txt",
  content: `{jira_plaintext_content}`
})

// JIRA escalation (wiki markup for JIRA paste)
Write({
  file_path: "{case_dir}/JIRA_TICKET_WIKI_MARKUP.md",
  content: `{jira_wiki_content}`
})

// Customer response
Write({
  file_path: "{case_dir}/CUSTOMER_RESPONSE_DRAFT.md",
  content: `{customer_response_content}`
})

// Update living documents
Edit({
  file_path: "{case_dir}/JIRA_DRAFT_REVIEW.md",
  old_string: "{placeholder_section}",
  new_string: "{actual_content}"
})

Edit({
  file_path: "{case_dir}/EVIDENCE_REPOSITORY.md",
  old_string: "{placeholder_section}",
  new_string: "{actual_content}"
})

Edit({
  file_path: "{case_dir}/README.md",
  old_string: "{placeholder_section}",
  new_string: "{actual_content}"
})
```

**Step 3**: JIRA Template Compliance (12 Required Sections):

```
1. Issue Description - What happened, when, customer/product/case ID
2. Reason for Jira - Why Engineering (not TAC/customer), precedent cases
3. Observed Behavior - Timeline, specific errors, customer quotes
4. Expected Behavior - What should happen normally
5. Troubleshooting Steps Taken - What we verified, what's pending
6. Similar Issues - JIRA numbers, how Engineering resolved, timeline
7. Workaround - Available workarounds, why customer can't fix root cause
8. Reproducible? - Yes/No, how consistently, trigger events
9. Steps To Reproduce - Sequence of events (not config steps if state issue)
10. Tenant info - Customer, tenant ID, product/version, environment
11. Attachments - Evidence location, file list, pending items
12. Additional Notes - Summary request, timeline, story in a nutshell
```

**Step 4**: Citation Requirements:

Every claim must have a citation:
```
[1] HAR:filename.har:entry_145 - HTTP 500 error
[2] Log:server.log:line_234 - Exception message
[3] Screenshot:error_screenshot.png - UI error state
[4] Pattern:P-XSOAR-127 - Rate limiting pattern match
[5] JIRA:XSUP-12345 - Similar issue resolution
[6] Docs:Cortex XSIAM/Page Title - Documentation reference
```

**Step 5**: Verify deliverables before finalizing:
```bash
# Check JIRA has all 12 sections
grep -c "^##" "{case_dir}/JIRA_TICKET_WIKI_MARKUP.md"  # Should be >= 12

# Check citations exist
grep -c "\[" "{case_dir}/JIRA_TICKET_PLAINTEXT.txt"  # Should have citations

# Check no placeholders remain
grep -i "TODO\|PLACEHOLDER\|TBD\|XXX" "{case_dir}/"*.md  # Should be empty
```

### Step 7.3: JIRA Enhancement (Optional)

```bash
# Get TACO PILOT JIRA insights
open "${CAPPY_DASHBOARD_URL}/dashboard/{case_number}"
# User can run jira-aid query
```

### Step 7.4: Close-Loop Language (MANDATORY)

**PURPOSE**: Use proactive close-loop language to accelerate case closure. Pattern from top quick-closers (Gregory Durkin, Iltiaz Nagaria).

**REQUIRED in every customer response**:

| Scenario | Close-Loop Language |
|----------|---------------------|
| Solution provided | "Let me know if ok to close, or if the issue persists." |
| Awaiting customer action | "Please let me know the results, or I will follow up by {date}." |
| JIRA escalated | "You can track progress on the JIRA. Let me know if ok to close the SF case." |
| Platform issue resolved | "Since service has been restored, let me know if ok to close." |
| No response needed | "If no response by EOD {date}, I will close this case." |

**Customer Response Template Addition**:

Every customer response MUST end with ONE of these close-loop statements:

```markdown
## Next Steps

{Specific action items for customer}

---

Please let me know if this resolves your issue, or if you have any further questions.
If I don't hear back by {date + 2 business days}, I will follow up or close this case.

Best,
{signature}
```

**Why This Works**:
- Sets clear expectations for closure
- Gives customer permission to close
- Creates natural follow-up trigger
- Reduces cases lingering in "Customer Action Requested"

### CHECKPOINT - Final Review

```javascript
AskUserQuestion({
  questions: [{
    question: "Deliverables generated. All claims cited. Ready to finalize?",
    header: "Final",
    options: [
      { label: "Finalize", description: "Save to case directory" },
      { label: "Edit JIRA", description: "Modify escalation draft" },
      { label: "Edit customer response", description: "Adjust communication" },
      { label: "Review citations", description: "Verify evidence chain" },
      { label: "Add TAC review", description: "Generate case review" }
    ],
    multiSelect: false
  }]
})
```

---

## POST-PHASE MENU (Always Available)

After ANY checkpoint, if user seems uncertain or selects "Other":

| Option | Action |
|--------|--------|
| Continue | Proceed to next phase |
| Rerun phase | Apply new context, redo |
| Go back | Return to previous phase |
| Deep dive | Investigate specific finding |
| Research | Documentation/JIRA/Confluence lookup |
| TACO PILOT | Get case-helper guidance |
| Pause | Save state for later |
| Escalate | Flag for human TAC help |

---

## MANDATES (NON-NEGOTIABLE)

### SF Case Lookup
If case number provided, MUST call case-get + case-comments BEFORE triage.

### Cortex Documentation
NEVER use WebFetch for docs.your-vendor.example.com.
ALWAYS use: `call_tool({ operation: "execute", tool: "cortex-docs", params: {...} })`

### Citations
Every claim in deliverables MUST include `[file:location]` citation.
Deliverables without citations are INVALID.

### Verification
After each CAPPY Task returns, MANUALLY verify key claims against actual files.

### Fallback Logging
When orchestrators fail and you use manual commands, LOG to inv_context.json:
```json
{
  "tool_usage": {
    "toolFallbacks": [{
      "timestamp": "ISO8601",
      "phase": N,
      "toolAttempted": "tool_name",
      "failureReason": "reason",
      "manualCommandsUsed": ["commands"],
      "result": "what was found"
    }]
  }
}
```

---

## TOOL QUICK REFERENCE

### Fallback Hierarchy (Primary → Tier 1 → Tier 2 → Tier 3)

| Task | Primary (MCP Orchestrator) | Tier 1 (MCP Individual) | Tier 2 (MCP Gateway) | Tier 3 (Native) |
|------|----------------------------|-------------------------|----------------------|-----------------|
| Pattern match | `triage_case` | `pattern-match` | - | `jq` on pattern DB |
| HAR analysis | `analyze_evidence` | - | - | `jq` commands |
| Log analysis | `analyze_evidence` | - | - | `grep` + `Read` |
| Hypothesis | `cappy_synthesis` | `cortex-docs` + `jira-search` | Gateway search | WebSearch + Claude reasoning |
| Validation | `validate_solution` | `cortex-docs` + `confluence-search` | `jira_search` + `confluence_search` | WebSearch + WebFetch |
| Deliverables | `generate_deliverables` | - | - | Read templates + Write |
| SF Case | `case-get`, `case-comments` | - | - | Vision Direct URL |
| Documentation | `cortex-docs` | `confluence-search` | `confluence_get_page` | WebSearch + WebFetch |
| JIRA search | `jira-search` | - | `jira_search` | Manual JIRA lookup |
| Confluence | `confluence-search` | - | `confluence_search` | WebSearch |

### Native Tool Commands Quick Reference

| Task | Command |
|------|---------|
| Pattern DB search | `jq --arg s "{symptom}" '.patterns[] \| select(.title \| test($s; "i"))' ~/.claude/tools/cappy-cache_latest.json` |
| HAR error count | `jq '[.log.entries[] \| select(.response.status >= 400)] \| length' file.har` |
| HAR status dist | `jq '[.log.entries[].response.status] \| group_by(.) \| map({status: .[0], count: length})' file.har` |
| Log errors | `grep -rn "error\|exception" "$case_dir/extracted/" \| head -100` |
| Timeline | `grep -oP "\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}" file.log \| sort -u` |
| Verify line | `sed -n '{N}p' file.log` |
| Verify HAR entry | `jq '.log.entries[{N}]' file.har` |
| Search docs | `WebSearch({ query: "{topic} site:docs.your-vendor.example.com" })` |

---

## SUB-SKILLS REFERENCE

Read these for detailed guidance:
- `skills/investigate/sub-skills/gate.md` - Phase thresholds, recovery options
- `skills/investigate/sub-skills/curator.md` - Evidence registration, claim tracking
- `skills/investigate/sub-skills/sherlock.md` - Hypothesis validation logic
- `skills/investigate/sub-skills/escalation.md` - Escalation decision trees
- `skills/investigate/SKILL.md` - Complete specification

---

## SCRIBE BANNERS

Display progress banners between phases. Templates in:
`skills/investigate/scribe/banners/BANNER_TEMPLATES.md`

---

## PRIORITY ROUTING (WLA Mapping)

| Customer Priority | CAPPY Severity | Strategy |
|-------------------|----------------|----------|
| P1 | SEV-1 | Parallel agents, checkpoint every phase |
| P2 | SEV-2 | Standard flow, checkpoint at Phase 4 |
| P3 | SEV-3 | Standard flow, checkpoint at deliverables |
| P4 | SEV-4 | Quick triage, async acceptable |

---

---

## FALLBACK STRATEGY (Complete Reference)

### When to Use Each Tier

| Tier | When to Use | Tools Available |
|------|-------------|-----------------|
| **Primary** | Default - MCP server healthy | `triage_case`, `analyze_evidence`, `cappy_synthesis`, `validate_solution`, `generate_deliverables` |
| **Tier 1** | Orchestrator fails, individual tools work | `pattern-match`, `cortex-docs`, `jira-search`, `confluence-search`, `case-get` |
| **Tier 2** | MCP CAPPY tools fail, Gateway works | `mcp__mcp-gateway__mcp_jira__*`, `mcp__mcp-gateway__mcp_confluence__*` |
| **Tier 3** | All MCP fails, native only | `jq`, `grep`, `Read`, `Write`, `WebSearch`, `WebFetch` |

### Fallback Detection

Detect MCP failure by error response:
```
- "Tool not found" → Tier 1
- "Connection refused" → Tier 2 or 3
- "MCP server unavailable" → Tier 3
- Timeout after 30s → Try next tier
```

### Tier 3 Native Commands (No MCP Required)

**Pattern Database**:
```bash
# Location
~/.claude/tools/cappy-cache_latest.json

# Search by keyword
jq --arg k "{keyword}" '.patterns[] | select(.title | test($k; "i"))' ~/.claude/tools/cappy-cache_latest.json

# Filter by product
jq --arg p "{product}" '.patterns[] | select(.product == $p)' ~/.claude/tools/cappy-cache_latest.json

# Get pattern by ID
jq --arg id "{P-XXX}" '.patterns[] | select(.pattern_id == $id)' ~/.claude/tools/cappy-cache_latest.json
```

**HAR Analysis**:
```bash
# Entry count
jq '.log.entries | length' file.har

# Status distribution
jq '[.log.entries[].response.status] | group_by(.) | map({status: .[0], count: length})' file.har

# Find errors
jq '.log.entries[] | select(.response.status >= 400)' file.har

# Slow requests
jq '.log.entries[] | select(.time > 5000)' file.har

# Timeline
jq '[.log.entries[] | .startedDateTime] | sort' file.har
```

**Log Analysis**:
```bash
# Find errors
grep -rn "error\|exception\|failed" "$case_dir/extracted/" | head -100

# With context
grep -rn -B 2 -A 5 "error" "$case_dir/extracted/"*.log

# Timeline
grep -oP "\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}" "$case_dir/extracted/"*.log | sort -u

# Stack traces
grep -rn -A 10 "Traceback\|Exception:" "$case_dir/extracted/"
```

**Documentation Search**:
```javascript
WebSearch({
  query: "{symptom} {product} site:docs.your-vendor.example.com"
})

WebFetch({
  url: "{doc_url}",
  prompt: "Extract: known issues, requirements, troubleshooting steps"
})
```

**JIRA/Confluence (via Gateway)**:
```javascript
// JIRA search
mcp__mcp-gateway__mcp_jira__jira_search({
  jql: "project = XSUP AND text ~ \"{symptom}\"",
  limit: 10
})

// Confluence search
mcp__mcp-gateway__mcp_confluence__confluence_search({
  query: "{symptom} {product}",
  limit: 10
})
```

### Confidence Calculation (Manual)

When MCP tools unavailable, calculate confidence manually:

**Phase 2 (Triage)**:
```
| Factor | Points |
|--------|--------|
| Exact pattern keyword match | +30 |
| Partial pattern match | +15 |
| Multiple patterns match | +10 |
| Evidence errors correlate | +20 |
| Product/version match | +10 |
| JIRA precedent found | +15 |

Total >= 90% = PASS
```

**Phase 5 (Validation)**:
```
| Factor | Weight |
|--------|--------|
| Evidence alignment | 40% |
| Documentation support | 25% |
| JIRA precedent | 20% |
| No contradictions | 15% |

Weighted total >= 99% = PASS
```

### Fallback Logging

**ALWAYS log fallback usage** to inv_context.json:
```json
{
  "tool_usage": {
    "toolFallbacks": [{
      "timestamp": "2026-02-20T10:30:00Z",
      "phase": 2,
      "toolAttempted": "triage_case",
      "failureReason": "MCP connection timeout",
      "fallbackTier": 3,
      "manualCommandsUsed": [
        "jq '.patterns[] | select(...)' pattern-db.json",
        "grep -rn 'error' extracted/"
      ],
      "result": "Found pattern P-XSOAR-127 with 75% confidence"
    }]
  }
}
```

---

**Version**: 2.0.0 | **Updated**: 2026-02-22 | **Architecture**: Main Claude + CAPPY Task + 3-Tier Fallbacks
