//! Energy monitoring for CrabScore

#![warn(missing_docs)]
#![forbid(unsafe_code)]

// Platform-specific energy monitoring not yet implemented for Linux

/// Platform-specific energy monitoring
#[cfg(target_os = "macos")]
pub mod macos;

// Platform-specific energy monitoring not yet implemented for Windows

/// Cross-platform energy monitoring interface
pub mod interface;
