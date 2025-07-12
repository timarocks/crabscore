//! Static analysis & safety utilities.
//!
//! NOTE: This is a lightweight, best-effort implementation that avoids
//! heavyweight compiler plugins. It scans Rust source files via `walkdir`
//! & `syn` to calculate:
//!  * number of `unsafe` blocks/usages
//!  * very rough cyclomatic complexity (branches per fn)
//!  * stub clippy warning count (future work)

use anyhow::Result;
use crabscore_core::metrics::SafetyMetrics;
use syn::{visit::Visit, ItemFn};
use walkdir::WalkDir;

struct UnsafeCounter {
    count: u32,
}
impl<'ast> Visit<'ast> for UnsafeCounter {
    fn visit_expr_unsafe(&mut self, _i: &'ast syn::ExprUnsafe) {
        self.count += 1;
    }
}

struct ComplexityVisitor {
    branches: u32,
}
impl<'ast> Visit<'ast> for ComplexityVisitor {
    fn visit_expr_if(&mut self, _: &'ast syn::ExprIf) {
        self.branches += 1;
    }
    fn visit_expr_match(&mut self, _: &'ast syn::ExprMatch) {
        self.branches += 1;
    }
    fn visit_expr_for_loop(&mut self, _: &'ast syn::ExprForLoop) {
        self.branches += 1;
    }
    fn visit_expr_while(&mut self, _: &'ast syn::ExprWhile) {
        self.branches += 1;
    }
}

/// Analyse a Rust project directory recursively and produce `SafetyMetrics`.
pub fn analyse_project<P: AsRef<std::path::Path>>(root: P) -> Result<SafetyMetrics> {
    let mut unsafe_blocks = 0u32;
    let mut total_complexity = 0u32;
    let mut fn_count = 0u32;

    for entry in WalkDir::new(root) {
        let entry = entry?;
        if entry.file_type().is_file()
            && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
        {
            let src = std::fs::read_to_string(entry.path())?;
            let syntax = syn::parse_file(&src)?;

            // Unsafe counting
            let mut counter = UnsafeCounter { count: 0 };
            counter.visit_file(&syntax);
            unsafe_blocks += counter.count;

            // Cyclomatic complexity (branch count per function + 1)
            for item in syntax.items.iter() {
                if let syn::Item::Fn(ItemFn { block, .. }) = item {
                    let mut visitor = ComplexityVisitor { branches: 0 };
                    visitor.visit_block(block);
                    total_complexity += visitor.branches + 1; // +1 per McCabe
                    fn_count += 1;
                }
            }
        }
    }
    let avg_cyclo = if fn_count > 0 {
        total_complexity as f64 / fn_count as f64
    } else {
        1.0
    };

    Ok(SafetyMetrics {
        unsafe_blocks,
        clippy_warnings: 0, // TODO: invoke clippy or parse warnings file
        avg_cyclomatic: avg_cyclo,
    })
}
