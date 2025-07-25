use crate::process_runner::{run_command, CommandConfig};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

pub fn handle_e2e_command(
    workspace_root: &Path,
    ui_mode: bool,
    debug_mode: bool,
    headed_mode: bool,
    project: Option<String>,
    additional_args: Vec<String>,
) -> Result<()> {
    println!("\nRunning Playwright E2E tests...");

    let mut pnpm_script_args: Vec<String> = vec!["test:e2e".to_string()];

    if ui_mode {
        pnpm_script_args.push("--ui".to_string());
    } else if debug_mode {
        pnpm_script_args.push("--debug".to_string());
    } else if headed_mode {
        pnpm_script_args.push("--headed".to_string());
    }

    if let Some(p_name) = project {
        pnpm_script_args.push("--project".to_string());
        pnpm_script_args.push(p_name);
    }

    if !additional_args.is_empty() {
        pnpm_script_args.extend(additional_args);
    }

    let pnpm_script_args_refs: Vec<&str> = pnpm_script_args.iter().map(AsRef::as_ref).collect();

    let cmd_config = CommandConfig {
        program_name: "pnpm",
        args: &pnpm_script_args_refs,
        current_dir: Some(workspace_root),
        env_vars: Some(HashMap::from([("CI".to_string(), "true".to_string())])),
        inherit_stdio: true,
    };

    run_command(cmd_config).context("Failed to execute Playwright command")?;

    println!("\nPlaywright E2E tests passed successfully!");
    Ok(())
}
