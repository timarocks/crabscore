//! Industry profiles for CrabScore

use serde::{Deserialize, Serialize};

/// Weights for different aspects of the score
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialOrd)]
pub struct ProfileWeights {
    /// Weight for performance metrics (0.0-1.0)
    pub performance: f64,
    /// Weight for energy metrics (0.0-1.0)
    pub energy: f64,
    /// Weight for cost metrics (0.0-1.0)
    pub cost: f64,
}

impl Default for ProfileWeights {
    fn default() -> Self {
        Self {
            performance: 0.4,
            energy: 0.3,
            cost: 0.3,
        }
    }
}

impl ProfileWeights {
    /// Create a new set of profile weights
    ///
    /// # Panics
    /// Panics if the weights don't sum to 1.0 within a small epsilon
    pub fn new(performance: f64, energy: f64, cost: f64) -> Self {
        let sum = performance + energy + cost;
        assert!(
            (sum - 1.0).abs() < 0.0001,
            "Weights must sum to 1.0, got {sum}"
        );
        Self {
            performance,
            energy,
            cost,
        }
    }
}

// Manually implement PartialEq and Eq for ProfileWeights
// We use a small epsilon for floating point comparison
impl PartialEq for ProfileWeights {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 0.0001;
        (self.performance - other.performance).abs() < EPSILON
            && (self.energy - other.energy).abs() < EPSILON
            && (self.cost - other.cost).abs() < EPSILON
    }
}

impl Eq for ProfileWeights {}

/// Industry-specific profiles with predefined weights
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndustryProfile {
    /// Web services and APIs (40% performance, 30% energy, 30% cost)
    WebServices,
    /// IoT and embedded systems (20% performance, 60% energy, 20% cost)
    IotEmbedded,
    /// Financial applications (50% performance, 20% energy, 30% cost)
    Financial,
    /// Game development (60% performance, 20% energy, 20% cost)
    Gaming,
    /// Enterprise software (30% performance, 30% energy, 40% cost)
    Enterprise,
    /// Custom profile with specific weights
    Custom(ProfileWeights),
}

impl Default for IndustryProfile {
    fn default() -> Self {
        Self::WebServices
    }
}

impl IndustryProfile {
    /// Get the weights for this profile
    pub fn weights(&self) -> ProfileWeights {
        match self {
            Self::WebServices => ProfileWeights::new(0.4, 0.3, 0.3),
            Self::IotEmbedded => ProfileWeights::new(0.2, 0.6, 0.2),
            Self::Financial => ProfileWeights::new(0.5, 0.2, 0.3),
            Self::Gaming => ProfileWeights::new(0.6, 0.2, 0.2),
            Self::Enterprise => ProfileWeights::new(0.3, 0.3, 0.4),
            Self::Custom(weights) => *weights,
        }
    }
}
