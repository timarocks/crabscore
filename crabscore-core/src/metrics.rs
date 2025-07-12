//! Performance, energy, and cost metrics for CrabScore

use serde::{Deserialize, Serialize};

/// Performance-related metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    /// Latency measurements
    pub latency: LatencyMetrics,
    /// Throughput measurements
    pub throughput: ThroughputMetrics,
    /// Resource usage metrics
    pub resource_usage: ResourceMetrics,
    /// Scalability metrics
    pub scalability: ScalabilityMetrics,
}

/// Latency measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyMetrics {
    /// 50th percentile latency in milliseconds
    pub p50_ms: f64,
    /// 95th percentile latency in milliseconds
    pub p95_ms: f64,
    /// 99th percentile latency in milliseconds
    pub p99_ms: f64,
    /// Cold start latency in milliseconds
    pub cold_start_ms: f64,
    /// Time to first byte in milliseconds
    pub ttfb_ms: f64,
}

/// Throughput measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    /// Requests per second
    pub requests_per_second: f64,
    /// Data throughput in MB/s
    pub mb_per_second: f64,
    /// Number of concurrent connections
    pub concurrent_connections: u64,
    /// Queue depth
    pub queue_depth: f64,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// CPU efficiency (operations per cycle)
    pub cpu_efficiency: f64,
    /// Memory bandwidth in GB/s
    pub memory_bandwidth_gb_s: f64,
    /// I/O operations per second
    pub io_operations_per_sec: f64,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
}

/// Scalability metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyMetrics {
    /// Total number of `unsafe` blocks
    pub unsafe_blocks: u32,
    /// Total Clippy warnings (opt-level = pedantic)
    pub clippy_warnings: u32,
    /// Average cyclomatic complexity per function
    pub avg_cyclomatic: f64,
}

impl Default for SafetyMetrics {
    fn default() -> Self {
        Self {
            unsafe_blocks: 0,
            clippy_warnings: 0,
            avg_cyclomatic: 1.0,
        }
    }
}

/// Scalability metrics for concurrent and parallel Rust workloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalabilityMetrics {
    /// Linear scaling factor (1.0 = perfect linear scaling)
    pub linear_scaling_factor: f64,
    /// Performance at different concurrency levels (threads, performance_ratio)
    pub degradation_curve: Vec<(u32, f64)>,
    /// Bottleneck score (0.0 to 1.0, higher is worse)
    pub bottleneck_score: f64,
    /// Elasticity coefficient (0.0 to 1.0, higher is better)
    pub elasticity_coefficient: f64,
}

/// Energy-related metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnergyMetrics {
    /// Direct power consumption measurements
    pub direct_consumption: PowerConsumption,
    /// Carbon efficiency metrics
    pub carbon_efficiency: CarbonEfficiency,
    /// Hardware lifecycle metrics
    pub hardware_lifecycle: HardwareLifecycle,
    /// Algorithmic efficiency metrics
    pub algorithmic_efficiency: AlgorithmEfficiency,
}

/// Power consumption metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerConsumption {
    /// Average power consumption in watts
    pub average_watts: f64,
    /// Peak power consumption in watts
    pub peak_watts: f64,
    /// Idle power consumption in watts
    pub idle_watts: f64,
    /// Energy per operation in joules
    pub joules_per_operation: f64,
}

/// Carbon efficiency metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonEfficiency {
    /// Grams of CO2 per operation
    pub co2_per_operation: f64,
    /// Carbon intensity of energy source (gCO2/kWh)
    pub carbon_intensity: f64,
    /// Renewable energy percentage (0.0 to 1.0)
    pub renewable_percentage: f64,
}

/// Hardware lifecycle metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareLifecycle {
    /// Thermal efficiency (0.0 to 1.0)
    pub thermal_efficiency: f64,
    /// Component stress score (0.0 to 1.0, lower is better)
    pub component_stress: f64,
    /// Expected hardware lifespan in years
    pub expected_lifespan_years: f64,
}

/// Algorithm efficiency metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmEfficiency {
    /// Time complexity (O(n), O(n log n), etc.)
    pub time_complexity: String,
    /// Space complexity (O(1), O(n), etc.)
    pub space_complexity: String,
    /// Actual observed time complexity coefficient
    pub actual_time_coefficient: f64,
    /// Actual observed space coefficient
    pub actual_space_coefficient: f64,
}

/// Cost-related metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CostMetrics {
    /// Infrastructure costs
    pub infrastructure: InfrastructureCosts,
    /// Operational costs
    pub operations: OperationalCosts,
    /// Development costs
    pub development: DevelopmentCosts,
    /// Business impact metrics
    pub business_impact: BusinessImpact,
}

/// Infrastructure cost metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureCosts {
    /// Cloud compute costs per month in USD
    pub cloud_compute_usd: f64,
    /// Storage costs per month in USD
    pub storage_usd: f64,
    /// Network egress costs per month in USD
    pub network_egress_usd: f64,
    /// Cost per million operations in USD
    pub cost_per_million_ops: f64,
}

/// Operational cost metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalCosts {
    /// Mean time to resolve incidents in minutes
    pub mttr_minutes: f64,
    /// Number of incidents per month
    pub incidents_per_month: f64,
    /// Operational overhead percentage (0.0 to 1.0)
    pub overhead_percentage: f64,
    /// Monitoring and alerting costs per month in USD
    pub monitoring_usd: f64,
}

/// Development cost metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentCosts {
    /// Lines of code
    pub loc: u64,
    /// Cyclomatic complexity
    pub cyclomatic_complexity: f64,
    /// Code churn (lines changed per month)
    pub code_churn: f64,
    /// Time to onboard new developer in days
    pub onboarding_days: f64,
}

/// Business impact metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessImpact {
    /// Revenue impact per 100ms latency improvement in USD
    pub revenue_per_100ms_latency: f64,
    /// Customer satisfaction score (0-100)
    pub csat_score: f64,
    /// SLA compliance percentage (0.0 to 1.0)
    pub sla_compliance: f64,
    /// Competitive advantage score (0-10)
    pub competitive_advantage: f64,
}

// -----------------------------------------------------------------------------
// Default implementations for high-level metric structs (temporary placeholders)
// -----------------------------------------------------------------------------

impl Default for LatencyMetrics {
    fn default() -> Self {
        Self {
            p50_ms: 0.0,
            p95_ms: 0.0,
            p99_ms: 0.0,
            cold_start_ms: 0.0,
            ttfb_ms: 0.0,
        }
    }
}

impl Default for ThroughputMetrics {
    fn default() -> Self {
        Self {
            requests_per_second: 0.0,
            mb_per_second: 0.0,
            concurrent_connections: 0,
            queue_depth: 0.0,
        }
    }
}

impl Default for ResourceMetrics {
    fn default() -> Self {
        Self {
            cpu_efficiency: 0.0,
            memory_bandwidth_gb_s: 0.0,
            io_operations_per_sec: 0.0,
            cache_hit_rate: 0.0,
        }
    }
}

impl Default for ScalabilityMetrics {
    fn default() -> Self {
        Self {
            linear_scaling_factor: 1.0,
            degradation_curve: Vec::new(),
            bottleneck_score: 0.0,
            elasticity_coefficient: 0.0,
        }
    }
}

impl Default for PowerConsumption {
    fn default() -> Self {
        Self {
            average_watts: 0.0,
            peak_watts: 0.0,
            idle_watts: 0.0,
            joules_per_operation: 0.0,
        }
    }
}

impl Default for CarbonEfficiency {
    fn default() -> Self {
        Self {
            co2_per_operation: 0.0,
            carbon_intensity: 0.0,
            renewable_percentage: 0.0,
        }
    }
}

impl Default for HardwareLifecycle {
    fn default() -> Self {
        Self {
            thermal_efficiency: 0.0,
            component_stress: 0.0,
            expected_lifespan_years: 0.0,
        }
    }
}

impl Default for AlgorithmEfficiency {
    fn default() -> Self {
        Self {
            time_complexity: String::new(),
            space_complexity: String::new(),
            actual_time_coefficient: 0.0,
            actual_space_coefficient: 0.0,
        }
    }
}

impl Default for InfrastructureCosts {
    fn default() -> Self {
        Self {
            cloud_compute_usd: 0.0,
            storage_usd: 0.0,
            network_egress_usd: 0.0,
            cost_per_million_ops: 0.0,
        }
    }
}

impl Default for OperationalCosts {
    fn default() -> Self {
        Self {
            mttr_minutes: 0.0,
            incidents_per_month: 0.0,
            overhead_percentage: 0.0,
            monitoring_usd: 0.0,
        }
    }
}

impl Default for DevelopmentCosts {
    fn default() -> Self {
        Self {
            loc: 0,
            cyclomatic_complexity: 0.0,
            code_churn: 0.0,
            onboarding_days: 0.0,
        }
    }
}

impl Default for BusinessImpact {
    fn default() -> Self {
        Self {
            revenue_per_100ms_latency: 0.0,
            csat_score: 0.0,
            sla_compliance: 0.0,
            competitive_advantage: 0.0,
        }
    }
}
