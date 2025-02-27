# Rust Project Configuration

## Project Structure
```
project_name/
├── cline_docs/
│   ├── .clinerules
│   └── projectBrief.md
├── src/
│   ├── lib.rs
│   └── main.rs
├── tests/
├── Cargo.toml
└── README.md
```

## Rust Version
Rust 1.65+

## Dependencies
- anyhow
- thiserror
- clap
- serde
- tokio (optional)
- reqwest (optional)

## Build & Test
```bash
# Build project
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Lint code
cargo clippy

# Generate documentation
cargo doc --open
```

## CI/CD Integration
- Add .github/workflows/ci.yml for GitHub Actions
- Configure pre-commit hooks
