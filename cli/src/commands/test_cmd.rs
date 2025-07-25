use crate::test_runner::{run_app_tests, run_base_tests};
use anyhow::{Context, Result};
use std::path::Path;

pub fn handle_test_command(
    app_path: &Path,
    base: bool,
    app: bool,
    browser: Option<String>,
    headless: bool,
) -> Result<()> {
    let run_base = base || !app;
    let run_app = app || !base;

    if run_base {
        run_base_tests().context("Failed to run base tests")?;
    }

    if run_app {
        if let Some(specific_browser) = browser {
            run_app_tests(app_path, &specific_browser, headless)?;
        } else {
            let browsers = if cfg!(target_os = "macos") {
                vec!["chrome", "firefox", "safari"]
            } else {
                vec!["chrome", "firefox"]
            };
            for b in browsers {
                run_app_tests(app_path, b, headless)?;
            }
        }
    }
    Ok(())
}
