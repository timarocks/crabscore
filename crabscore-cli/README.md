# crabscore-cli

Command-line interface for CrabScore - The Rust Efficiency Standard.

[![Crates.io](https://img.shields.io/crates/v/crabscore-cli?style=flat-square)](https://crates.io/crates/crabscore-cli)
[![Documentation](https://docs.rs/crabscore-cli/badge.svg)](https://docs.rs/crabscore-cli)
[![License](https://img.shields.io/crates/l/crabscore-cli?style=flat-square)](LICENSE)

## Overview

The official CLI for measuring Rust project efficiency with graceful degradation, working with everything from single files to massive applications.

## Features

- **Universal Compatibility**: Score binaries, libraries, or single files
- **Graceful Degradation**: Meaningful scores even without runnable binaries
- **Bonus System**: Rewards for documentation, tests, and minimal dependencies
- **Multiple Output Formats**: Console, JSON, HTML reports
- **Web Dashboard**: Live interactive scoring dashboard
- **Industry Profiles**: Tailored for different domains

## Installation

```bash
cargo install crabscore-cli
```

## Usage

```bash
# Score your current project
crabscore score .

# Score a single file
crabscore score main.rs

# Score a library crate
crabscore score path/to/library

# Generate reports
crabscore report

# Start web dashboard
crabscore report --serve --port 8080
```

## Example Output

```
CrabScore Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Overall Score: 86/100 [Verified]

Breakdown:
  Performance:  59/100 ███████████░░░░░░░░░
  Energy:       63/100 ████████████░░░░░░░░
  Cost:         95/100 ██████████████████░░

Bonuses: +15.0
  ✓ Small Project Bonus (+2.0)
  ✓ Zero Dependencies (+3.0)
```

## License

Licensed under Apache-2.0. See [LICENSE](../LICENSE) for details.