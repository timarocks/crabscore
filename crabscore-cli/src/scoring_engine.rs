//! Complexity-aware scoring engine with bonus system

use crate::complexity::ProjectComplexity;
use crabscore_core::{
    metrics::{CostMetrics, EnergyMetrics, PerformanceMetrics, SafetyMetrics},
    scoring::ScoringEngine,
    CrabScore, IndustryProfile,
};

/// Scoring engine that adapts to project complexity and awards bonuses
pub struct ComplexityAwareScoringEngine {
    base_engine: ScoringEngine,
    complexity: ProjectComplexity,
}

impl ComplexityAwareScoringEngine {
    /// Create a new complexity-aware scoring engine
    pub fn new(profile: IndustryProfile, complexity: ProjectComplexity) -> Self {
        Self {
            base_engine: ScoringEngine::new(profile),
            complexity,
        }
    }

    /// Calculate score with complexity bonuses applied
    pub fn calculate_score(
        &self,
        performance: &PerformanceMetrics,
        energy: &EnergyMetrics,
        cost: &CostMetrics,
        safety: &SafetyMetrics,
    ) -> CrabScore {
        let mut score = self
            .base_engine
            .calculate_score(performance, energy, cost, safety);

        // Apply complexity bonuses
        let bonus = self.calculate_complexity_bonus();
        score.bonuses += bonus;
        score.overall += bonus;

        // Add metadata about the analysis
        score.metadata.measurements.environment.os = std::env::consts::OS.to_string();
        score.metadata.measurements.environment.rust_version = "1.88.0".to_string();

        score
    }

    /// Calculate bonus points based on project complexity and best practices
    fn calculate_complexity_bonus(&self) -> f64 {
        let mut bonus: f64 = 0.0;

        // Small project bonus (encourages Rust philosophy of starting small)
        if self.complexity.total_lines < 100 {
            bonus += 2.0;
        } else if self.complexity.total_lines < 500 {
            bonus += 1.0;
        }

        // Documentation bonus (encourages good practices)
        let doc_ratio = self.complexity.doc_coverage();
        if doc_ratio > 0.2 {
            bonus += 2.0; // Excellent documentation
        } else if doc_ratio > 0.1 {
            bonus += 1.0; // Good documentation
        }

        // Test coverage bonus (encourages testing)
        let test_ratio = self.complexity.test_coverage();
        if test_ratio > 0.8 {
            bonus += 3.0; // Excellent test coverage
        } else if test_ratio > 0.5 {
            bonus += 2.0; // Good test coverage
        } else if test_ratio > 0.2 {
            bonus += 1.0; // Basic test coverage
        }

        // Minimal dependencies bonus (zero-cost abstractions principle)
        if self.complexity.dependency_count == 0 {
            bonus += 3.0; // Self-contained project
        } else if self.complexity.dependency_count < 5 {
            bonus += 2.0; // Minimal dependencies
        } else if self.complexity.dependency_count < 10 {
            bonus += 1.0; // Reasonable dependencies
        }

        // Cap bonus at 10 points to prevent inflation
        bonus.min(10.0)
    }

    /// Get breakdown of earned bonuses for display
    pub fn get_bonus_breakdown(&self) -> Vec<(String, f64)> {
        let mut bonuses = Vec::new();

        // Small project bonus
        if self.complexity.total_lines < 100 {
            bonuses.push(("Small Project Bonus".to_string(), 2.0));
        } else if self.complexity.total_lines < 500 {
            bonuses.push(("Compact Project Bonus".to_string(), 1.0));
        }

        // Documentation bonus
        let doc_ratio = self.complexity.doc_coverage();
        if doc_ratio > 0.2 {
            bonuses.push(("Excellent Documentation".to_string(), 2.0));
        } else if doc_ratio > 0.1 {
            bonuses.push(("Good Documentation".to_string(), 1.0));
        }

        // Test coverage bonus
        let test_ratio = self.complexity.test_coverage();
        if test_ratio > 0.8 {
            bonuses.push(("Excellent Tests".to_string(), 3.0));
        } else if test_ratio > 0.5 {
            bonuses.push(("Good Test Coverage".to_string(), 2.0));
        } else if test_ratio > 0.2 {
            bonuses.push(("Basic Test Coverage".to_string(), 1.0));
        }

        // Dependencies bonus
        if self.complexity.dependency_count == 0 {
            bonuses.push(("Zero Dependencies".to_string(), 3.0));
        } else if self.complexity.dependency_count < 5 {
            bonuses.push(("Minimal Dependencies".to_string(), 2.0));
        } else if self.complexity.dependency_count < 10 {
            bonuses.push(("Reasonable Dependencies".to_string(), 1.0));
        }

        bonuses
    }
}
