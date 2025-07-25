use crate::server::serve_static;
use anyhow::Result;
use std::path::Path;

pub fn handle_serve_command(
    static_dir_path: &Path,
    static_pkg_dir_path: &Path,
    port: u16,
    host: &str,
) -> Result<()> {
    if !static_pkg_dir_path.join("app.js").exists()
        || !static_pkg_dir_path.join("app_bg.wasm").exists()
    {
        println!(
            "Warning: WASM output files not found in '{}'.",
            static_pkg_dir_path.display()
        );
        println!("Consider running 'cargo run --bin cli -- build' first.");
    }
    serve_static(static_dir_path, port, host)
}
