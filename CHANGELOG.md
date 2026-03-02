# Changelog

All notable changes to the CAPPY TAC Toolkit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.1.0] - 2026-02-22

### Added
- **Container Sandbox**: All 28 library tools now execute in isolated containers
  - Multi-backend support: Podman (recommended), Docker, Bubblewrap (Linux), Job Objects (Windows)
  - Automatic backend detection with graceful fallback
  - Read-only input mounts, network isolation for forensics tools
- **SandboxedProcessor Enum**: 35 variants covering all internal and library tools
- **SandboxOutput Envelope**: Standardized JSON output with audit metadata
- **Suggested Writes**: Tools return write suggestions instead of writing directly
- **Mount Resolution**: Automatic file path detection and container mount creation
- **Security Tests**: Path traversal, network isolation, write prevention tests
- **Performance Benchmarks**: Sandbox overhead measurement suite

### Security
- Network isolation: 12 tools run with `--network none`
- Filesystem isolation: All input mounts read-only
- HITL gate: `case-private-comment` requires explicit approval
- Audit logging: Every execution tracked with UUID

### Documentation
- `docs/CONTAINER_COMPATIBILITY.md` - Tool-by-tool compatibility matrix
- `docs/SANDBOX_OUTPUT_CONTRACT.md` - JSON schema for sandbox output
- `docs/TOOL_IO_MATRIX.md` - I/O requirements for all 28 tools
- `docs/SANDBOX_TESTING.md` - Test procedures and E2E guide
- `SECURITY.md` - Security policy and threat model

---

## [2.0.0] - 2026-02-22

### Added
- **Domain Library Architecture**: Reorganized tools into 5 domains (Case, Knowledge, Evidence, Synthesis, Delivery)
- **Salesforce Integration**: Full case management with `case-get`, `case-comments`, `case-private-comment`, `case-similar`, `case-attach`
- **3-Tier Fallback Architecture**: Orchestrator → Library Tool → Manual CLI fallback chain
- **CLI Utility Commands**: `doctor`, `status`, `config`, `logs`, `clean`, `init`, `selftest`, `completions`
- **Structured Logging**: Correlation tracking with trace/span IDs across all tool executions
- **Pattern P557**: BIOC PowerShell detection pattern (SF-DEMO-001)
- **Shell Completions**: Auto-generated completions for bash, zsh, fish, powershell

### Changed
- **BREAKING**: Renamed `sf_case` module to `salesforce` internally (tool names unchanged)
- **Version Alignment**: Unified version to 2.0.0 across all components
- **Pattern Database**: Expanded to 540 patterns (was 400)
  - DEFINITIVE: 178 (33%)
  - STRONG: 307 (57%)
  - MODERATE: 55 (10%)
- **Documentation**: Grounded all docs in actual implementation
  - Clarified skills vs CLI commands
  - Updated tool counts (35 capabilities: 7 orchestrators, 7 workflows, 28 library tools)
  - Fixed installation paths to use `~/.cappy/bin/`

### Fixed
- Smart-viewer positional params and Unicode path handling
- SF CLI automatic authentication recovery
- Pattern database duplicate and corrupt pattern cleanup

### Known Issues
- **Bug B-002**: CAPPY Task agent crashes on permission issues in Phases 5-7 (workaround: use Main Claude directly)
- **SF Quoting**: Apostrophes in comment body cause shell escaping issues

---

## [1.7.0] - 2026-02-14

### Added
- **Claude Code Plugin**: Initial plugin structure with skills, commands, agents
- **8-Phase Investigation Methodology**: Full workflow from pre-flight to deliverables
- **HITL Checkpoints**: Mandatory human-in-the-loop at every phase boundary
- **SCRIBE Deliverable Library**: Templates for JIRA, customer response, RCA, infographics
- **inv_context.json Schema v3.5**: Comprehensive investigation state tracking
- **MCP Server**: JSON-RPC stdio server for Claude Code integration

### Changed
- Simplified agent architecture to Main Claude + CAPPY Assistant model
- Reorganized skill directory structure with sub-skills

---

## [1.6.0] - 2026-02-01

### Added
- **Schema v3.5 Features**:
  - Claims verification lifecycle with citations
  - Case outcome tracking with classification
  - Phase timing and duration fields
  - Comprehensive metrics for quarterly reporting
  - Automation triggers for tool invocation

### Changed
- Removed cost estimates from metrics (too variable)
- Added resolution days tracking

---

## [1.5.0] - 2026-01-15

### Added
- **Pattern Database Enrichment**: Quality audit and cleanup
- **Declarative Phase Gates**: Security hooks with automatic verification
- **PII Pattern Consolidation**: Centralized sensitive data detection

### Fixed
- Duplicate pattern removal
- Corrupt pattern data cleanup

---

## [1.0.0] - 2026-01-01

### Added
- Initial release of CAPPY Core
- 7 Core Orchestrators: triage_case, analyze_evidence, research_topic, cappy_synthesis, validate_solution, generate_deliverables, call_tool
- 7 Parallel Workflows: multi_ variants for batch processing
- Pattern database with 400+ known issues
- HAR forensics and log analysis tools
- Cortex documentation search integration
- JIRA and Confluence gateway tools

---

## Version History Summary

| Version | Date | Highlights |
|---------|------|------------|
| 2.1.0 | 2026-02-22 | Container sandbox, security isolation, audit logging |
| 2.0.0 | 2026-02-22 | Domain architecture, Salesforce integration, 540 patterns |
| 1.7.0 | 2026-02-14 | Claude Code plugin, HITL checkpoints, SCRIBE library |
| 1.6.0 | 2026-02-01 | Schema v3.5, metrics, automation triggers |
| 1.5.0 | 2026-01-15 | Pattern enrichment, phase gates, PII detection |
| 1.0.0 | 2026-01-01 | Initial release |
