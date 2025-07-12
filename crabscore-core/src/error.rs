//! Error types for the CrabScore library

use std::io;
use thiserror::Error;

/// Main error type for the CrabScore library
#[derive(Error, Debug)]
pub enum CrabScoreError {
    /// Error during measurement collection
    #[error("Measurement failed: {0}")]
    MeasurementError(String),

    /// Error during code analysis
    #[error("Analysis failed: {0}")]
    AnalysisError(String),

    /// Configuration error
    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Environment detection error
    #[error("Failed to detect environment: {0}")]
    EnvironmentError(String),

    /// Unsupported platform or feature
    #[error("Unsupported: {0}")]
    UnsupportedError(String),

    /// External command execution failed
    #[error("Command '{command}' failed with status {status:?}: {stderr}")]
    CommandError {
        /// The command that caused the error
        command: String,
        /// The exit status code, if available
        status: Option<i32>,
        /// The captured standard error output
        stderr: String,
    },

    /// Invalid input data
    #[error("Invalid input: {0}")]
    ValidationError(String),
}

impl CrabScoreError {
    /// Create a new measurement error
    pub fn measurement<S: Into<String>>(msg: S) -> Self {
        Self::MeasurementError(msg.into())
    }

    /// Create a new analysis error
    pub fn analysis<S: Into<String>>(msg: S) -> Self {
        Self::AnalysisError(msg.into())
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(msg: S) -> Self {
        Self::ConfigError(msg.into())
    }

    /// Create a new unsupported error
    pub fn unsupported<S: Into<String>>(feature: S) -> Self {
        Self::UnsupportedError(feature.into())
    }

    /// Create a new validation error
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        Self::ValidationError(msg.into())
    }
}

impl From<std::num::TryFromIntError> for CrabScoreError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::ValidationError(format!("Integer conversion failed: {err}"))
    }
}

impl From<std::string::FromUtf8Error> for CrabScoreError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::ValidationError(format!("UTF-8 conversion failed: {err}"))
    }
}
