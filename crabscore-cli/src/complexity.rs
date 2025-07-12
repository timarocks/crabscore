//! Project complexity analysis for graceful scoring degradation

use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

/// Project complexity metrics for enhanced scoring
#[derive(Debug, Clone, Default)]
pub struct ProjectComplexity {
    /// Number of Rust source files
    pub file_count: usize,
    /// Total lines of code
    pub total_lines: usize,
    /// Number of function definitions
    pub function_count: usize,
    /// Number of module definitions
    pub module_count: usize,
    /// Number of test functions
    pub test_count: usize,
    /// Number of documentation lines
    pub doc_lines: usize,
    /// Number of dependencies in Cargo.toml
    pub dependency_count: usize,
}

impl ProjectComplexity {
    /// Calculate documentation coverage ratio
    pub fn doc_coverage(&self) -> f64 {
        if self.total_lines == 0 {
            0.0
        } else {
            self.doc_lines as f64 / self.total_lines as f64
        }
    }

    /// Calculate test coverage ratio
    pub fn test_coverage(&self) -> f64 {
        if self.function_count == 0 {
            0.0
        } else {
            self.test_count as f64 / self.function_count as f64
        }
    }

    /// Calculate complexity factor for performance estimation
    pub fn complexity_factor(&self) -> f64 {
        (self.total_lines as f64 / 1000.0).min(10.0)
    }
}

/// Analyze project complexity for enhanced scoring
pub async fn analyze_project_complexity(path: &Path) -> Result<ProjectComplexity> {
    let mut complexity = ProjectComplexity::default();

    // Count dependencies from Cargo.toml if it exists
    let cargo_toml = path.join("Cargo.toml");
    if cargo_toml.exists() {
        if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
            if let Ok(toml) = content.parse::<toml::Value>() {
                if let Some(deps) = toml.get("dependencies").and_then(|d| d.as_table()) {
                    complexity.dependency_count = deps.len();
                }
            }
        }
    }

    // Walk through all Rust files
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
    {
        complexity.file_count += 1;

        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            complexity.total_lines += content.lines().count();

            // Parse code structure with simple heuristics
            for line in content.lines() {
                let trimmed = line.trim();

                // Documentation comments
                if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                    complexity.doc_lines += 1;
                }

                // Function definitions
                if trimmed.starts_with("fn ") || trimmed.contains("fn ") {
                    complexity.function_count += 1;
                }

                // Module definitions
                if trimmed.starts_with("mod ") {
                    complexity.module_count += 1;
                }

                // Test annotations
                if trimmed.contains("#[test]") || trimmed.contains("#[cfg(test)]") {
                    complexity.test_count += 1;
                }
            }
        }
    }

    // Handle single file case
    if complexity.file_count == 0
        && path.is_file()
        && path.extension().and_then(|s| s.to_str()) == Some("rs")
    {
        complexity.file_count = 1;
        if let Ok(content) = std::fs::read_to_string(path) {
            complexity.total_lines = content.lines().count();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("fn ") || trimmed.contains("fn ") {
                    complexity.function_count += 1;
                }
                if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                    complexity.doc_lines += 1;
                }
            }
        }
    }

    Ok(complexity)
}
