# crabscore-analysis

Static code analysis for CrabScore - The Rust Efficiency Standard.

[![Crates.io](https://img.shields.io/crates/v/crabscore-analysis?style=flat-square)](https://crates.io/crates/crabscore-analysis)
[![Documentation](https://docs.rs/crabscore-analysis/badge.svg)](https://docs.rs/crabscore-analysis)
[![License](https://img.shields.io/crates/l/crabscore-analysis?style=flat-square)](LICENSE)

## Overview

Advanced static analysis for Rust code safety, identifying patterns like unsafe blocks, unwrap usage, panic points, and security vulnerabilities.

## Features

- **Safety Analysis**: Detect unsafe blocks and operations
- **Error Handling**: Find unwrap/expect usage patterns
- **Panic Detection**: Identify potential panic points
- **Security Scanning**: Basic vulnerability pattern detection
- **Parallel Processing**: Rayon-powered fast analysis
- **AST Analysis**: Full Rust syntax tree parsing with syn

## Usage

```rust
use crabscore_analysis::analysis;

let safety_metrics = analysis::run("path/to/project")?;
println!("Unsafe blocks: {}", safety_metrics.unsafe_blocks);
```

## Installation

```toml
[dependencies]
crabscore-analysis = "0.1.0"
```

## License

Licensed under Apache-2.0. See [LICENSE](../LICENSE) for details.