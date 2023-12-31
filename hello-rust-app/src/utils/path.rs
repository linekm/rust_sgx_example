use super::*;
use anyhow::Context;
use std::{env, path::PathBuf};

pub fn binary_directory() -> Result<PathBuf> {
    let binary_path = env::current_exe()?;
    let binary_dir = binary_path
        .parent()
        .context("Failed to get the current directory.")?;
    Ok(binary_dir.to_path_buf())
}

pub fn project_root_directory() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root_dir = manifest_dir
        .parent()
        .context("Failed to get the project root directory.")?;
    Ok(root_dir.to_path_buf())
}
