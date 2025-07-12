//! Cost provider trait and simple implementations.

use anyhow::Result;
use async_trait::async_trait;
use crabscore_core::metrics::{
    BusinessImpact, CostMetrics, DevelopmentCosts, InfrastructureCosts, OperationalCosts,
};
use std::time::Duration;

/// Provides `CostMetrics` for a project.
#[async_trait]
pub trait CostProvider {
    /// Collect cost metrics given a project root path.
    async fn collect(&self, project_root: &str) -> Result<CostMetrics>;
}

/// A static cost provider that returns user-specified numbers via a JSON file.
/// Useful for on-prem or when cloud APIs are unavailable.
pub struct StaticCostProvider {
    file_path: String,
    #[allow(dead_code)]
    refresh: Duration,
}

impl StaticCostProvider {
    /// Create new provider pointing to a JSON or YAML config.
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
            refresh: Duration::from_secs(300),
        }
    }
}

#[async_trait]
impl CostProvider for StaticCostProvider {
    async fn collect(&self, _project_root: &str) -> Result<CostMetrics> {
        let content = std::fs::read_to_string(&self.file_path)?;
        let v: serde_json::Value = serde_json::from_str(&content)?;
        let infra = v.get("infrastructure").cloned().unwrap_or_default();
        let ops = v.get("operations").cloned().unwrap_or_default();
        let dev = v.get("development").cloned().unwrap_or_default();
        let biz = v.get("business_impact").cloned().unwrap_or_default();

        Ok(CostMetrics {
            infrastructure: InfrastructureCosts {
                cloud_compute_usd: infra
                    .get("cloud_compute_usd")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                storage_usd: infra
                    .get("storage_usd")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                network_egress_usd: infra
                    .get("network_egress_usd")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                cost_per_million_ops: infra
                    .get("cost_per_million_ops")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
            },
            operations: OperationalCosts {
                mttr_minutes: ops
                    .get("mttr_minutes")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                incidents_per_month: ops
                    .get("incidents_per_month")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                overhead_percentage: ops
                    .get("overhead_percentage")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                monitoring_usd: ops
                    .get("monitoring_usd")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
            },
            development: DevelopmentCosts {
                loc: dev.get("loc").and_then(|x| x.as_u64()).unwrap_or(0),
                cyclomatic_complexity: dev
                    .get("cyclomatic_complexity")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                code_churn: dev
                    .get("code_churn")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                onboarding_days: dev
                    .get("onboarding_days")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
            },
            business_impact: BusinessImpact {
                revenue_per_100ms_latency: biz
                    .get("revenue_per_100ms_latency")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                csat_score: biz
                    .get("csat_score")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                sla_compliance: biz
                    .get("sla_compliance")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
                competitive_advantage: biz
                    .get("competitive_advantage")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0),
            },
        })
    }
}
