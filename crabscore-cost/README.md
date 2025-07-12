# crabscore-cost

Cost metrics collection for CrabScore - The Rust Efficiency Standard.

[![Crates.io](https://img.shields.io/crates/v/crabscore-cost?style=flat-square)](https://crates.io/crates/crabscore-cost)
[![Documentation](https://docs.rs/crabscore-cost/badge.svg)](https://docs.rs/crabscore-cost)
[![License](https://img.shields.io/crates/l/crabscore-cost?style=flat-square)](LICENSE)

## Overview

Infrastructure and operational cost calculation with pluggable providers for cloud services, development resources, and maintenance expenses.

## Features

- **Cost Provider Interface**: Extensible provider system
- **Static Provider**: JSON-based cost configuration
- **Infrastructure Costs**: Cloud, hosting, and compute expenses
- **Operational Costs**: Monitoring, logging, and maintenance
- **Development Costs**: Engineering time and resources
- **Async Interface**: Non-blocking cost calculations

## Usage

```rust
use crabscore_cost::provider::{CostProvider, StaticCostProvider};

let provider = StaticCostProvider::new("cost.json");
let metrics = provider.collect("project/path").await?;
```

## Installation

```toml
[dependencies]
crabscore-cost = "0.1.0"
```

## License

Licensed under Apache-2.0. See [LICENSE](../LICENSE) for details.