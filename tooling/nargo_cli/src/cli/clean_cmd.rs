use clap::Args;
use std::path::PathBuf;

use nargo::workspace::Workspace;


use crate::errors::CliError;

use super::{NargoConfig, PackageSelection, WorkspaceCommand, LockType}; // Import necessary items from parent module

/// Removes the build artifacts and CRS files.
#[derive(Debug, Clone, Args)]
pub(crate) struct CleanCommand {}

impl WorkspaceCommand for CleanCommand {
    fn package_selection(&self) -> PackageSelection {
        // Clean command typically operates on the entire workspace context
        PackageSelection::All
    }

    fn lock_type(&self) -> LockType {
        // Exclusive lock needed as we are deleting files/directories
        LockType::Exclusive
    }
}

pub(crate) fn run(_args: CleanCommand, workspace: Workspace) -> Result<(), CliError> {
    // 1. Clean target directory
    let target_dir = workspace.target_directory_path();
    if target_dir.exists() {
        if let Err(e) = std::fs::remove_dir_all(&target_dir) {
            return Err(CliError::Generic(format!(
                "Failed to remove target directory {}: {}",
                target_dir.display(),
                e
            )));
        }
        println!("Removed target directory: {}", target_dir.display());
    } else {
        println!("Target directory not found: {}", target_dir.display());
    }

    // 2. Clean project-local crs directory (relative to workspace root_dir)
    let local_crs_dir = workspace.root_dir.join("crs");
    if local_crs_dir.exists() {
        if let Err(e) = std::fs::remove_dir_all(&local_crs_dir) {
            return Err(CliError::Generic(format!(
                "Failed to remove local CRS directory {}: {}",
                local_crs_dir.display(),
                e
            )));
        }
        println!("Removed local CRS directory: {}", local_crs_dir.display());
    } else {
        // Optional: message if not found, but usually clean commands are silent on non-existent optional paths
        // println!("Local CRS directory not found: {}", local_crs_dir.display());
    }

    // 3. Clean global nargo crs cache
    // This path is based on where nargo_toml stores git dependencies ($HOME/nargo/)
    match dirs::home_dir() {
        Some(home_dir) => {
            let global_nargo_dir = home_dir.join("nargo");
            let global_crs_dir = global_nargo_dir.join("crs");

            if global_crs_dir.exists() {
                if let Err(e) = std::fs::remove_dir_all(&global_crs_dir) {
                    return Err(CliError::Generic(format!(
                        "Failed to remove global CRS directory {}: {}",
                        global_crs_dir.display(),
                        e
                    )));
                }
                println!("Removed global CRS directory: {}", global_crs_dir.display());
            } else {
                // Optional: message if not found
                // println!("Global CRS directory not found: {}", global_crs_dir.display());
            }
        }
        None => {
            println!("Could not determine home directory. Skipping clean of global CRS cache.");
        }
    }

    println!("Nargo clean finished.");
    Ok(())
} 