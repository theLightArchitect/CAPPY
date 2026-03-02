# Cookbook 00: Getting Started with Rust MCP Servers

**Purpose**: Entry point for developers building MCP servers in Rust
**Audience**: Developers new to MCP or Rust MCP development
**Prerequisites**: Rust 1.70+, cargo, basic async/await knowledge

---

## What is an MCP Server?

Model Context Protocol (MCP) servers expose tools to AI assistants like Claude Code via JSON-RPC over stdio. Your server:
1. Receives JSON-RPC requests from the AI assistant
2. Executes tool logic
3. Returns JSON-RPC responses

```
┌─────────────────┐     JSON-RPC      ┌─────────────────┐
│  Claude Code    │ ◄──── stdio ────► │  Your MCP Server │
│  (AI Assistant) │                   │  (Rust Binary)   │
└─────────────────┘                   └─────────────────┘
```

---

## Project Structure

```
your-mcp-server/
├── Cargo.toml
├── Makefile.toml              # cargo-make tasks
├── src/
│   ├── main.rs                # CLI entry point
│   ├── lib.rs                 # Library exports
│   ├── mcp.rs                 # MCP server (JSON-RPC over stdio)
│   ├── error.rs               # Error types (thiserror)
│   │
│   ├── orchestrators/         # MCP Tools (your business logic)
│   │   ├── mod.rs
│   │   └── your_tool.rs
│   │
│   ├── hooks/                 # Pre/post tool execution hooks
│   │   ├── mod.rs
│   │   ├── traits.rs
│   │   └── builtin/
│   │
│   └── providers/             # Optional: AI model backends
│       ├── mod.rs
│       └── ollama.rs
│
├── tests/
│   └── integration_tests.rs
│
└── resources/
    └── config.json            # Runtime configuration
```

---

## Minimal Cargo.toml

```toml
[package]
name = "your-mcp-server"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Error handling
thiserror = "1"
anyhow = "1"

# CLI
clap = { version = "4", features = ["derive"] }

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

---

## Quick Start

### 1. Create the MCP Server Entry Point

```rust
// src/main.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "your-server")]
#[command(about = "Your MCP Server")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run as MCP server (JSON-RPC over stdio)
    McpServer,
    /// CLI mode for testing
    YourTool { input: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::McpServer => your_server::mcp::run_mcp_server().await?,
        Commands::YourTool { input } => {
            let result = your_server::orchestrators::your_tool::execute(&input).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}
```

### 2. Implement the MCP Server

See [05-mcp.md](./05-mcp.md) for complete JSON-RPC implementation.

### 3. Build and Test

```bash
# Development
cargo build
cargo test

# Release
cargo build --release

# Run as MCP server
./target/release/your-server mcp-server

# Test a tool via CLI
./target/release/your-server your-tool "test input"
```

### 4. Configure Claude Code

Add to `~/.config/claude-code/config.json`:

```json
{
  "mcpServers": {
    "your-server": {
      "command": "/path/to/your-server",
      "args": ["mcp-server"]
    }
  }
}
```

---

## Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- mcp-server

# Deploy (with cargo-make)
cargo make deploy
```

---

## Next Steps

1. **[01-foundations.md](./01-foundations.md)** - Hook system and utilities
2. **[02-orchestrator.md](./02-orchestrator.md)** - Building MCP tools
3. **[05-mcp.md](./05-mcp.md)** - JSON-RPC server implementation

---

*Platform-agnostic guide for any Rust MCP server*
