# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Project Overview

CAPPY (AI-Powered Pattern Analysis) is a Rust MCP server for AI-assisted security investigations. It provides 7 orchestrators through a single `call_tool` entry point, a 540-pattern database, and an 8-phase investigation methodology.

## Repository Structure

```
CAPPY/
├── agents/           # Agent definitions for Claude Code
├── commands/         # Slash command definitions (/investigate, /evidence, etc.)
├── databases/        # Pattern database schema and samples
├── docs/             # Architecture decisions, cookbooks, reference docs
├── servers/          # Pre-built binary (darwin-arm64)
├── skills/           # Investigation methodology and sub-skills
├── src/              # Cargo.toml and lib.rs (module overview)
└── templates/        # Deliverable and response templates
```

## Build & Run

```bash
# Install binary
./install.sh

# Or manually
cp servers/cappy-core ~/.cappy/bin/cappy-core
chmod +x ~/.cappy/bin/cappy-core

# Run as MCP server
cappy-core mcp-server

# Health check
cappy-core --version
```

## Key Architecture

- **Meta-Orchestrator Pattern**: Single `call_tool` routes to 7 domain orchestrators
- **Hook Pipeline**: 30+ pre/post hooks for validation, evidence tracking, PII detection
- **Container Sandbox**: All tool executions isolated via Podman/Docker
- **3-Tier AI Routing**: Ollama (free, local) -> Claude -> Gemini with fallback chain

## Coding Standards

- No `.unwrap()` / `.expect()` in production
- No `panic!()` - use `Result<T, E>`
- `clippy::pedantic` as errors
- Cyclomatic complexity <= 10
