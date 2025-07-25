use crate::process_runner::{run_command, CommandConfig};
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::path::Path;

pub fn run_base_tests() -> Result<()> {
    println!("\nRunning base tests...");
    let cmd_config = CommandConfig {
        program_name: "cargo",
        args: &["test", "-p", "base"],
        current_dir: None,
        env_vars: None,
        inherit_stdio: true,
    };
    run_command(cmd_config).context("Failed to run base tests")?;
    println!("base tests passed successfully!");
    Ok(())
}

pub fn run_app_tests(app_path: &Path, browser: &str, headless: bool) -> Result<()> {
    println!(
        "\nRunning app WASM tests with {} (headless: {})",
        browser, headless
    );

    if std::process::Command::new("wasm-pack")
        .arg("--version")
        .output()
        .is_err()
    {
        bail!("wasm-pack not found. Please install it.");
    }

    let mut args = vec!["test"];
    match browser.to_lowercase().as_str() {
        "chrome" => args.push("--chrome"),
        "firefox" => args.push("--firefox"),
        "safari" => args.push("--safari"),
        _ => bail!("Unsupported browser: {}", browser),
    };

    if headless {
        args.push("--headless");
    }

    let cmd_config = CommandConfig {
        program_name: "wasm-pack",
        args: &args,
        current_dir: Some(app_path),
        env_vars: Some(HashMap::from([(
            "RUST_LOG".to_string(),
            "warn".to_string(),
        )])),
        inherit_stdio: true,
    };

    run_command(cmd_config).context("Failed to run app tests")?;
    println!("app tests with {} passed successfully!", browser);
    Ok(())
}
