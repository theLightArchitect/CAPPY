# Contributing to CAPPY

Thank you for your interest in contributing to CAPPY.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/CAPPY.git`
3. Create a feature branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Run quality checks: `cargo fmt && cargo clippy -- -D warnings`
6. Submit a pull request

## Development Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/theLightArchitect/CAPPY.git
cd CAPPY
./install.sh
```

## Code Standards

- **No `.unwrap()` or `.expect()` in production code** - use `?` or `match`
- **No `panic!()`** - use `Result<T, E>`
- `clippy::pedantic` enforced as errors
- Cyclomatic complexity <= 10 per function
- 60-line function limit

## Reporting Issues

- Use the GitHub issue templates for bugs and feature requests
- For security vulnerabilities, see [SECURITY.md](SECURITY.md)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
