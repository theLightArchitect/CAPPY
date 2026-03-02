# Phase 0-1: Investigation Initialization (v1.7)

**Version**: 2.0.0
**Component**: Investigation Initialization Knowledge
**Purpose**: Guidance for Phase 0-1 setup (no agent spawning in v1.7)
**Created**: 2026-02-05
**Updated**: 2026-02-13 (v1.7 wiring)

---

## Overview (v1.7)

Phase 0-1 is handled entirely by Main Claude with no external agents or MCP tools.

**Flow**:
1. Create case directory structure
2. Extract bundles to extracted/ subdirectory
3. Parse env.log to detect product/version/build
4. Create inv_context.json
5. Ask user to confirm environment detection
6. List evidence files and summarize
7. Checkpoint: Ready for Phase 2?

**Output**: Fully initialized case directory with `inv_context.json` v3.0

---

## Phase 0-1 Detailed Steps

### Purpose
Set up investigation environment, extract bundles, detect product/version, create case context. Main Claude executes these steps locally (no MPC tools).

### Key Responsibilities

#### 1. Case Directory Structure Creation
Create the following directory hierarchy:
```
/case/SF-XXXXXXX/
├── evidence/           # Raw evidence files from customer
├── extracted/          # Extracted/parsed content from bundles
├── analysis/           # Claude's investigation analysis
├── deliverables/       # Final RCA, customer response, JIRA updates
└── inv_context.json    # Authoritative investigation state
```

**Why This Matters**: Central location for all case files ensures consistency and enables distributed team access (P-003 GCS streaming).

#### 2. Bundle Extraction & Analysis
When evidence contains log bundles (.tar.gz, .zip, .gzip):
1. Extract to `extracted/` directory
2. List all files found (count by type)
3. Detect file types:
   - `env.log` → environment information
   - `server.log`, `access.log`, etc. → application logs
   - `*.har` → network traffic (HTTP Archive)
   - `*.json`, `*.xml`, `*.yaml` → configuration files
4. Report findings to CAPPY (master orchestrator)

**Critical Files to Locate**:
- `env.log` - MUST HAVE for environment detection
- HAR files - network analysis
- Application logs - error traces

#### 3. Environment Detection (env.log Parsing)
Parse `env.log` to extract:
```json
{
  "product": "XSOAR|XSIAM|XDR|Firewall|Panorama|Unknown",
  "version": "8.9.0-2464525",
  "build": "2464525",
  "hostname": "xsoar.example.com",
  "architecture": "standalone|cluster|docker",
  "detected_at": "2026-02-05T14:30:00Z",
  "detection_method": "env_log_parsing",
  "confirmed_by_human": false
}
```

**Why env.log First**: Wrong version = wrong troubleshooting patterns. This is the single most critical piece of information for the investigation.

**Detection Patterns**:
```bash
# XSOAR
grep -i "product.*xsoar\|platform.*version" env.log

# XSIAM
grep -i "xsiam\|security analytics" env.log

# XDR
grep -i "xdr\|endpoint detection" env.log

# Version/Build
grep -i "version.*8\.\|build.*[0-9]" env.log
```

#### 4. inv_context.json v3.0 Creation
Create the authoritative investigation state file with pre-populated fields:

```json
{
  "investigation_id": "SF-DEMO-002",
  "created_at": "2026-02-05T14:30:00Z",
  "status": "INITIALIZING",

  "symptom": {
    "user_provided": "XSOAR integration timeout",
    "user_provided_product": "XSOAR (optional)"
  },

  "environment": {
    "product": "XSOAR",
    "version": "8.9.0-2464525",
    "build": "2464525",
    "hostname": "xsoar.example.com",
    "architecture": "standalone",
    "detected_at": "2026-02-05T14:30:00Z",
    "detection_method": "env_log_parsing",
    "confirmed_by_human": false
  },

  "evidence_files": [
    {
      "path": "evidence/bundle.tar.gz",
      "type": "LOG_BUNDLE",
      "size_bytes": 5242880,
      "extracted": true,
      "files_found": 23,
      "extracted_at": "2026-02-05T14:30:30Z"
    },
    {
      "path": "evidence/file.har",
      "type": "HAR",
      "size_bytes": 1048576,
      "entries_count": 267,
      "date_start": "2026-01-15T10:30:00Z",
      "date_end": "2026-01-15T10:45:00Z",
      "user_classification": "API request trace during timeout"
    }
  ],

  "claims": [],

  "verification": {
    "enabled": true,
    "depth": "DEEP",
    "verifiers": ["CAPPY"],
    "citation_format": "file:line|file:entry|timestamp",
    "threshold": 0.90
  },

  "phases": {
    "phase_0": {
      "status": "COMPLETE",
      "started_at": "2026-02-05T14:30:00Z",
      "completed_at": "2026-02-05T14:35:00Z",
      "bundles_extracted": 1,
      "files_found": 23,
      "env_detected": true,
      "env_confidence": 0.95
    },
    "phase_1": {
      "status": "COMPLETE",
      "user_discoveries": [
        {
          "file": "bundle.tar.gz",
          "classification": "platform logs from production",
          "selected_for_analysis": true
        },
        {
          "file": "file.har",
          "classification": "API request trace during integration test",
          "selected_for_analysis": true
        }
      ]
    }
  },

  "timeline": [],
  "lock": {},
  "workflow_issues": [],
  "metadata": {
    "investigator": "Main Claude",
    "start_time": "2026-02-05T14:30:00Z",
    "last_updated": "2026-02-05T14:35:00Z"
  }
}
```

**Fields Explanation**:
- `investigation_id`: Case number from user
- `environment`: Auto-detected from env.log (MUST be verified by user)
- `evidence_files`: List of all files found, including user's classification
- `claims`: Empty at Phase 0-1 (filled during Phases 2-7)
- `phases`: Tracks completion status of each phase
- `verification`: Quality gate thresholds (90% citations required)

#### 5. Interactive Evidence Discovery (Phase 1)
After extracting and detecting, ask user about each evidence file:

**For Each File**:
1. "I found `{filename}` ({size}, {type}). What is this file?"
2. Listen to user's classification
3. "Should we include this in analysis?" (yes/no)
4. Store classification and selection in `inv_context.json`

**Example Conversation**:
```
Agent: "I found bundle.tar.gz (5.2 MB) - appears to be a log bundle. What is this?"
User: "That's the platform logs from production when the integration timeout occurred"
Agent: "Include in analysis?"
User: "Yes"
Agent: "✓ Recorded. Next file..."

Agent: "I found file.har (1.0 MB) - HTTP Archive. What is this?"
User: "That's the API request trace from our integration test"
Agent: "Include in analysis?"
User: "Yes"
```

**Why Interactive**: Users often have context about their evidence that machines can't detect. This ensures we analyze the RIGHT files for the RIGHT reasons.

---

## Agent Orchestration Model

### When CAPPY Reads This Skill

CAPPY (master orchestrator) will:

1. **Read this skill** to understand Phase 0-1 procedures
2. **Create directory structure** (deterministic, no reasoning needed)
3. **Extract bundles** using `call_tool("analyze_evidence", { depth: "EXTRACTION" })`
4. **Parse env.log** to detect product/version/build
5. **Ask user confirmation** on auto-detected environment
6. **Conduct interactive discovery** for each evidence file
7. **Build inv_context.json** using detected + user-provided information
8. **Return results** to Main Claude

### Key Orchestration Decisions

**When to Call `call_tool`**:
- Bundle extraction → `call_tool("analyze_evidence", { bundlePath: "...", depth: "EXTRACTION" })`
- Initial triage for confidence scoring → `call_tool("triage_case", { symptom: "...", includePatterns: true })`

**When to Use Human-in-the-Loop**:
- Environment detection confirmation → AskUserQuestion
- Evidence file classification → AskUserQuestion
- Evidence selection → AskUserQuestion

**When to Use Skill Knowledge**:
- What fields go in inv_context.json → this skill
- What directory structure to create → this skill
- env.log parsing patterns → this skill
- Citation format rules → this skill

---

## Success Criteria

Phase 0-1 is COMPLETE when:

✅ **Directory Structure**: All 4 subdirectories created
✅ **Bundle Extraction**: All files extracted, listed, and typed
✅ **Environment Detected**: Product/version/build/hostname identified from env.log with >85% confidence
✅ **User Confirmed**: User confirmed environment is correct
✅ **inv_context.json Created**: Valid JSON with all required sections
✅ **Evidence Catalogued**: All evidence files listed with user classifications
✅ **Evidence Selected**: User selected which files to analyze
✅ **Ready for Phase 2**: Main Claude can proceed with full context

**Return to Main Claude**:
```
Phase 0-1 COMPLETE ✓

Environment Detected:
  Product: XSOAR
  Version: 8.9.0-2464525
  Build: 2464525
  Hostname: xsoar.example.com

Evidence Files (3 found, 2 selected):
  ✓ bundle.tar.gz (5.2 MB) - 23 files extracted
  ✓ file.har (1.0 MB) - 267 HTTP entries
  ✗ config.xml (ignored)

Case Directory: /case/SF-DEMO-002/
inv_context.json: Ready

Ready for Phase 2 triage.
```

---

## Error Handling

### What If env.log Doesn't Exist?

**Agent Decision**:
1. Ask user: "I couldn't find env.log. Do you know the product/version/build?"
2. Accept user's input
3. Set `environment.detection_method = "user_provided"`
4. Set `environment.confirmed_by_human = true`
5. Continue to Phase 1

**Why This Matters**: Some customers might not have env.log, but they know their environment. Don't block initialization.

### What If Bundle Extraction Fails?

**Agent Decision**:
1. Log error: "Failed to extract bundle"
2. Ask user: "Do you have a pre-extracted copy?"
3. If yes: Use user's extracted files
4. If no: Continue with whatever files are in evidence/
5. Proceed with Phase 1 (use what we have)

**Why This Matters**: Don't let bundle extraction failure block the investigation. We can still analyze individual files.

### What If No Evidence Files Found?

**Agent Decision**:
1. Ask user: "I didn't find any evidence files in the evidence/ directory. Should I:"
   - "A) Create the directory and ask you to add files"
   - "B) Look elsewhere (specify path)"
   - "C) Proceed with pattern/case search only"
2. Based on answer, adjust case directory and proceed

---

## Integration with Existing Orchestrators

### `analyze_evidence` Usage
```
CAPPY calls:
  call_tool("analyze_evidence", {
    bundlePath: "/case/SF-DEMO-002/evidence/bundle.tar.gz",
    depth: "EXTRACTION",
    extractTo: "/case/SF-DEMO-002/extracted"
  })

Returns:
  {
    files_found: 23,
    file_types: {
      logs: 15,
      configs: 5,
      other: 3
    },
    env_log_found: true,
    extraction_time_ms: 1250
  }
```

### `triage_case` Optional Usage
Agent MAY call triage_case during Phase 0-1 for:
- Confidence scoring on detected product/version
- Initial pattern matching

But this is OPTIONAL - Phase 0-1 focuses on setup, not analysis.

---

## Related Components

- **CAPPY agent**: Reads this skill and orchestrates Phase 0-1
- **inv_context.json v3.0**: Output of this skill, used by all subsequent phases
- **/investigate SKILL.md**: Documents Phase 0-1 invocation
- **curator.md**: Evidence management logic for Phase 2-7

---

## Next Phase

Once CAPPY completes Phase 0-1:
1. Main Claude has fully initialized context in inv_context.json
2. Main Claude can proceed to Phase 2 triage
3. CAPPY reads curator.md for claim registration
4. CAPPY reads gate.md for confidence gates

---

**Skill Version**: 2.0.0
**Last Updated**: 2026-02-13
**Status**: Ready for CAPPY orchestration (simplified architecture)
