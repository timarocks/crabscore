//! Binary discovery with graceful fallbacks

use colored::*;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use tracing::warn;

/// Enhanced binary discovery with graceful fallbacks
pub async fn find_or_build_binary(
    input_path: &Path,
    bin: &Option<String>,
    is_cargo_project: bool,
) -> Option<PathBuf> {
    // 1. Check if user provided a direct binary path
    if let Some(b) = bin.as_ref() {
        let p = Path::new(b);
        if p.is_file() && is_executable(p) {
            return Some(p.to_path_buf());
        }
    }

    // 2. If it's already an executable file
    if input_path.is_file() && is_executable(input_path) {
        return Some(input_path.to_path_buf());
    }

    // 3. Try to build if it's a Cargo project
    if is_cargo_project && input_path.is_dir() {
        println!("{}", "Attempting to build Cargo project...".bright_cyan());

        let mut cmd = Command::new("cargo");
        cmd.arg("build").arg("--release");
        if let Some(bin_name) = bin {
            if !Path::new(bin_name).exists() {
                cmd.arg("--bin").arg(bin_name);
            }
        }

        match cmd.current_dir(input_path).status().await {
            Ok(status) if status.success() => {
                // Try to find the built binary
                let target_dir = input_path.join("target").join("release");

                if let Some(bin_name) = bin {
                    let candidate = target_dir.join(bin_name);
                    if candidate.exists() && is_executable(&candidate) {
                        return Some(candidate);
                    }
                }

                // Scan for any executable
                if let Ok(entries) = std::fs::read_dir(&target_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() && is_executable(&path) {
                            return Some(path);
                        }
                    }
                }
            }
            Ok(_) => {
                warn!("Cargo build failed - continuing with static analysis");
                println!("{}", "Build failed - using static analysis".yellow());
            }
            Err(e) => {
                warn!(
                    "Failed to run cargo build: {} - continuing with static analysis",
                    e
                );
                println!("{}", "Build failed - using static analysis".yellow());
            }
        }
    }

    // 4. Try to find examples or tests as fallback
    if is_cargo_project && input_path.is_dir() {
        // Try building examples
        let examples_dir = input_path.join("examples");
        if examples_dir.exists() {
            println!("{}", "Attempting to build examples...".bright_cyan());
            let mut cmd = Command::new("cargo");
            cmd.arg("build").arg("--examples").arg("--release");

            if let Ok(status) = cmd.current_dir(input_path).status().await {
                if status.success() {
                    let target_examples =
                        input_path.join("target").join("release").join("examples");
                    if let Ok(entries) = std::fs::read_dir(&target_examples) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file() && is_executable(&path) {
                                println!("{}", "Using example binary for analysis".yellow());
                                return Some(path);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Check if a path is an executable file
#[cfg(unix)]
pub fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(metadata) = path.metadata() {
        let permissions = metadata.permissions();
        permissions.mode() & 0o111 != 0
    } else {
        false
    }
}

/// Check if a path is an executable file (Windows)
#[cfg(not(unix))]
pub fn is_executable(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some("exe")
}
