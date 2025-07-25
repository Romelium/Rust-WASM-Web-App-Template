use anyhow::Result;
use std::path::PathBuf;

pub fn get_workspace_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get parent of CARGO_MANIFEST_DIR"))
        .map(PathBuf::from)
}
