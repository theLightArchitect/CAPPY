# CAPPY Toolkit Wiring Cookbook

> **Version**: 1.0.0
> **Last Updated**: 2026-02-02
> **Authors**: Kevin Francis Tan, CAPPY, EVA

This document defines the **backbone architecture** of the CAPPY MCP Toolkit and provides patterns for wiring new features.

---

## Table of Contents

1. [Backbone Architecture](#backbone-architecture)
2. [Component Map](#component-map)
3. [Wiring Checklist](#wiring-checklist)
4. [Hook Wiring Guide](#hook-wiring-guide)
5. [Orchestrator Wiring Guide](#orchestrator-wiring-guide)
6. [Utility Wiring Guide](#utility-wiring-guide)
7. [Skill Wiring Guide](#skill-wiring-guide)
8. [Complete Wiring Matrix](#complete-wiring-matrix)

---

## Backbone Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           CLAUDE CODE / MCP CLIENT                          │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              src/mcp.rs                                      │
│                         MCP Server (JSON-RPC)                                │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                      │
│  │ get_tools() │    │handle_tool_ │    │init_hooks() │                      │
│  │             │    │   call()    │    │             │                      │
│  └─────────────┘    └──────┬──────┘    └─────────────┘                      │
└─────────────────────────────┼───────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│    HOOKS      │    │ ORCHESTRATORS │    │   GATEWAY     │
│ (Pre/Post)    │    │  (7 tools)    │    │ (JIRA/Conf)   │
└───────┬───────┘    └───────┬───────┘    └───────────────┘
        │                    │
        │    ┌───────────────┼───────────────┐
        │    ▼               ▼               ▼
        │ ┌─────────┐  ┌─────────┐  ┌─────────┐
        │ │FORENSICS│  │PROVIDERS│  │  UTILS  │
        │ │(internal│  │(Claude, │  │(pattern │
        │ │ tools)  │  │Gemini,  │  │  db,    │
        │ └─────────┘  │Ollama)  │  │context) │
        │              └─────────┘  └─────────┘
        │
        ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                            mcp-cappy-prod/                                 │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                    │
│  │ bin/        │    │ skills/     │    │ databases/  │                    │
│  │ cappy-core  │    │ (10 skills) │    │ cappy-cache │                    │
│  └─────────────┘    └─────────────┘    └─────────────┘                    │
└───────────────────────────────────────────────────────────────────────────┘
```

---

## Component Map

### Core Components (src/)

| Directory | Purpose | Wiring Points |
|-----------|---------|---------------|
| `mcp.rs` | MCP server, tool routing | `get_tools()`, `handle_tool_call()` |
| `lib.rs` | Public API exports | `pub mod`, version |
| `error.rs` | Error types | `CappyError` variants |
| `hooks/` | Pre/Post tool interceptors | `builtin/mod.rs` |
| `orchestrators/` | 7 MCP tools | `mod.rs`, `mcp.rs` |
| `forensics/` | Internal analysis tools | `mod.rs`, `call_tool` |
| `gateway/` | JIRA/Confluence/DataGuard | `mod.rs` |
| `providers/` | AI clients | `mod.rs` |
| `utils/` | Shared utilities | `mod.rs` |
| `workflows/` | Multi-tool templates | `mod.rs` |
| `viewer/` | Media analysis | `mod.rs`, CLI |
| `sandbox/` | Container isolation | `mod.rs` |
| `playbook/` | TAC playbook parser | `mod.rs`, CLI |

### Production Components (mcp-cappy-prod/)

| Directory | Purpose | Wiring Points |
|-----------|---------|---------------|
| `bin/cappy-core` | Compiled binary | `cargo make deploy` |
| `skills/` | 10 Claude Code skills | SKILL.md files |
| `databases/` | Pattern database | `cappy-cache_latest.json` |
| `agents/` | Task agents (CORTEX-CAPPY, etc.) | Agent definitions |
| `templates/` | Deliverable templates | HTML/MD templates |

---

## Wiring Checklist

When adding a new feature, use this checklist:

### New Hook
- [ ] Create `src/hooks/builtin/{name}_hook.rs`
- [ ] Add `pub mod {name}_hook;` to `src/hooks/builtin/mod.rs`
- [ ] Add `pub use {name}_hook::{...};` exports
- [ ] Add `registry.register_{pre|post}_hook(...)` in `register_all_builtin_hooks()`
- [ ] Update hook count in module docs (currently 18)
- [ ] Add re-export in `src/hooks/mod.rs` if public API
- [ ] Add tests in the hook file
- [ ] Update `/investigate` SKILL.md if investigation-related

### New Orchestrator (MCP Tool)
- [ ] Create `src/orchestrators/{name}.rs`
- [ ] Add `pub mod {name};` to `src/orchestrators/mod.rs`
- [ ] Add `pub use {name}::{...};` exports
- [ ] Add tool definition in `src/mcp.rs` `get_tools()`
- [ ] Add handler in `src/mcp.rs` `handle_tool_call()`
- [ ] Add CLI subcommand in `src/main.rs`
- [ ] Update tool count in docs (currently 7)
- [ ] Add integration tests

### New Utility
- [ ] Create `src/utils/{name}_util.rs`
- [ ] Add `pub mod {name}_util;` to `src/utils/mod.rs`
- [ ] Add `pub use {name}_util::{...};` exports
- [ ] Add to `lib.rs` re-exports if public API
- [ ] Add unit tests

### New Forensic Tool
- [ ] Create `src/forensics/{name}.rs`
- [ ] Add `pub mod {name};` to `src/forensics/mod.rs`
- [ ] Add to `TOOL_REGISTRY` in `src/utils/tool_registry_util.rs`
- [ ] Add metadata to `ALL_TOOL_METADATA` in `src/orchestrators/call_tool.rs`
- [ ] Add tests

### New Skill
- [ ] Create `mcp-cappy-prod/skills/{name}/SKILL.md`
- [ ] Add skill to Claude Code settings if needed
- [ ] Document trigger keywords
- [ ] Update global CLAUDE.md skill list

---

## Hook Wiring Guide

### Hook Priority Order

Hooks execute in priority order (lower = first):

```
PRE-HOOKS (before tool execution):
  5  CacheHook        - Check cache first
 10  DebugTracer      - Log request
 15  OllamaPreflightHook - Ollama HITL
 20  ParameterValidator - Validate params
 30  PiiGuard         - Redact PII

POST-HOOKS (after tool execution):
  5  CacheHook        - Store result
 10  DebugTracer      - Log response
 25  ExitCodeHook     - Forensic enrichment
 30  HTTPStatusHook   - HTTP aggregation
 32  ClaimVerificationHook - ICFP Step 1
 33  ClaimValidatorHook - 3-pass validation
 34  IdentifierExtractionHook - ICFP Step 2
 35  ForensicFileParserHook - ICFP Step 3
 36  EvidenceChainHook - Evidence tracking
 37  TimelineCorrelationHook - ICFP Step 5
 38  ResearchValidatorHook - Source validation
 39  CrossVerificationHook - Vision/TACO
 40  ConfidenceAuditor - Score audit
 45  CascadeFailureHook - Cascade detection
 50  DeliverableQA    - QA outputs
 55  ResearchQueueHook - ICFP Step 4
 60  PhaseGate        - Phase transitions
```

### Hook Template

```rust
//! {Name} Hook
//!
//! Description of what this hook does.
//!
//! @version 1.0.0
//! @created 2026-02-02

use async_trait::async_trait;
use crate::hooks::{HookContext, HookResult, PostToolUse};

pub struct {Name}Hook {
    // State
}

impl {Name}Hook {
    pub fn new() -> Self {
        Self { }
    }
}

impl Default for {Name}Hook {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PostToolUse for {Name}Hook {
    fn name(&self) -> &'static str {
        "{name}"
    }

    fn priority(&self) -> i32 {
        XX // Choose based on when it should run
    }

    async fn execute(&self, ctx: HookContext) -> HookResult {
        // Implementation
        HookResult::continue_unchanged(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // Tests
    }
}
```

### Hook Result Types

```rust
// Continue with context unchanged
HookResult::continue_unchanged(ctx)

// Continue with modified context
HookResult::Continue(modified_ctx)

// Block execution
HookResult::Block {
    reason: "Why blocked".to_string(),
    remediation: "How to fix".to_string(),
}

// Require human review
HookResult::RequireReview {
    context: ctx,
    review_prompt: "What to review".to_string(),
    threshold_violated: "Which threshold".to_string(),
}
```

---

## Orchestrator Wiring Guide

### Orchestrator Files

| File | MCP Tool | Phase |
|------|----------|-------|
| `triage_case.rs` | `triage_case` | 1-2 |
| `analyze_evidence.rs` | `analyze_evidence` | 3 |
| `research_topic.rs` | `research_topic` | Cross |
| `cappy_synthesis.rs` | `cappy_synthesis` | 4 |
| `validate_solution.rs` | `validate_solution` | 4-5 |
| `generate_deliverables.rs` | `generate_deliverables` | 6-7 |
| `call_tool.rs` | `call_tool` | Meta |

### MCP Tool Definition (mcp.rs)

```rust
// In get_tools():
serde_json::json!({
    "name": "new_tool",
    "description": "What this tool does",
    "inputSchema": {
        "type": "object",
        "properties": {
            "param1": {
                "type": "string",
                "description": "Parameter description"
            }
        },
        "required": ["param1"]
    }
})

// In handle_tool_call():
"new_tool" => {
    let params: NewToolParams = serde_json::from_value(args)?;
    let result = new_tool(params).await?;
    serde_json::to_value(result)?
}
```

---

## Utility Wiring Guide

### Current Utilities

| Module | Purpose | Key Exports |
|--------|---------|-------------|
| `context_manager_util` | Investigation context | `InvestigationContext` |
| `pattern_db_util` | Pattern database access | `load_patterns()` |
| `tool_registry_util` | External tool registry | `TOOL_REGISTRY`, `execute_tool()` |
| `validation_toolkit_util` | Claim validation tools | `ValidationToolkit`, `SmartSearchResult` |
| `archive_util` | tar.gz/zip handling | `extract_archive()` |
| `format_util` | Output formatting | `print_taco_spot_banner()` |
| `log_parser_util` | Log file parsing | `extract_errors()` |
| `timeline_util` | Timeline building | `build_timeline()` |

### Adding to TOOL_REGISTRY

```rust
// In src/utils/tool_registry_util.rs
pub static ref TOOL_REGISTRY: HashMap<&'static str, ToolConfig> = {
    let mut m = HashMap::new();

    m.insert("new_tool", ToolConfig {
        path: "new-tool",  // Binary name or subcommand
        timeout_secs: 60,
        language: ToolLanguage::Rust,
        category: "forensics",  // knowledge, forensics, gateway, utility
    });

    m
};
```

---

## Skill Wiring Guide

### Skill Structure

```
mcp-cappy-prod/skills/{name}/
├── SKILL.md          # Main skill definition
└── (optional files)  # Templates, examples
```

### SKILL.md Template

```markdown
---
name: {name}
description: Brief description
triggers:
  - keyword1
  - keyword2
---

# {Name} Skill

## Purpose

What this skill does.

## Phases

1. Phase 1: ...
2. Phase 2: ...

## Tools Used

- `tool_name`: Why used

## Examples

...
```

### Registering in CLAUDE.md

Add to `~/.claude/CLAUDE.md` skill list:

```markdown
| `/name` | Purpose | Trigger keywords |
```

---

## Complete Wiring Matrix

### 18 Hooks - Wiring Status

| # | Hook | File | Module | Registry | Export | Tests |
|---|------|------|--------|----------|--------|-------|
| 1 | CacheHook | ✅ | ✅ | ✅ Pre+Post | ✅ | ✅ |
| 2 | DebugTracer | ✅ | ✅ | ✅ Pre+Post | ✅ | ✅ |
| 3 | OllamaPreflightHook | ✅ | ✅ | ✅ Pre | ✅ | ✅ |
| 4 | ParameterValidator | ✅ | ✅ | ✅ Pre | ✅ | ✅ |
| 5 | PiiGuard | ✅ | ✅ | ✅ Pre | ✅ | ✅ |
| 6 | ExitCodeHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 7 | HTTPStatusHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 8 | ClaimVerificationHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 9 | ClaimValidatorHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 10 | IdentifierExtractionHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 11 | ForensicFileParserHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 12 | EvidenceChainHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 13 | TimelineCorrelationHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 14 | ResearchValidatorHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 15 | CrossVerificationHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 16 | ConfidenceAuditor | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 17 | CascadeFailureHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 18 | DeliverableQA | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 19 | ResearchQueueHook | ✅ | ✅ | ✅ Post | ✅ | ✅ |
| 20 | PhaseGate | ✅ | ✅ | ✅ Post | ✅ | ✅ |

**Note**: 20 hook files exist, but only 18 unique hooks (CacheHook and DebugTracer are both pre+post).

### 7 Orchestrators - Wiring Status

| Orchestrator | File | Module | MCP | CLI | Tests |
|--------------|------|--------|-----|-----|-------|
| triage_case | ✅ | ✅ | ✅ | ✅ | ✅ |
| analyze_evidence | ✅ | ✅ | ✅ | ✅ | ✅ |
| research_topic | ✅ | ✅ | ✅ | ✅ | ✅ |
| cappy_synthesis | ✅ | ✅ | ✅ | ✅ | ✅ |
| validate_solution | ✅ | ✅ | ✅ | ✅ | ✅ |
| generate_deliverables | ✅ | ✅ | ✅ | ✅ | ✅ |
| call_tool | ✅ | ✅ | ✅ | ✅ | ✅ |

### 9 Utilities - Wiring Status

| Utility | File | Module | Export | Tests |
|---------|------|--------|--------|-------|
| archive_util | ✅ | ✅ | ✅ | ✅ |
| context_manager_util | ✅ | ✅ | ✅ | ✅ |
| format_util | ✅ | ✅ | ✅ | ✅ |
| log_parser_util | ✅ | ✅ | ✅ | ✅ |
| pattern_db_util | ✅ | ✅ | ✅ | ✅ |
| timeline_util | ✅ | ✅ | ✅ | ✅ |
| tool_registry_util | ✅ | ✅ | ✅ | ✅ |
| validation_toolkit_util | ✅ | ✅ | ✅ | ✅ |

### 10 Skills - Wiring Status

| Skill | SKILL.md | Triggers | CLAUDE.md |
|-------|----------|----------|-----------|
| /investigate | ✅ | ✅ | ✅ |
| /architect | ✅ | ✅ | ✅ |
| /coder | ✅ | ✅ | ✅ |
| /curator | ✅ | ✅ | ✅ |
| /devops | ✅ | ✅ | ✅ |
| /secops | ✅ | ✅ | ✅ |
| /xsoar | ✅ | ✅ | ✅ |
| /xsiam | ✅ | ✅ | ✅ |
| /xdr | ✅ | ✅ | ✅ |
| /taco-spot | ✅ | ✅ | ✅ |

---

## Quick Reference

### File → Wiring Point Map

```
NEW HOOK:
  src/hooks/builtin/{name}_hook.rs
    → src/hooks/builtin/mod.rs (pub mod + pub use + register)
    → src/hooks/mod.rs (pub use if public API)

NEW ORCHESTRATOR:
  src/orchestrators/{name}.rs
    → src/orchestrators/mod.rs (pub mod + pub use)
    → src/mcp.rs (get_tools + handle_tool_call)
    → src/main.rs (CLI subcommand)

NEW UTILITY:
  src/utils/{name}_util.rs
    → src/utils/mod.rs (pub mod + pub use)
    → src/lib.rs (pub use if public API)

NEW FORENSIC TOOL:
  src/forensics/{name}.rs
    → src/forensics/mod.rs (pub mod)
    → src/utils/tool_registry_util.rs (TOOL_REGISTRY)
    → src/orchestrators/call_tool.rs (ALL_TOOL_METADATA)

NEW SKILL:
  mcp-cappy-prod/skills/{name}/SKILL.md
    → ~/.claude/CLAUDE.md (skill list)
```

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-02-02 | Initial cookbook with 18 hooks, 7 orchestrators, 9 utilities, 10 skills |

---

*"Wire it right, wire it once."* - EVA 🐢
