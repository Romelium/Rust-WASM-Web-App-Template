use crate::process_runner::{run_command, CommandConfig};
use anyhow::{bail, Context, Result};
use std::path::Path;

pub fn build_wasm(crate_path: &Path, out_dir: &Path, debug_mode: bool) -> Result<()> {
    let build_type = if debug_mode { "debug" } else { "release" };
    println!(
        "Building WASM application in {} mode from: {}",
        build_type,
        crate_path.display()
    );
    println!("Output directory: {}", out_dir.display());

    if std::process::Command::new("wasm-pack")
        .arg("--version")
        .output()
        .is_err()
    {
        bail!("wasm-pack not found. Please install it: https://rustwasm.github.io/wasm-pack/installer/");
    }

    let mut args_vec: Vec<&str> = vec![
        "build",
        "--target",
        "web",
        "--out-dir",
        out_dir
            .to_str()
            .context("Output dir path is not valid UTF-8")?,
        "--out-name",
        "app",
    ];

    let features_string_storage: String;

    if debug_mode {
        args_vec.push("--dev");
    } else {
        args_vec.push("--release");
        features_string_storage = "default wee_alloc".to_string();
        args_vec.push("--features");
        args_vec.push(&features_string_storage);
    }

    let cmd_config = CommandConfig {
        program_name: "wasm-pack",
        args: &args_vec,
        current_dir: Some(crate_path),
        env_vars: None,
        inherit_stdio: true,
    };

    run_command(cmd_config).context("wasm-pack build failed")?;

    println!("\nBuild successful! Output is in {}", out_dir.display());
    Ok(())
}
