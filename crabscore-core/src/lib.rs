//! Core functionality for CrabScore - The Rust Efficiency Standard
//!
//! This crate provides the fundamental data structures and algorithms for
//! calculating and analyzing software efficiency metrics in Rust projects.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod analysis;
pub mod error;
pub mod metrics;
pub mod profiles;
pub mod scoring;

pub use profiles::ProfileWeights;

/// Main result type for the CrabScore library
pub type Result<T> = std::result::Result<T, error::CrabScoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_industry_profile_weights() {
        let profile = IndustryProfile::WebServices;
        let weights = profile.weights();
        assert!((weights.performance + weights.energy + weights.cost - 1.0).abs() < f64::EPSILON);
    }
}

/// Represents a complete CrabScore assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrabScore {
    /// Overall score (0-100)
    pub overall: f64,
    /// Performance component score (0-100)
    pub performance: f64,
    /// Energy efficiency component score (0-100)
    pub energy: f64,
    /// Cost efficiency component score (0-100)
    pub cost: f64,
    /// Bonus points from various optimizations
    pub bonuses: f64,
    /// Certification level achieved
    pub certification: Certification,
    /// When this score was calculated
    pub timestamp: DateTime<Utc>,
    /// Additional metadata about the score
    pub metadata: ScoreMetadata,
}

/// Metadata about the score calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreMetadata {
    /// Name of the project
    pub project_name: String,
    /// Project version
    pub version: String,
    /// Profile used for scoring
    pub profile: IndustryProfile,
    /// Summary of measurements taken
    pub measurements: MeasurementSummary,
}

/// Summary of measurements used in scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementSummary {
    /// Duration of the measurement period
    pub duration: std::time::Duration,
    /// Number of iterations used in measurements
    pub iterations: u64,
    /// Environment where measurements were taken
    pub environment: Environment,
}

/// Description of the execution environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Operating system
    pub os: String,
    /// CPU model
    pub cpu: String,
    /// Total memory in GB
    pub memory_gb: f32,
    /// Rust version used
    pub rust_version: String,
}

/// Certification levels for CrabScore
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Certification {
    /// No certification
    None,
    /// Basic verification passed
    Verified,
    /// Meets all certification requirements
    Certified,
    /// Top-tier performance and efficiency
    Elite,
    /// Cutting-edge optimizations
    Pioneer,
    /// Exceptional energy efficiency
    Sustainable,
}

/// Industry profile for certification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
}

// Implement default weights for standard profiles
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
        }
    }
}
