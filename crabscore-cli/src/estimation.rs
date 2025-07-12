//! Performance, energy, and cost estimation functions for graceful degradation

use crate::complexity::ProjectComplexity;
use crabscore_core::metrics::*;

/// Estimate performance metrics based on code complexity
pub fn estimate_performance_from_complexity(complexity: &ProjectComplexity) -> PerformanceMetrics {
    // Heuristic: smaller, simpler projects tend to have better latency
    let complexity_factor = complexity.complexity_factor();
    let base_latency = 10.0 + complexity_factor * 5.0;

    PerformanceMetrics {
        latency: LatencyMetrics {
            p50_ms: base_latency,
            p95_ms: base_latency * 1.5,
            p99_ms: base_latency * 2.0,
            cold_start_ms: base_latency * 3.0,
            ttfb_ms: base_latency * 0.3,
        },
        throughput: ThroughputMetrics {
            requests_per_second: 1000.0 / base_latency,
            mb_per_second: 100.0 / complexity_factor.max(1.0),
            concurrent_connections: 100,
            queue_depth: 10.0,
        },
        resource_usage: ResourceMetrics {
            cpu_efficiency: 0.8 - (complexity_factor * 0.05).min(0.5),
            memory_bandwidth_gb_s: 10.0,
            io_operations_per_sec: 1000.0,
            cache_hit_rate: 0.9 - (complexity_factor * 0.02).min(0.3),
        },
        scalability: ScalabilityMetrics::default(),
    }
}

/// Estimate energy metrics based on project size and complexity
pub fn estimate_energy_from_complexity(complexity: &ProjectComplexity) -> EnergyMetrics {
    let size_factor = complexity.complexity_factor();

    EnergyMetrics {
        direct_consumption: PowerConsumption {
            average_watts: 5.0 + size_factor * 2.0,
            peak_watts: 10.0 + size_factor * 5.0,
            idle_watts: 2.0 + size_factor * 0.5,
            joules_per_operation: 0.001 * (1.0 + size_factor * 0.1),
        },
        carbon_efficiency: CarbonEfficiency {
            co2_per_operation: 0.0001 * (1.0 + size_factor * 0.1),
            carbon_intensity: 400.0,   // Default grid intensity
            renewable_percentage: 0.3, // Assume 30% renewable by default
        },
        hardware_lifecycle: HardwareLifecycle {
            thermal_efficiency: 0.8,
            component_stress: 0.2 + (size_factor * 0.05).min(0.5),
            expected_lifespan_years: 5.0,
        },
        algorithmic_efficiency: AlgorithmEfficiency {
            time_complexity: "O(n)".to_string(),
            space_complexity: "O(1)".to_string(),
            actual_time_coefficient: 1.0 + size_factor * 0.1,
            actual_space_coefficient: 1.0 + size_factor * 0.05,
        },
    }
}

/// Estimate cost metrics based on project complexity and maintenance burden
pub fn estimate_cost_from_complexity(complexity: &ProjectComplexity) -> CostMetrics {
    let size_factor = complexity.complexity_factor();
    let maintenance_factor = complexity.function_count as f64 / 10.0;

    CostMetrics {
        infrastructure: InfrastructureCosts {
            cloud_compute_usd: 10.0 + size_factor * 20.0,
            storage_usd: 1.0 + size_factor * 2.0,
            network_egress_usd: 5.0 + size_factor * 5.0,
            cost_per_million_ops: 0.1 + size_factor * 0.05,
        },
        operations: OperationalCosts {
            mttr_minutes: 30.0 + maintenance_factor * 10.0,
            incidents_per_month: 0.5 + size_factor * 0.2,
            overhead_percentage: 0.1 + (size_factor * 0.02).min(0.3),
            monitoring_usd: 5.0 + size_factor * 5.0,
        },
        development: DevelopmentCosts {
            loc: complexity.total_lines as u64,
            cyclomatic_complexity: 1.0 + (complexity.function_count as f64 / 10.0),
            code_churn: 100.0 + size_factor * 50.0,
            onboarding_days: 1.0 + (size_factor * 2.0).min(14.0),
        },
        business_impact: BusinessImpact {
            revenue_per_100ms_latency: 100.0,
            csat_score: 80.0 - size_factor * 2.0,
            sla_compliance: 0.99 - (size_factor * 0.01).min(0.1),
            competitive_advantage: 7.0 - (size_factor * 0.3).min(4.0),
        },
    }
}
