//! Scoring logic for CrabScore

use crate::{
    metrics::{CostMetrics, EnergyMetrics, PerformanceMetrics, SafetyMetrics},
    CrabScore, IndustryProfile,
};

/// Engine for calculating CrabScores
pub struct ScoringEngine {
    profile: IndustryProfile,
}

impl ScoringEngine {
    /// Create a new scoring engine with the given industry profile
    pub fn new(profile: IndustryProfile) -> Self {
        Self { profile }
    }

    /// Calculate a CrabScore based on the provided metrics
    pub fn calculate_score(
        &self,
        performance: &PerformanceMetrics,
        energy: &EnergyMetrics,
        cost: &CostMetrics,
        safety: &SafetyMetrics,
    ) -> CrabScore {
        let weights = self.profile.weights();

        let perf_score = self.score_performance(performance);
        let energy_score = self.score_energy(energy);
        let cost_score = self.score_cost(cost);
        let bonuses = self.score_safety(safety);

        let overall = (perf_score * weights.performance)
            + (energy_score * weights.energy)
            + (cost_score * weights.cost)
            + bonuses;

        let certification = if overall >= 85.0 {
            crate::Certification::Certified
        } else if overall >= 70.0 {
            crate::Certification::Verified
        } else {
            crate::Certification::None
        };

        let metadata = crate::ScoreMetadata {
            project_name: String::new(),
            version: String::new(),
            profile: self.profile,
            measurements: crate::MeasurementSummary {
                duration: std::time::Duration::from_secs(0),
                iterations: 0,
                environment: crate::Environment {
                    os: String::new(),
                    cpu: String::new(),
                    memory_gb: 0.0,
                    rust_version: String::new(),
                },
            },
        };

        CrabScore {
            overall,
            performance: perf_score,
            energy: energy_score,
            cost: cost_score,
            bonuses,
            certification,
            timestamp: chrono::Utc::now(),
            metadata,
        }
    }

    // ---------------------------------------------------------------------
    // Internal helpers
    // ---------------------------------------------------------------------

    fn clamp(v: f64) -> f64 {
        v.clamp(0.0, 100.0)
    }

    fn score_performance(&self, m: &PerformanceMetrics) -> f64 {
        // Simple heuristic combining latency (lower better) & throughput (higher better)
        let latency_ms = m.latency.p95_ms.max(1.0); // avoid div-by-zero
        let latency_score = (1.0 / (1.0 + latency_ms / 100.0)) * 100.0;

        let tps = m.throughput.requests_per_second;
        let throughput_score = (tps / (tps + 1000.0)) * 100.0;

        let resource_score = 100.0 * m.resource_usage.cpu_efficiency.min(1.0);

        Self::clamp((latency_score + throughput_score + resource_score) / 3.0)
    }

    fn score_energy(&self, m: &EnergyMetrics) -> f64 {
        // Lower power and higher renewable percentage boost score
        let watts = m.direct_consumption.average_watts.max(1.0);
        let power_score = (1.0 / (1.0 + watts / 100.0)) * 100.0;
        let renewable_score = m.carbon_efficiency.renewable_percentage * 100.0;
        Self::clamp((power_score + renewable_score) / 2.0)
    }

    fn score_cost(&self, m: &CostMetrics) -> f64 {
        let infra = m.infrastructure.cloud_compute_usd;
        let infra_score = (1.0 / (1.0 + infra / 1000.0)) * 100.0;
        let ops_score = (1.0 / (1.0 + m.operations.overhead_percentage)) * 100.0;
        Self::clamp((infra_score + ops_score) / 2.0)
    }

    fn score_safety(&self, s: &SafetyMetrics) -> f64 {
        let mut bonus = 0.0;
        if s.unsafe_blocks == 0 {
            bonus += 4.0;
        }
        if s.clippy_warnings == 0 {
            bonus += 3.0;
        }
        if s.avg_cyclomatic <= 10.0 {
            bonus += 3.0;
        }
        bonus // out of 10 max, added directly
    }
}
