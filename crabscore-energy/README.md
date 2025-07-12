# crabscore-energy

Energy monitoring for CrabScore - The Rust Efficiency Standard.

[![Crates.io](https://img.shields.io/crates/v/crabscore-energy?style=flat-square)](https://crates.io/crates/crabscore-energy)
[![Documentation](https://docs.rs/crabscore-energy/badge.svg)](https://docs.rs/crabscore-energy)
[![License](https://img.shields.io/crates/l/crabscore-energy?style=flat-square)](LICENSE)

## Overview

Platform-specific energy monitoring with pluggable backends for measuring power consumption and carbon footprint of Rust applications.

## Features

- **Cross-Platform Interface**: Unified API for energy monitoring
- **macOS Support**: Native energy monitoring implementation
- **Graceful Fallback**: NullMonitor for unsupported platforms
- **Async Interface**: Built on Tokio for non-blocking measurements
- **Future Linux/Windows Support**: Ready for platform extensions

## Usage

```rust
use crabscore_energy::interface::EnergyMonitor;

let monitor = crabscore_energy::interface::NullMonitor;
let metrics = monitor.collect().await?;
```

## Installation

```toml
[dependencies]
crabscore-energy = "0.1.0"
```

## License

Licensed under Apache-2.0. See [LICENSE](../LICENSE) for details.