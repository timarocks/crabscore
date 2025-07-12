//! Cross-platform energy monitoring interface

use anyhow::Result;
use crabscore_core::metrics::{
    AlgorithmEfficiency, CarbonEfficiency, EnergyMetrics, HardwareLifecycle, PowerConsumption,
};

/// Collects `EnergyMetrics` for the current host.
#[async_trait::async_trait]
pub trait EnergyMonitor {
    /// Sample power/energy information and return a populated `EnergyMetrics`.
    async fn collect(&self) -> Result<EnergyMetrics>;
}

/// Fallback monitor that returns zeros everywhere (works on any OS).
pub struct NullMonitor;

#[async_trait::async_trait]
impl EnergyMonitor for NullMonitor {
    async fn collect(&self) -> Result<EnergyMetrics> {
        Ok(EnergyMetrics {
            direct_consumption: PowerConsumption::default(),
            carbon_efficiency: CarbonEfficiency::default(),
            hardware_lifecycle: HardwareLifecycle::default(),
            algorithmic_efficiency: AlgorithmEfficiency::default(),
        })
    }
}
