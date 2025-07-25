mod args;
mod build;
mod commands;
mod paths;
mod process_runner;
mod server;
mod test_runner;

use anyhow::{Context, Result};
use args::{Cli, Commands as CliCommands};
use clap::Parser;
use paths::get_workspace_root;

use commands::{
    handle_build_command, handle_dev_command, handle_e2e_command, handle_serve_command,
    handle_test_command,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let workspace_root = get_workspace_root().context("Failed to determine workspace root")?;
    println!("Workspace root detected as: {}", workspace_root.display());

    let app_path = workspace_root.join("app");
    let static_dir_path = workspace_root.join("static");
    let static_pkg_dir_path = static_dir_path.join("pkg");

    match cli.command {
        CliCommands::Build { wasm_debug } => {
            handle_build_command(&app_path, &static_pkg_dir_path, wasm_debug)?;
        }
        CliCommands::Serve { port, host } => {
            handle_serve_command(&static_dir_path, &static_pkg_dir_path, port, &host)?;
        }
        CliCommands::Dev {
            port,
            host,
            wasm_debug,
        } => {
            handle_dev_command(
                &app_path,
                &static_dir_path,
                &static_pkg_dir_path,
                port,
                &host,
                wasm_debug,
            )?;
        }
        CliCommands::Test {
            base,
            app,
            browser,
            headless,
        } => {
            handle_test_command(&app_path, base, app, browser, headless)?;
        }
        CliCommands::E2E {
            ui,
            debug,
            headed,
            project,
            playwright_args,
        } => {
            handle_e2e_command(&workspace_root, ui, debug, headed, project, playwright_args)?;
        }
    }
    Ok(())
}
