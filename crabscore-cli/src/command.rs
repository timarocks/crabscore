//! Command execution for CrabScore CLI with graceful degradation
//!
//! This module implements graceful degradation when binaries aren't found.
//! It analyzes project complexity and provides meaningful scores even for
//! simple Rust files without runnable binaries.

use anyhow::Result;
use colored::*;
use tracing::{error, info, warn};

use crabscore_analysis::analysis;
use crabscore_analysis::metrics::BenchmarkRunner;
use crabscore_core::{
    metrics::{CostMetrics, PerformanceMetrics, SafetyMetrics},
    IndustryProfile,
};
use crabscore_cost::provider::{CostProvider, StaticCostProvider};
use crabscore_energy::interface::EnergyMonitor;

use crate::{
    binary_discovery::find_or_build_binary,
    complexity::{analyze_project_complexity, ProjectComplexity},
    estimation::*,
    scoring_engine::ComplexityAwareScoringEngine,
};

use std::path::Path;

/// Execute a CLI command
pub async fn execute(cmd: crate::cli::Commands, verbosity: u8) -> Result<()> {
    init_logging(verbosity);

    match cmd {
        crate::cli::Commands::Score { path, bin } => {
            let input_path = Path::new(&path);

            // Check if this is a Cargo project
            let is_cargo_project = input_path.join("Cargo.toml").exists()
                || input_path
                    .parent()
                    .map(|p| p.join("Cargo.toml").exists())
                    .unwrap_or(false);

            // Analyze project complexity for better scoring
            let project_complexity = analyze_project_complexity(input_path).await?;

            println!("{}", "Analyzing Rust project...".bright_cyan());
            println!("  Files: {}", project_complexity.file_count);
            println!("  Lines of code: {}", project_complexity.total_lines);
            println!("  Functions: {}", project_complexity.function_count);

            // Try to find or build a binary, but don't fail if we can't
            let binary_path = find_or_build_binary(input_path, &bin, is_cargo_project).await;

            // Collect metrics - with graceful degradation
            let (perf, energy, safety, cost) = if let Some(ref exe_path) = binary_path {
                info!("Found executable {} for benchmarking", exe_path.display());
                collect_full_metrics(exe_path, input_path, is_cargo_project).await?
            } else {
                info!("No executable found - using static analysis only");
                collect_static_metrics(input_path, is_cargo_project, &project_complexity).await?
            };

            // Calculate score with complexity-aware engine
            let engine = ComplexityAwareScoringEngine::new(
                IndustryProfile::default(),
                project_complexity.clone(),
            );
            let score = engine.calculate_score(&perf, &energy, &cost, &safety);

            // Display results
            display_results(&score, &project_complexity, binary_path.is_none(), &engine);
        }
        crate::cli::Commands::Report { serve, port } => {
            // Reuse Score flow to gather metrics then generate/serve
            let project_complexity = analyze_project_complexity(Path::new(".")).await?;
            let binary_path = find_or_build_binary(Path::new("."), &None, true).await;

            let (perf, energy, safety, cost) = if let Some(ref exe_path) = binary_path {
                collect_full_metrics(exe_path, Path::new("."), true).await?
            } else {
                collect_static_metrics(Path::new("."), true, &project_complexity).await?
            };

            let engine =
                ComplexityAwareScoringEngine::new(IndustryProfile::default(), project_complexity);
            let score = engine.calculate_score(&perf, &energy, &cost, &safety);

            if serve {
                use crabscore_report::web;
                let addr = ([0, 0, 0, 0], port).into();
                web::serve(score, addr).await?;
            } else {
                use crabscore_report::{formats, generator};
                std::fs::write(
                    "crabscore_report.json",
                    generator::generate_json(&score).to_pretty_string(),
                )?;
                std::fs::write("crabscore_report.html", generator::generate_html(&score))?;
                println!("Reports written to crabscore_report.(json|html)");
                // example exporters
                std::fs::write("report_csrd.json", formats::export_csrd(&score))?;
            }
        }
        crate::cli::Commands::Version => {
            println!("CrabScore CLI {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Metrics Collection Functions
// -----------------------------------------------------------------------------

/// Collect full metrics when binary is available
async fn collect_full_metrics(
    exe_path: &Path,
    project_root: &Path,
    is_cargo_project: bool,
) -> Result<(
    PerformanceMetrics,
    crabscore_core::metrics::EnergyMetrics,
    SafetyMetrics,
    CostMetrics,
)> {
    // Measure performance metrics
    let runner = BenchmarkRunner::default();
    let perf = runner.benchmark(exe_path).await.unwrap_or_else(|e| {
        error!("Performance benchmark failed: {}", e);
        PerformanceMetrics::default()
    });

    // Collect energy metrics
    let monitor = crabscore_energy::interface::NullMonitor;
    let energy = monitor.collect().await.unwrap_or_default();

    // Safety metrics via static analysis
    let analysis_root = if is_cargo_project {
        project_root
    } else {
        project_root.parent().unwrap_or(Path::new("."))
    };
    let safety = analysis::run(analysis_root.to_str().unwrap()).unwrap_or_default();

    // Cost metrics
    let cost_provider = StaticCostProvider::new("cost.json");
    let cost = cost_provider
        .collect(analysis_root.to_str().unwrap())
        .await
        .unwrap_or_else(|_| {
            warn!("Cost provider returned no data – using defaults");
            CostMetrics::default()
        });

    Ok((perf, energy, safety, cost))
}

/// Collect static metrics when no binary is available
async fn collect_static_metrics(
    project_root: &Path,
    _is_cargo_project: bool,
    complexity: &ProjectComplexity,
) -> Result<(
    PerformanceMetrics,
    crabscore_core::metrics::EnergyMetrics,
    SafetyMetrics,
    CostMetrics,
)> {
    // Estimate performance based on code complexity
    let estimated_perf = estimate_performance_from_complexity(complexity);

    // Energy metrics - estimate based on complexity
    let estimated_energy = estimate_energy_from_complexity(complexity);

    // Safety metrics via static analysis
    let safety = analysis::run(project_root.to_str().unwrap()).unwrap_or_default();

    // Cost metrics - estimate based on complexity
    let estimated_cost = estimate_cost_from_complexity(complexity);

    Ok((estimated_perf, estimated_energy, safety, estimated_cost))
}

/// Display results with complexity information
fn display_results(
    score: &crabscore_core::CrabScore,
    complexity: &ProjectComplexity,
    static_only: bool,
    engine: &ComplexityAwareScoringEngine,
) {
    println!("\n{}", "CrabScore Report".bold().bright_white());
    println!("{}", "━".repeat(50).bright_white());

    if static_only {
        println!("{}", "Mode: Static Analysis Only".yellow());
        println!(
            "{}",
            "Note: Performance metrics are estimated based on project complexity".dimmed()
        );
        println!();
    }

    let cert_str = format!("{:?}", score.certification);
    println!(
        "{}: {:.0}/100 [{}]",
        "Overall Score".bold(),
        score.overall,
        cert_str.bright_yellow()
    );

    println!("\n{}", "Breakdown:".bold());
    print_score_bar("Performance", score.performance);
    print_score_bar("Energy", score.energy);
    print_score_bar("Cost", score.cost);

    if score.bonuses > 0.0 {
        println!("\n{}: +{:.1}", "Bonuses".bold(), score.bonuses);

        // Show which bonuses were earned
        for (bonus_name, bonus_value) in engine.get_bonus_breakdown() {
            println!("  {} {} (+{:.1})", "✓".green(), bonus_name, bonus_value);
        }
    }

    println!("\n{}", "Project Complexity:".bold());
    println!("  Files: {}", complexity.file_count);
    println!("  Lines: {}", complexity.total_lines);
    println!("  Functions: {}", complexity.function_count);
    println!("  Dependencies: {}", complexity.dependency_count);
}

// -----------------------------------------------------------------------------
fn init_logging(verbosity: u8) {
    let level = match verbosity {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    std::env::set_var("RUST_LOG", level);
    let _ = tracing_subscriber::fmt::try_init();
}

fn print_score_bar(name: &str, score: f64) {
    let width = 20;
    let filled = ((score / 100.0) * width as f64) as usize;
    let bar = "█".repeat(filled) + &"░".repeat(width - filled);

    let color = if score >= 80.0 {
        bar.bright_green()
    } else if score >= 60.0 {
        bar.yellow()
    } else {
        bar.red()
    };

    println!("  {:12} {:3.0}/100 {}", format!("{}:", name), score, color);
}
