use crate::build::build_wasm;
use anyhow::Result;
use std::path::Path;

pub fn handle_build_command(
    app_path: &Path,
    static_pkg_dir_path: &Path,
    wasm_debug: bool,
) -> Result<()> {
    build_wasm(app_path, static_pkg_dir_path, wasm_debug)
}
