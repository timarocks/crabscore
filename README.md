# CrabScore - The Rust Efficiency Standard

![CrabScore Logo](src/assets/crabcore-logo.png)

[![Crates.io](https://img.shields.io/crates/v/crabscore-cli?style=flat-square)](https://crates.io/crates/crabscore-cli)
[![Downloads](https://img.shields.io/crates/d/crabscore-cli?style=flat-square)](https://crates.io/crates/crabscore-cli)
[![License](https://img.shields.io/crates/l/crabscore-cli?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange?style=flat-square)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/github/actions/workflow/status/crabcore/crab-score/ci.yml?style=flat-square)](https://github.com/crabcore/crab-score/actions)
[![codecov](https://img.shields.io/codecov/c/github/crabcore/crab-score?style=flat-square)](https://codecov.io/gh/crabcore/crab-score)
[![GitHub stars](https://img.shields.io/github/stars/crabcore/crab-score?style=flat-square)](https://github.com/crabcore/crab-score/stargazers)

CrabScore is an open-source framework for measuring **performance, energy efficiency, cost, and code safety** of Rust projects. It provides universal compatibility with graceful degradation, working with everything from 10-line scripts to massive applications.

## Features

* **Universal Compatibility**: Works with any Rust project - binaries, libraries, or single files
* **Graceful Degradation**: Provides meaningful scores even without runnable binaries
* **Modular Architecture**: Six specialized crates for different aspects of analysis
* **Bonus System**: Rewards Rust best practices like good documentation and minimal dependencies
* **Industry Profiles**: Tailored scoring for WebServices, IoT, Financial, Gaming, and Enterprise
* **Multiple Output Formats**: JSON reports, HTML dashboards, and live web interface

## Installation

### From crates.io (Recommended)
```bash
cargo install crabscore-cli
```

### From Source
```bash
git clone https://github.com/crabcore/crab-score
cd crab-score
cargo build --workspace --release
cargo install --path crabscore-cli
```

## Quick Start

### Score Any Rust Code
```bash
# Score your current project
crabscore score .

# Score a single file (works with any .rs file)
crabscore score src/main.rs

# Score a library crate
crabscore score path/to/library

# Score a specific binary
crabscore score . --bin my-binary
```

### Generate Reports
```bash
# Generate JSON and HTML reports
crabscore report

# Start live web dashboard
crabscore report --serve --port 8080
```

## Examples

See the [examples/](examples/) directory for complete usage examples:

- **[hello_world.rs](examples/hello_world.rs)** - Minimal example showing graceful degradation
- **[fibonacci.rs](examples/fibonacci.rs)** - Comprehensive example with documentation and tests
- **[library_crate.rs](examples/library_crate.rs)** - Library-only project demonstrating static analysis

## Example Output

```text
CrabScore Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Mode: Static Analysis Only
Note: Performance metrics are estimated based on project complexity

Overall Score: 86/100 [Verified]

Breakdown:
  Performance:  59/100 ███████████░░░░░░░░░
  Energy:       63/100 ████████████░░░░░░░░
  Cost:         95/100 ██████████████████░░

Bonuses: +15.0
  ✓ Small Project Bonus (+2.0)
  ✓ Zero Dependencies (+3.0)

Project Complexity:
  Files: 1
  Lines: 9
  Functions: 1
  Dependencies: 0
```

## Rust 2.0 Philosophy

CrabScore embodies the principles of Rust 2.0:

- **Start Small, Score High**: Rewards minimal, focused code
- **Zero-Cost Abstractions**: Measures actual efficiency
- **Fearless Concurrency**: Ready for async/parallel analysis
- **Memory Safety**: Comprehensive safety metrics
- **Community First**: Open metrics and transparent scoring

## Architecture

CrabScore is built as a Cargo workspace with six specialized crates:

```text
crabscore-core/        Central scoring engine and data structures
crabscore-cli/         Command-line interface and user experience  
crabscore-energy/      Energy monitoring (pluggable backends)
crabscore-analysis/    Static code analysis and safety metrics
crabscore-cost/        Cost calculation and resource mapping
crabscore-report/      Report generation and web dashboard
```

## Bonus System

CrabScore rewards Rust best practices:

- **Small Projects**: +2.0 for <100 lines, +1.0 for <500 lines
- **Documentation**: +2.0 for >20% coverage, +1.0 for >10%
- **Test Coverage**: +3.0 for >80%, +2.0 for >50%, +1.0 for >20%
- **Dependencies**: +3.0 for zero deps, +2.0 for <5, +1.0 for <10

## CI/CD Integration

### GitHub Actions
```yaml
- name: CrabScore Check
  run: |
    cargo install crabscore-cli
    crabscore score --target 85
```

### Cargo.toml Configuration
```toml
[package.metadata.crabscore]
target = "90"  # Fail CI if score < 90
profile = "WebServices"
exclude = ["tests/*", "benches/*"]
```

## Development

```bash
# Fast feedback during development
cargo check --workspace

# Run tests
cargo test --workspace

# Lint code
cargo clippy --all-targets -- -D warnings

# Build documentation
cargo doc --workspace --no-deps
```

## Contributing

We welcome contributions! Please:

1. File an issue for major changes
2. Follow the Rust Code of Conduct
3. Ensure all tests pass
4. Add documentation for new features

## License

Licensed under Apache-2.0. See [LICENSE](LICENSE) for details.

## Links

- [User Guide](USER_GUIDE.md) - Comprehensive usage instructions
- [API Documentation](https://docs.rs/crabscore-cli)
- [GitHub Repository](https://github.com/crabcore/crab-score)
- [crates.io](https://crates.io/crates/crabscore-cli)

---

**CrabScore**: Setting the standard for Rust efficiency since 2025.