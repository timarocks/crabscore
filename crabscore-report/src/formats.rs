//! Compliance & reporting export formats.

use crabscore_core::CrabScore;
use serde_json::json;

/// Export to CSRD-compatible JSON
pub fn export_csrd(score: &CrabScore) -> String {
    serde_json::to_string_pretty(&json!({
        "standard": "CSRD",
        "overall": score.overall,
        "energy": score.energy,
        "timestamp": score.timestamp,
        "certification": format!("{:?}", score.certification),
    }))
    .unwrap()
}

/// Export to Software Bill of Materials (SBOM) fragment.
/// For demo we output minimal SPDX JSON snippet.
pub fn export_sbom(score: &CrabScore) -> String {
    serde_json::to_string_pretty(&json!({
        "SPDXID": "SPDXRef-CrabScore",
        "name": "CrabScore Report",
        "summary": format!("Overall {}", score.overall),
    }))
    .unwrap()
}

/// Export to Cyber Resilience Act (CRA) JSON stub
pub fn export_cra(score: &CrabScore) -> String {
    serde_json::to_string_pretty(&json!({
        "standard": "EU CRA",
        "score": score.overall,
        "compliance": if score.overall >= 70.0 { "PASS" } else { "FAIL" },
    }))
    .unwrap()
}
