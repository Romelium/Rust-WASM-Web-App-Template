use clap::builder::TypedValueParser;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about = "Project Development CLI", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Builds the WASM application
    Build {
        #[clap(long, help = "Build WASM in debug mode (faster, larger output)")]
        wasm_debug: bool,
    },
    /// Serves the static directory
    Serve {
        #[clap(short, long, default_value_t = 8080)]
        port: u16,
        #[clap(long, default_value = "0.0.0.0")]
        host: String,
    },
    /// Builds the WASM app and then serves the static directory
    Dev {
        #[clap(short, long, default_value_t = 8080)]
        port: u16,
        #[clap(long, default_value = "0.0.0.0")]
        host: String,
        #[clap(long, help = "Build WASM in debug mode")]
        wasm_debug: bool,
    },
    /// Runs tests for base (native) and/or app (WASM)
    Test {
        #[clap(long, help = "Only run tests for the base crate")]
        base: bool,
        #[clap(long, help = "Only run tests for the app crate")]
        app: bool,
        #[clap(
            long,
            value_parser = clap::builder::PossibleValuesParser::new(["chrome", "firefox", "safari"])
                .map(|s: String| s.to_lowercase()),
            help = "Specify browser for app tests (chrome, firefox, safari)"
        )]
        browser: Option<String>,
        #[clap(long, help = "Run app tests in headless mode")]
        headless: bool,
    },
    /// Runs Playwright End-to-End (E2E) tests
    E2E {
        #[clap(long, help = "Run Playwright tests in UI mode")]
        ui: bool,
        #[clap(long, help = "Run Playwright tests in debug mode")]
        debug: bool,
        #[clap(long, help = "Specify a Playwright project (e.g., chromium)")]
        project: Option<String>,
        #[clap(long, help = "Run tests in headed mode", conflicts_with_all(&["ui", "debug"]))]
        headed: bool,
        #[clap(last = true, help = "Pass additional arguments to Playwright CLI")]
        playwright_args: Vec<String>,
    },
}
