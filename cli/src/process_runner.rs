use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct CommandConfig<'a> {
    pub program_name: &'a str,
    pub args: &'a [&'a str],
    pub current_dir: Option<&'a Path>,
    pub env_vars: Option<HashMap<String, String>>,
    pub inherit_stdio: bool,
}

pub fn run_command(config: CommandConfig) -> Result<()> {
    let mut cmd = Command::new(config.program_name);
    cmd.args(config.args);

    if let Some(dir) = config.current_dir {
        cmd.current_dir(dir);
    }
    if let Some(envs) = config.env_vars {
        cmd.envs(envs);
    }

    if config.inherit_stdio {
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }

    let command_display_str = format!("{} {}", config.program_name, config.args.join(" "));
    println!("Executing: {}", command_display_str);

    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute command: {}", command_display_str))?;

    if !status.success() {
        bail!(
            "Command failed with status {}: {}",
            status,
            command_display_str
        );
    }

    Ok(())
}
