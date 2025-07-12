# CrabScore – User Guide

Welcome to **CrabScore**, the Rust efficiency standard. This comprehensive guide covers installation, usage, advanced features, and best practices for achieving optimal efficiency scores.

---

## 1. Installation

### From crates.io (Recommended)
```bash
# Install the latest stable version
cargo install crabscore-cli

# Verify installation
crabscore --version
```

### From Source
```bash
# Clone and build from source
git clone https://github.com/crabcore/crab-score
cd crab-score
cargo build --workspace --release
cargo install --path crabscore-cli
```

### System Requirements
- Rust 1.88.0 or later
- Cargo (included with Rust)
- Optional: Git for source installation

---

## 2. Basic Usage

### 2.1 Universal Scoring

CrabScore works with any Rust code through graceful degradation:

```bash
# Score your current project (auto-detects type)
crabscore score .

# Score a single Rust file (perfect for beginners)
crabscore score hello.rs

# Score a library crate (no binary required)
crabscore score path/to/my-library

# Score a specific binary target
crabscore score . --bin my-server

# Score with verbose output
crabscore score . -vv

# Try the included examples
crabscore score examples/hello_world.rs
crabscore score examples/fibonacci.rs
crabscore score examples/library_crate.rs
```

### 2.2 Understanding Output

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

**Score Components:**
- **Performance**: Latency, throughput, resource efficiency
- **Energy**: Power consumption, carbon footprint  
- **Cost**: Infrastructure, operational, development costs
- **Bonuses**: Rewards for Rust best practices

**Analysis Modes:**
- **Full Benchmarking**: When runnable binary is available
- **Static Analysis Only**: When no binary found (libraries, single files)

---

## 3. Advanced Features

### 3.1 Report Generation

```bash
# Generate JSON and HTML reports
crabscore report

# Start interactive web dashboard
crabscore report --serve --port 8080

# Custom port
crabscore report --serve --port 9000

# Export compliance reports
crabscore report --format csrd  # CSRD compliance
crabscore report --format sbom  # SBOM SPDX format
```

### 3.2 Industry Profiles

CrabScore supports industry-specific scoring profiles:

```bash
# Web services (default)
crabscore score . --profile WebServices

# IoT devices
crabscore score . --profile IoT

# Financial systems
crabscore score . --profile Financial

# Gaming applications
crabscore score . --profile Gaming

# Enterprise software
crabscore score . --profile Enterprise
```

### 3.3 CI/CD Integration

#### GitHub Actions
```yaml
name: CrabScore Quality Check
on: [push, pull_request]

jobs:
  crabscore:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Install CrabScore
        run: cargo install crabscore-cli
      - name: Run CrabScore
        run: crabscore score --target 85
      - name: Upload Reports
        uses: actions/upload-artifact@v3
        with:
          name: crabscore-reports
          path: crabscore_report.*
```

#### Cargo.toml Configuration
```toml
[package.metadata.crabscore]
target = "90"              # Minimum required score
profile = "WebServices"    # Industry profile
exclude = [               # Exclude from analysis
  "tests/*",
  "benches/*",
  "examples/*"
]
bonus_multiplier = 1.2    # Extra weight for specific project
```

---

## 4. Maximizing Your Score

### 4.1 Understanding the Bonus System

CrabScore rewards Rust best practices with bonus points:

**Size Bonuses**
- **Small Project** (+2.0): Less than 100 lines
- **Compact Project** (+1.0): Less than 500 lines

**Documentation Bonuses**  
- **Excellent Documentation** (+2.0): >20% doc coverage
- **Good Documentation** (+1.0): >10% doc coverage

**Test Coverage Bonuses**
- **Excellent Tests** (+3.0): >80% test-to-function ratio
- **Good Test Coverage** (+2.0): >50% test-to-function ratio
- **Basic Test Coverage** (+1.0): >20% test-to-function ratio

**Dependency Bonuses**
- **Zero Dependencies** (+3.0): Self-contained project
- **Minimal Dependencies** (+2.0): Less than 5 dependencies
- **Reasonable Dependencies** (+1.0): Less than 10 dependencies

### 4.2 Best Practices for High Scores

#### Start Small, Score High
```rust
// This 10-line program scores 86/100!
fn main() {
    println!("Hello, Rust 2.0!");
}

/// Calculate fibonacci numbers efficiently
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

#### Document Everything
```rust
//! This crate provides efficient string processing utilities.
//! 
//! # Examples
//! 
//! ```rust
//! use my_crate::process_string;
//! 
//! let result = process_string("hello");
//! assert_eq!(result, "HELLO");
//! ```

/// Converts a string to uppercase with optimal performance.
/// 
/// # Arguments
/// 
/// * `input` - The string to convert
/// 
/// # Returns
/// 
/// The uppercased string
pub fn process_string(input: &str) -> String {
    input.to_uppercase()
}
```

#### Test Thoroughly
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_string() {
        assert_eq!(process_string("hello"), "HELLO");
        assert_eq!(process_string(""), "");
        assert_eq!(process_string("123"), "123");
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
    }
}
```

#### Minimize Dependencies
```toml
[dependencies]
# Only include truly necessary dependencies
serde = { version = "1.0", features = ["derive"] }

[dev-dependencies]
# Test dependencies don't count toward dependency penalty
criterion = "0.5"
```

---

## 5. Troubleshooting

### Common Issues

**"No runnable binary found"**
- **Solution**: This is normal for library crates. CrabScore will use static analysis.
- **Alternative**: Add a simple example in `examples/` directory.
- **Reference**: See [examples/library_crate.rs](../examples/library_crate.rs) for a complete library example.

**Low performance scores**
- **Check**: Is your code optimal? Consider algorithmic improvements.
- **Verify**: Are you using the right industry profile?
- **Improve**: Add documentation and tests for bonus points.

**Build failures**
- **Ensure**: Your project compiles with `cargo build --release`
- **Check**: All dependencies are properly specified
- **Fix**: Any compilation errors before running CrabScore

### Getting Help

```bash
# Show help for main command
crabscore --help

# Show help for score subcommand  
crabscore score --help

# Enable verbose logging
crabscore score . -vv

# Check version
crabscore --version
```

---

## 6. Example Projects by Score Range

### Excellent (90-100): Production Ready
```text
Features:
- Comprehensive documentation (>20%)
- Excellent test coverage (>80%)
- Minimal dependencies (<5)
- Clean, efficient code
- Industry-specific optimizations
```

### Very Good (80-89): Well Crafted
```text
Features:
- Good documentation (>10%)
- Solid test coverage (>50%)
- Reasonable dependencies (<10)
- Good performance characteristics
- Following Rust idioms
```

### Good (70-79): Solid Foundation
```text
Features:
- Basic documentation
- Some test coverage (>20%)
- Moderate complexity
- Acceptable performance
- Room for improvement
```

### Needs Improvement (60-69): Development Phase
```text
Areas to focus:
- Add documentation
- Increase test coverage
- Review dependencies
- Optimize performance
- Simplify where possible
```

---

## 7. Contributing to CrabScore

CrabScore is open source and welcomes contributions:

### Reporting Issues
- Use GitHub Issues for bugs and feature requests
- Provide minimal reproduction cases
- Include CrabScore version and Rust version

### Contributing Code
- Fork the repository
- Create a feature branch
- Ensure all tests pass
- Follow Rust conventions
- Add documentation for new features

### Community
- Follow the Rust Code of Conduct
- Be respectful and constructive
- Help others in discussions

---

## 8. AI Guidance Protocol

When extending CrabScore with AI-based tooling, follow these principles:

### Core Principles
- **NO EMOJIS**: Avoid computational contamination
- **Ethical Alignment**: Ensure AI decisions align with human values
- **Transparency**: Make AI decision-making processes explainable
- **Human Oversight**: Maintain human control over critical decisions

### Practical Application
- **Contextual Understanding**: AI must understand project goals
- **Data-Driven Insights**: Base all changes on measurable outcomes
- **Efficient Automation**: Automate without over-engineering
- **Continuous Improvement**: Monitor and improve AI components

---

**CrabScore**: Your path to Rust efficiency excellence. Start scoring today and join the community building the future of efficient Rust development.