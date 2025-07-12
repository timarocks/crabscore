//! Performance metrics collection utilities for CrabScore analysis.

use anyhow::Result;
use crabscore_core::metrics::{
    LatencyMetrics, PerformanceMetrics, ResourceMetrics, ScalabilityMetrics, ThroughputMetrics,
};
use std::time::Instant;
use tokio::process::Command;

/// Options controlling how benchmarks are executed.
#[derive(Debug, Clone)]
pub struct BenchmarkOptions {
    /// Number of warm-up iterations (not recorded).
    pub warmup: u32,
    /// Number of measured iterations.
    pub iterations: u32,
    /// Arguments to pass to the executable.
    pub args: Vec<String>,
}

impl Default for BenchmarkOptions {
    fn default() -> Self {
        Self {
            warmup: 1,
            iterations: 5,
            args: Vec::new(),
        }
    }
}

/// Runs a target executable multiple times and aggregates latency statistics.
#[derive(Default)]
pub struct BenchmarkRunner {
    opts: BenchmarkOptions,
}

impl BenchmarkRunner {
    /// Create a new BenchmarkMetrics with the given options.
    pub fn new(opts: BenchmarkOptions) -> Self {
        Self { opts }
    }

    /// Benchmark the given executable and return `PerformanceMetrics`.
    pub async fn benchmark<P: AsRef<std::path::Path>>(
        &self,
        executable: P,
    ) -> Result<PerformanceMetrics> {
        let exe = executable.as_ref();
        let mut samples = Vec::with_capacity(self.opts.iterations as usize);

        // Warm-up runs (ignored)
        for _ in 0..self.opts.warmup {
            let _ = Command::new(exe).args(&self.opts.args).status().await?;
        }

        // Measured runs
        for _ in 0..self.opts.iterations {
            let start = Instant::now();
            let status = Command::new(exe).args(&self.opts.args).status().await?;
            let elapsed = start.elapsed();
            if status.success() {
                samples.push(elapsed.as_secs_f64() * 1000.0); // ms
            }
        }

        if samples.is_empty() {
            return Ok(PerformanceMetrics::default());
        }

        samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let idx =
            |p: f64| ((p * (samples.len() as f64 - 1.0)).round() as usize).min(samples.len() - 1);

        let latency = LatencyMetrics {
            p50_ms: samples[idx(0.50)],
            p95_ms: samples[idx(0.95)],
            p99_ms: samples[idx(0.99)],
            cold_start_ms: samples[0],
            ttfb_ms: 0.0, // not measured here
        };

        // Throughput: ops per second = 1000 / median latency
        let throughput = ThroughputMetrics {
            requests_per_second: if latency.p50_ms > 0.0 {
                1000.0 / latency.p50_ms
            } else {
                0.0
            },
            mb_per_second: 0.0,
            concurrent_connections: 0,
            queue_depth: 0.0,
        };

        let perf = PerformanceMetrics {
            latency,
            throughput,
            resource_usage: ResourceMetrics::default(),
            scalability: ScalabilityMetrics::default(),
        };

        Ok(perf)
    }
}
