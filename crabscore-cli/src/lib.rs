//! Command-line interface for CrabScore

#![warn(missing_docs)]
#![forbid(unsafe_code)]

/// Main CLI module
pub mod cli;

/// Command execution
pub mod command;

/// Project complexity analysis
pub mod complexity;

/// Binary discovery with graceful fallbacks
pub mod binary_discovery;

/// Performance, energy, and cost estimation
pub mod estimation;

/// Complexity-aware scoring engine
pub mod scoring_engine;
