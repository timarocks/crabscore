# crabscore-report

Report generation and web dashboard for CrabScore - The Rust Efficiency Standard.

[![Crates.io](https://img.shields.io/crates/v/crabscore-report?style=flat-square)](https://crates.io/crates/crabscore-report)
[![Documentation](https://docs.rs/crabscore-report/badge.svg)](https://docs.rs/crabscore-report)
[![License](https://img.shields.io/crates/l/crabscore-report?style=flat-square)](LICENSE)

## Overview

Beautiful report generation with multiple output formats including JSON, HTML, and live web dashboards powered by Axum.

## Features

- **JSON Reports**: Machine-readable detailed metrics
- **HTML Reports**: Static HTML with embedded visualizations
- **Web Dashboard**: Live interactive dashboard with real-time updates
- **Export Formats**: CSRD, CSV, and custom formats
- **Template Engine**: Handlebars-powered customizable reports
- **Static Assets**: Professional UI with charts and visualizations

## Usage

```rust
use crabscore_report::{generator, web};

// Generate static reports
let json = generator::generate_json(&score);
let html = generator::generate_html(&score);

// Serve live dashboard
let addr = ([0, 0, 0, 0], 8080).into();
web::serve(score, addr).await?;
```

## Installation

```toml
[dependencies]
crabscore-report = "0.1.0"
```

## License

Licensed under Apache-2.0. See [LICENSE](../LICENSE) for details.