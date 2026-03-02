# CAPPY Tools Reference - Complete Toolkit

## Overview

CAPPY provides **35 capabilities** accessible via the MCP (Model Context Protocol) server:

- **7 Orchestrators**: Phase-based investigation tools (triage_case, analyze_evidence, etc.)
- **7 Workflows**: Parallel execution variants (multi_triage_case, full_investigation, etc.)
- **28 Library Tools**: Forensics, knowledge, gateway, and Salesforce integrations

Tools are organized into categories: Core (7), Forensics (6), Knowledge (4), Gateway (3), Salesforce (8).

---

## Core Orchestrators (7 Tools)

These are the primary investigation tools, each handling a specific phase of the 8-phase investigation workflow.

### 1. triage_case
**Phase**: 0-1 (Pre-Flight + Discovery)

**Purpose**: Initial symptom analysis with parallel pattern matching, case history search, and JIRA lookup

**Parameters**:
- `symptom` (string): What the customer reported (e.g., "XSOAR War Room Edit button missing")
- `product` (enum): XSOAR, XSIAM, or XDR
- `minConfidence` (0-100): Minimum confidence threshold for pattern matches

**Output**:
- Matched patterns (DEFINITIVE, STRONG, MODERATE ranked by confidence)
- Similar cases from history
- Related JIRA issues
- Suggested next steps

**Example**:
```json
{
  "operation": "execute",
  "tool": "triage_case",
  "params": {
    "symptom": "Integration timeout on webhook trigger",
    "product": "XSOAR",
    "minConfidence": 60
  }
}
```

---

### 2. analyze_evidence
**Phase**: 2-3 (Triage + Evidence Analysis)

**Purpose**: Deep dive into evidence files (bundles, HAR, logs) with automatic extraction and timeline building

**Parameters**:
- `bundlePath` (string): Path to tar.gz log bundle (optional)
- `harPath` (string): Path to HAR file (optional)
- `depth` (enum): shallow, moderate, or deep analysis
- `extract` (array): Specific data to extract (timestamps, errors, HTTP codes)

**Output**:
- Extracted events with timestamps
- HTTP request/response summary
- Error logs categorized
- Timeline of events in order
- File manifest and integrity checks

**Features**:
- Auto-detects bundle format (tar.gz, zip, raw logs)
- Extracts environment details (product version, build, hostname)
- Handles large HAR files (tested up to 500MB)
- Correlates events across multiple log files

**Example**:
```json
{
  "operation": "execute",
  "tool": "analyze_evidence",
  "params": {
    "bundlePath": "/path/to/case123_bundle.tar.gz",
    "depth": "deep",
    "extract": ["timestamps", "error_codes", "http_status"]
  }
}
```

---

### 3. research_topic
**Phase**: Cross-phase (Knowledge Access)

**Purpose**: Multi-source knowledge research across Cortex docs, Confluence, JIRA, and internal playbooks

**Parameters**:
- `topic` (string): What to research (e.g., "Rate limiting in XSIAM collectors")
- `sources` (array): Where to search (cortex_docs, confluence, jira, playbooks)
- `limit` (1-50): Max results per source

**Output**:
- Ranked results from each source
- Key excerpts with citations
- Related topics
- Confidence scores

**Example**:
```json
{
  "operation": "execute",
  "tool": "research_topic",
  "params": {
    "topic": "XSOAR integration authentication failures",
    "sources": ["cortex_docs", "jira", "playbooks"],
    "limit": 5
  }
}
```

---

### 4. cappy_synthesis
**Phase**: 4 (Hypothesis Generation)

**Purpose**: AI-powered hypothesis synthesis from evidence with confidence scoring

**Parameters**:
- `evidence` (object): Extracted evidence from analyze_evidence
- `product` (enum): Product context (XSOAR, XSIAM, XDR)
- `tier` (enum): basic, intermediate, or advanced reasoning level

**Output**:
- Primary hypothesis with confidence (0-100)
- Alternative hypotheses ranked by plausibility
- Supporting evidence citations
- Confidence explanation (why this score)

**Example**:
```json
{
  "operation": "execute",
  "tool": "cappy_synthesis",
  "params": {
    "evidence": {
      "errors": ["CONNECTION_TIMEOUT", "RATE_LIMIT_EXCEEDED"],
      "timeline": "...",
      "config_diff": "..."
    },
    "product": "XSOAR",
    "tier": "advanced"
  }
}
```

---

### 5. validate_solution
**Phase**: 4-5 (Validation + Solution Design)

**Purpose**: Validate hypothesis against evidence and design customer-specific solutions

**Parameters**:
- `hypothesis` (string): Proposed root cause
- `evidence` (object): All available evidence
- `product` (enum): Target product
- `customerContext` (object): Account-specific constraints

**Output**:
- Validation result (VERIFIED, LIKELY, UNVERIFIED)
- Confidence score (0-100)
- Solution steps (ordered, actionable)
- Risk assessment (safe, moderate, risky)
- Rollback procedure if applicable

**Example**:
```json
{
  "operation": "execute",
  "tool": "validate_solution",
  "params": {
    "hypothesis": "Rate limiting is blocking the integration",
    "evidence": {...},
    "product": "XSOAR",
    "customerContext": {
      "plan": "enterprise",
      "environment": "production"
    }
  }
}
```

---

### 6. generate_deliverables
**Phase**: 6-7 (Solution + Deliverables)

**Purpose**: Generate customer-facing documentation and internal records

**Parameters**:
- `type` (enum): customer_response, jira_update, rca_report, html_summary
- `caseData` (object): Full investigation results
- `audience` (enum): customer, internal, executive

**Output**:
- Formatted document (markdown, HTML, or plain text)
- With citations and actionable steps
- PII automatically redacted

**Example**:
```json
{
  "operation": "execute",
  "tool": "generate_deliverables",
  "params": {
    "type": "customer_response",
    "caseData": {...},
    "audience": "customer"
  }
}
```

---

### 7. call_tool (Meta-Orchestrator)
**Phase**: Meta (All phases)

**Purpose**: Route tool calls, discover tools, run workflows, manage PTC

**Operations**:
- `execute`: Call any registered tool
- `discover`: Find tools via semantic search
- `workflow`: Run multi-tool orchestration templates

**Example**:
```json
{
  "operation": "execute",
  "tool": "pattern_match",
  "params": {...}
}
```

Or for workflow:
```json
{
  "operation": "workflow",
  "template": "full-investigation",
  "params": {...}
}
```

---

## Knowledge Tools (3 Tools)

### pattern_match
**Purpose**: Fast pattern lookup in 400+ pattern database

**Parameters**:
- `symptom` (string): Error description or symptom
- `product` (enum): Optional product filter
- `limit` (1-20): Max results

**Output**: Ranked patterns with confidence, solutions, and playbook IDs

---

### case_search
**Purpose**: Search historical cases by error code, symptom, or product

**Parameters**:
- `query` (string): Search term
- `product` (enum): Optional filter
- `limit` (1-50): Max results

**Output**: Matching cases with resolution time, who handled it, and outcome

---

### cortex_docs
**Purpose**: Search official Cortex documentation (XSOAR, XSIAM, XDR)

**Parameters**:
- `query` (string): Topic or error code
- `product` (enum): Product filter
- `version` (string): Optional specific version

**Output**: Matched docs with excerpts and links

---

## Forensics Tools (12 Tools)

### analyze_bundle
Extract and analyze log bundles with timeline generation

### analyze_har
Deep dive into HAR files—HTTP request/response analysis

### parse_logs
Parse text logs with regex and structured extraction

### build_xql
Generate XQL (XSIAM query language) from symptoms

### extract_data
General-purpose data extraction from evidence files

### timeline_correlate
Build and correlate timeline across multiple sources

### mine_har_patterns
Mine HAR for error patterns and request failures

### incident_performance
Extract performance metrics (latency, throughput, errors)

### tsf_analyzer
Analyze XDR TSF (Threat Stack Frame) files

### json_forensics
JSON parsing and schema validation

### har_extractor
Extract specific entries from HAR files

### smart_viewer
Universal file viewer (images, videos, PDFs, text)

---

## Gateway Tools (8 Tools)

### jira_search
Search JIRA by symptom, error code, or issue key

### jira_get_issue
Retrieve full JIRA issue with comments and attachments

### jira_create_issue
Create new JIRA ticket from investigation

### jira_update_issue
Update existing JIRA ticket with findings

### jira_add_comment
Add comment to JIRA issue

### confluence_search
Search internal Confluence wiki

### confluence_get_page
Retrieve Confluence page content

### confluence_get_children
List child pages of a Confluence page

---

## Utility Tools (1 Tool)

### pattern_database
Access pattern database directly—list, filter, and export patterns

---

## Workflow Templates (8 Templates)

Run multi-tool orchestration for complex investigations:

### phase1-parallel
Parallel pattern + case + JIRA search (fast initial triage)

### pattern-enrichment
Enrich patterns with case history and confidence scores

### har-analysis
Complete HAR file analysis with hypothesis generation

### multi-product-cascade
Cross-product investigation (XSOAR + XSIAM + XDR parallel)

### evidence-validation
Phase 4 multi-source validation

### timeline-correlation
Correlate events across bundle + HAR + logs

### full-investigation
Complete Phases 1-4 automated pipeline

### custom
Dynamic code for unique investigation needs (Programmatic Tool Calling)

---

## Tool Access & Invocation Methods

### Method 1: MCP (Model Context Protocol)
**Use case**: Direct tool invocation from Claude Code

All tools accessible through `call_tool`:

```json
{
  "operation": "execute",
  "tool": "triage_case",
  "params": {
    "symptom": "...",
    "product": "XSOAR",
    "minConfidence": 60
  }
}
```

**Advantages**:
- Direct access to tool parameters
- Full response data
- Parallel execution (multiple tools at once)
- Best for automation and complex workflows

---

### Method 2: CLI (Command Line Interface)
**Use case**: Standalone tool execution from terminal

```bash
# Triage with pattern matching
cappy-core triage-case \
  --symptom "Integration timeout" \
  --product XSOAR \
  --min-confidence 60

# Analyze evidence bundle
cappy-core analyze-evidence \
  --bundle-path /path/to/bundle.tar.gz \
  --depth deep

# Extract video frames from case
cappy-core viewer video frames \
  /path/to/video.mp4 \
  --interval 5 \
  --output-dir ./frames

# Extract PDF content
cappy-core viewer pdf extract \
  /path/to/document.pdf \
  --dpi 150

# Analyze HAR file
cappy-core analyze-har \
  --file /path/to/file.har \
  --extract-status-codes
```

**Available CLI Tools**:
- `triage-case` - Initial symptom triage
- `analyze-evidence` - Evidence analysis
- `analyze-bundle` - Log bundle extraction
- `analyze-har` - HAR file analysis
- `build-xql` - XQL query generation
- `viewer` - Media analysis (video, PDF, images, HAR)
- `pattern-match` - Pattern database lookup
- `case-search` - Case history search

**Advantages**:
- Quick testing and exploration
- Integration with shell scripts
- No JSON parameter formatting needed
- Human-readable output

---

### Method 3: Skills (Highest-Level Interface)
**Use case**: Complete investigation workflows with quality gates

```
/investigate SF-12345678
/investigate --symptom "XSOAR War Room not loading" --product XSOAR
/investigate /path/to/bundle.tar.gz

/xsoar --symptom "Integration timeout"
/xsiam --error "Collector connection failed"
/xdr --symptom "Agent offline"
```

**Advantages**:
- Automatic tool chaining
- Quality gates and checkpoints
- Human-in-the-loop verification
- Full audit trail
- Complete deliverables (customer response, JIRA, RCA)
- Hooks enforcing evidence integrity

**How /investigate Works**:
1. **Phase 0-1**: Run triage_case (pattern matching)
2. **Phase 2-3**: Run analyze_evidence (log/HAR analysis)
3. **Phase 4**: Run cappy_synthesis (hypothesis generation)
4. **Phase 4-5**: Run validate_solution (verification)
5. **Phase 6-7**: Run generate_deliverables (customer response, JIRA, RCA)

**Checkpoints**:
- After Phase 1: Review patterns → Select best match
- After Phase 3: Sufficient evidence? → Request more or proceed
- After Phase 4: Confidence > threshold? → Proceed or escalate
- After Phase 5: Solution verified? → Proceed or reconsider
- Before delivery: QA check → Release or revise

---

### Method 4: Sub-Agents (Autonomous Orchestrators)
**Use case**: Complex multi-step investigations with autonomous decision-making

Sub-agents automatically:
- Select appropriate tools
- Chain tools for complex workflows
- Make decisions at checkpoints
- Escalate when uncertain
- Generate complete deliverables

See `/docs/SUB_AGENT_REFERENCE.md` for autonomous agent orchestration details.

---

## Tool Access Matrix

| Tool | MCP | CLI | Skill | Sub-Agent |
|------|-----|-----|-------|-----------|
| triage_case | ✅ | ✅ | ✅ | ✅ |
| analyze_evidence | ✅ | ✅ | ✅ | ✅ |
| analyze_bundle | ✅ | ✅ | ✅ | ✅ |
| analyze_har | ✅ | ✅ | ✅ | ✅ |
| cappy_synthesis | ✅ | ❌ | ✅ | ✅ |
| validate_solution | ✅ | ❌ | ✅ | ✅ |
| generate_deliverables | ✅ | ❌ | ✅ | ✅ |
| pattern_match | ✅ | ✅ | ✅ | ✅ |
| case_search | ✅ | ✅ | ✅ | ✅ |
| jira_search | ✅ | ❌ | ✅ | ✅ |
| build_xql | ✅ | ✅ | ✅ | ✅ |
| viewer | ✅ | ✅ | ⚠️ | ✅ |

**Legend**:
- ✅ Full support
- ⚠️ Limited support
- ❌ Not available

---

## Choosing the Right Access Method

### Use MCP When:
- Building automation scripts
- Running multiple tools in parallel
- Need full control over parameters
- Integrating with external systems
- Programmatic Workflow Calling (PTC)

### Use CLI When:
- Quick testing/exploration
- Running from shell scripts
- One-off evidence analysis
- Integration with automation tools
- Human-friendly output

### Use Skills When:
- Investigating a customer case
- Need complete deliverables (customer response, JIRA, RCA)
- Want automatic quality gates and checkpoints
- First-time analysis (not optimization)
- Need audit trail and chain of custody

### Use Sub-Agents When:
- Case requires autonomous decision-making
- Multiple tools need intelligent coordination
- Want system to escalate when uncertain
- Need complete investigation without human intervention
- Integrating with enterprise workflow systems

---

## Tool Chaining Examples

### Example 1: Complete Investigation Flow

```
triage_case (find patterns)
  ↓
research_topic (gather context)
  ↓
analyze_evidence (deep dive)
  ↓
cappy_synthesis (hypothesis)
  ↓
validate_solution (confirm)
  ↓
generate_deliverables (customer response)
```

### Example 2: Parallel Investigation

```
[parallel]
├─ triage_case (pattern matching)
├─ case_search (history lookup)
└─ jira_search (related issues)

Then:
analyze_evidence (consolidate findings)
```

### Example 3: Custom Workflow (PTC)

```javascript
// Programmatic Tool Calling - dynamic workflow
const [patterns, cases, jira] = await parallel([
  pattern_match(symptom),
  case_search(symptom),
  jira_search(symptom)
]);

const evidence = await analyze_evidence(bundlePath);

const hypothesis = await cappy_synthesis({
  patterns,
  cases,
  evidence
});

const solution = await validate_solution(hypothesis);

return await generate_deliverables(solution);
```

---

## Tool Performance

| Tool | Typical Time | Scalability |
|------|--------------|-------------|
| triage_case | 50-200ms | 400+ patterns |
| analyze_evidence | 500ms-2s | Up to 500MB bundles |
| research_topic | 200-500ms | 129 Confluence pages |
| cappy_synthesis | 1-3s | Complex reasoning |
| validate_solution | 500ms-1s | Multi-source verification |
| generate_deliverables | 200-500ms | Template rendering |

---

## Tool Parameters Reference

### Common Enumerations

**product**: XSOAR, XSIAM, XDR

**depth**: shallow, moderate, deep

**tier**: basic, intermediate, advanced

**confidence**: 0-100 (integer)

**audience**: customer, internal, executive

---

## Error Handling

All tools return standardized error format:

```json
{
  "error": "Tool execution failed",
  "code": "ANALYSIS_FAILED",
  "details": "Pattern database unavailable",
  "suggestion": "Retry with minConfidence reduced to 50"
}
```

---

## Version

**CAPPY Tools**: v1.7.0
**Last Updated**: February 11, 2026
**Status**: All 31 tools active in cappy-core

---

**Related Documentation**:
- `/docs/TOOL_REFERENCE.md` - This document (tool capabilities)
- `/docs/SKILL_REFERENCE.md` - 17 skills (10 public + 7 internal)
- `/docs/AGENT_REFERENCE.md` - CAPPY agent orchestration
- `/docs/HOOKS_REFERENCE.md` - Quality gate pipeline (33 hooks)
- `/skills/investigate/SKILL.md` - Complete investigation workflow
