# crabscore-core

Core functionality for CrabScore - The Rust Efficiency Standard.

[![Crates.io](https://img.shields.io/crates/v/crabscore-core?style=flat-square)](https://crates.io/crates/crabscore-core)
[![Documentation](https://docs.rs/crabscore-core/badge.svg)](https://docs.rs/crabscore-core)
[![License](https://img.shields.io/crates/l/crabscore-core?style=flat-square)](LICENSE)

## Overview

This crate provides the fundamental data structures and algorithms for calculating and analyzing software efficiency metrics in Rust projects.

## Features

- **Scoring Engine**: Calculate comprehensive efficiency scores
- **Industry Profiles**: Tailored weights for different domains (WebServices, IoT, Financial, Gaming, Enterprise)
- **Metrics**: Performance, energy, cost, and safety metrics
- **Certification Levels**: From Verified to Pioneer status
- **Zero Unsafe Code**: `#![forbid(unsafe_code)]` for maximum safety

## Usage

```rust
use crabscore_core::{CrabScore, IndustryProfile, scoring::ScoringEngine};

let engine = ScoringEngine::new(IndustryProfile::WebServices);
let score = engine.calculate_score(&perf, &energy, &cost, &safety);
```

## Installation

```toml
[dependencies]
crabscore-core = "0.1.0"
```

## License

Licensed under Apache-2.0. See [LICENSE](../LICENSE) for details.