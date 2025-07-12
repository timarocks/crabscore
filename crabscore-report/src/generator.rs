//! CrabScore report generator – JSON + minimal HTML

use crabscore_core::CrabScore;
use serde::{Deserialize, Serialize};

/// Wrapper type returned by `/data.json` or `generate_json` helpers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonReport {
    /// The computed CrabScore values for this report.
    pub score: CrabScore,
}

impl JsonReport {
    /// Pretty-print the JSON for human consumption.
    pub fn to_pretty_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Package the score into the JSON wrapper so Serde can serialise it.
pub fn generate_json(score: &CrabScore) -> JsonReport {
    JsonReport {
        score: score.clone(),
    }
}

/// Produce a very small, self-contained HTML page that simply displays the
/// prettified JSON report in a `<pre>` block.  This is only needed by the CLI
/// when the user asks for an HTML file instead of JSON.
pub fn generate_html(score: &CrabScore) -> String {
    let json_pretty =
        serde_json::to_string_pretty(&generate_json(score)).unwrap_or_else(|_| "{}".to_string());

    format!(
        "<!DOCTYPE html><html><head><meta charset='utf-8'><title>CrabScore Report</title>\
         <style>body{{background:#18191c;color:#f7f7f7;font-family:'JetBrains Mono',monospace;\
         padding:2rem}}</style></head><body><h1 style='color:#ff5522;text-align:center'>\
         CRABSCORE REPORT</h1><pre style='background:#111;color:#aaa;padding:1rem;border-radius:0.5rem;\
         overflow-x:auto'>{json_pretty}</pre></body></html>"
    )
}
