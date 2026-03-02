//! CAPPY Core Library
//!
//! Unified Rust orchestration for TAC investigations.
//! Consolidates 7 TypeScript orchestrators into a single Rust library.
//!
//! # Architecture
//!
//! - **orchestrators/**: 7 core MCP tools (triage_case, analyze_evidence, etc.)
//! - **forensics/**: Internal tools accessible via call_tool (json_forensics)
//! - **workflows/**: 7 parallel templates with multi_ prefix
//! - **gateway/**: JIRA/Confluence integration
//! - **providers/**: AI model clients (Claude, Gemini, Ollama)
//! - **utils/**: Context management, pattern database access
//!
//! # Usage
//!
//! ```rust,ignore
//! use cappy_core::orchestrators::{triage_case, TriageCaseParams};
//!
//! let params = TriageCaseParams {
//!     symptom: "OOM error in server logs".to_string(),
//!     product: Some(Product::Xsoar),
//!     ..Default::default()
//! };
//!
//! let result = triage_case(params).await?;
//! ```
//!
//! @version 1.6.0
//! @component cappy-core
//! @tag CAPPY-CORE
//! @created 2026-01-20
//! @modified 2026-02-02
//! @author Kevin Francis Tan and Cappy
//!
//! Changes in v1.6.0:
//! - Cross-Verification Framework: Validate Vision/TACO outputs against sources of truth
//! - New hook: CrossVerificationHook (priority 39) - auto-triggers on Vision/TACO HTML
//! - HTML table extraction: html_table_cells, html_table_row_with_value, html_parse_tables
//! - Intelligent number normalization: 8,338 ↔ 8338, K/M/B suffix expansion
//! - 3-pass claim validation with multi-pattern fallbacks (grep → normalized → HTML)
//! - SmartSearchResult, MultiPassResult types for intelligent extraction
//! - 18 hooks total (was 17) in builtin hook registry
//!
//! Changes in v1.5.0:
//! - ClaimValidator hook for 3-pass verbatim evidence verification
//! - ResearchValidator hook for JIRA/Confluence citation validation
//! - ValidationToolkit with 30+ extraction tools (grep, jq, sed, awk, Python)
//! - CitationLocation enum: Line, HarEntry, JsonPath, Pattern, HtmlTableCell, SmartNumber
//!
//! Changes in v1.4.0:
//! - Added TAC Playbook Parser (src/playbook/mod.rs)
//! - Parse XSOAR/XSIAM/XDR playbook HTML files from Confluence
//! - Extract triage questions, troubleshooting scenarios, escalation templates
//! - Generate pattern database candidates from playbook scenarios
//! - New CLI: cappy-core playbook parse/stats/patterns
//!
//! Changes in v1.3.8:
//! - Phase 7: Automatic Pattern Discovery - error clustering with Levenshtein similarity
//! - New functions: discover_patterns(), cluster_errors(), detect_cooccurrence()
//! - Error normalization: normalize_error() replaces timestamps, UUIDs, IPs, paths with placeholders
//! - Regex generation: generate_match_pattern() creates regex from normalized patterns
//! - 12 new unit tests for pattern discovery functionality
//!
//! Changes in v1.3.7:
//! - Phase 6: Added 34 new patterns to cappy-cache (P379-P412) for Broker VM, Syslog, Networking
//! - Phase 4: Added 9 performance benchmark and stress tests
//! - Phase 6: Added 5 pattern validation tests
//! - Pattern database now at version 5.6.0 with 392 patterns
//!
//! Changes in v1.3.6:
//! - Fixed Environment::default() to return "Unknown" instead of empty strings
//! - analyze_evidence now correctly extracts environment from forensics results
//!
//! Changes in v1.3.5:
//! - Automatic chaining: AnalyzeEvidenceParams::from_triage_pattern() extracts search criteria
//! - Causality validation: validate_causality_chain() verifies causality_hints appear in order
//! - Confidence scoring: SymptomMatchStats tracks match rates with confidence adjustment
//! - New result fields: symptom_match_stats, causality_validation in AnalyzeEvidenceResult
//! - Full triage → analyze pipeline now automated (no manual field extraction needed)
//!
//! Changes in v1.3.4:
//! - broker_analytics: Added symptom-guided investigation via analyze_broker_with_params()
//! - agent_analytics: Added symptom-guided investigation via analyze_agent_with_params()
//! - Both now support search_terms, exclude_patterns, causality_hints for targeted extraction
//! - Unified approach: All 3 analytics libraries now support v1.3.3 pipeline (triage → analyze)
//!
//! Changes in v1.3.3:
//! - Added symptom-guided investigation: triage_case → analyze_evidence pipeline
//! - PatternMatch now includes match_patterns, exclude_if, causality_chain from pattern database
//! - AnalyzeEvidenceParams now accepts search_terms, exclude_patterns, causality_hints
//! - log_analytics: Added targeted extraction via analyze_logs_with_params()
//! - If search_terms provided: grep ONLY for those patterns (symptom-specific)
//! - If no search_terms: fallback to standard blind extraction (backwards compatible)
//!
//! Changes in v1.3.2:
//! - Added file size limits to prevent hangs on large log files (50MB for broker, 20MB for XSOAR)
//! - Added early exit after finding max errors (100 per file in log_parser_util, 50 in log_analytics)
//! - Added analytics integration tests with real bundles (tests/analytics_integration.rs)
//! - Fixed agent-analytics to skip binary .db/ log files (LevelDB/RocksDB logs)
//! - Performance: broker-analytics deep now completes in 2s instead of hanging
//! - Performance: log-analytics deep now completes in 5s instead of 1.5min
//! - Performance: agent-analytics deep now completes in 23s on real TSF bundles
//!
//! Changes in v1.3.1:
//! - Fixed tar.gz extraction to preserve directory structure (was flattening)
//! - Fixed sanitize_tar_path to accept absolute paths (strips leading /)
//! - Fixed broker_analytics to parse pipe-separated system_info.txt format
//! - Made all bundle detection recursive (no fixed directory expectations)
//! - Fixed log_analytics to handle XSOAR bundles with absolute paths
//!
//! Changes in v1.3.0:
//! - Added broker-analytics library (XSIAM Broker VM log analysis)
//! - Added agent-analytics library (XDR/XSIAM Agent TSF analysis)
//! - Added log-analytics library (XSOAR 6/8 bundles + D1 Engine logs)
//! - LIBRARY_EXECUTORS expanded to 13 tools (10 core + 3 Confluence)
//! - Shared utilities extracted: archive_util, log_parser_util, format_util
//!
//! Changes in v1.2.2:
//! - Removed legacy json-forensics - use har-forensics or json-utils directly
//! - Split json_forensics into focused libraries: har_forensics + json_utils
//! - Cleaned up LIBRARY_EXECUTORS (8 → 7 tools)
//! - Cleaned up TOOL_REGISTRY metadata (json-forensics → har-forensics + json-utils)
//!
//! Changes in v1.2.1:
//! - Added AI-powered vision analysis (viewer/vision.rs)
//! - Added Ollama preflight hook for HITL model selection
//! - Added ContainerNetwork enum for external AI security
//! - Added json_forensics orchestrator (G-03, G-07, G-08)
//! - DataGuard PII detection before cloud AI calls
//!
//! Changes in v1.1.0:
//! - Added PDF support (viewer/pdf.rs)
//! - Added Linux bubblewrap backend (sandbox/linux.rs)
//! - Added Windows Job Objects backend (sandbox/windows.rs)
//! - Added SandboxBackend enum for backend detection

pub mod cortex_docs;
pub mod error;
pub mod taco;
pub mod taco_reader;
pub mod forensics;
pub mod gateway;
pub mod hooks;
pub mod mcp;
pub mod orchestrators;
pub mod playbook;
pub mod providers;
pub mod sandbox;
pub mod utils;
pub mod viewer;
pub mod workflows;

// Re-export commonly used types
pub use error::{CappyError, Result};
pub use orchestrators::{CommonParams, CommonResult, Confidence, Product, Severity};
pub use providers::{Provider, Tier};
pub use sandbox::{
    ContainerRuntime, SandboxBackend, SandboxConfig, SandboxPolicy, SandboxResult,
    SandboxedProcessor, detect_best_backend, is_sandbox_available,
};
pub use viewer::{
    // Image types
    ExtractedImage, ImageFormat, SvgGenerator, TimelineEvent,
    // Video types
    VideoConvertOptions, VideoConvertResult, VideoInfo, VideoQuality,
    // PDF types
    PdfExtractOptions, PdfExtractResult, PdfMetadata, PdfOutputFormat, ExtractedPage,
};
pub use workflows::WorkflowTemplate;
pub use hooks::{
    HookContext, HookResult, PreToolUse, PostToolUse, HookRegistry,
    builtin::register_all_builtin_hooks,
    init_hooks, hooks_initialized,
};

/// CAPPY Core version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// CAPPY Core description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Initialize CAPPY Core with logging
pub fn init() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    tracing::info!("CAPPY Core v{} initialized", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "1.7.0");
    }

    #[test]
    fn test_product_display() {
        assert_eq!(Product::Xsoar.to_string(), "XSOAR");
        assert_eq!(Product::Xsiam.to_string(), "XSIAM");
        assert_eq!(Product::Xdr.to_string(), "XDR");
    }

    #[test]
    fn test_confidence_ordering() {
        assert!(Confidence::Definitive > Confidence::Strong);
        assert!(Confidence::Strong > Confidence::Moderate);
        assert!(Confidence::Moderate > Confidence::Low);
    }

    #[test]
    fn test_tier_costs() {
        assert_eq!(Tier::Tier0.cost_per_million(), (0.0, 0.0));
        assert_eq!(Tier::Tier1.cost_per_million(), (0.80, 4.00));
    }
}
