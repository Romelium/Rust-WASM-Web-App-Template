use crate::build::build_wasm;
use crate::server::serve_static;
use anyhow::Result;
use std::path::Path;

pub fn handle_dev_command(
    app_path: &Path,
    static_dir_path: &Path,
    static_pkg_dir_path: &Path,
    port: u16,
    host: &str,
    wasm_debug: bool,
) -> Result<()> {
    build_wasm(app_path, static_pkg_dir_path, wasm_debug)?;
    serve_static(static_dir_path, port, host)
}
