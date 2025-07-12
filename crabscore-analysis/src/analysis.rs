//! Source-code analysis helpers (wrappers).

use anyhow::Result;
use crabscore_core::metrics::SafetyMetrics;

/// Run full static analysis on given path (proxy to safety::analyse_project).
pub fn run(path: &str) -> Result<SafetyMetrics> {
    super::safety::analyse_project(path)
}
