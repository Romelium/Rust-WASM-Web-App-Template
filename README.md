# Rust+WASM Web App Template

A high-performance, interactive web application built with Rust and WebAssembly.

## Overview

This project provides a robust framework for building complex web applications where performance and state management are critical. The base logic is written in pure Rust, compiled to WebAssembly (WASM), and exposed to a lightweight JavaScript frontend.

- **`base`**: A pure Rust library containing the application's core data structures and business logic.
- **`app`**: A Rust library that compiles to WASM. It acts as the bridge between the `base` logic and the browser environment, using `wasm-bindgen` and `web-sys`.
- **`cli`**: A command-line tool to streamline development tasks like building, serving, and testing.
- **`static`**: Contains all static web assets, including the HTML, CSS, JS, and the compiled WASM module.
- **`e2e_tests`**: End-to-End tests written with Playwright.

### Example Application

The template includes a simple drawing application. Users can click on the canvas to add colored circles. A "Clear" button removes all circles. This example demonstrates state management, rendering, and UI interaction between JavaScript and Rust/WASM.

## Getting Started

<!-- SETUP_START -->
### First-Time Setup (Important!)

Before you begin, run the interactive setup script to personalize this template. This will rename the project, update dependencies, and configure everything for you.

```bash
python setup.py
```

Follow the on-screen instructions. The script will clean up and delete itself upon successful completion.
<!-- SETUP_END -->
### Prerequisites

1.  **Rust Toolchain:** Install from [rustup.rs](https://rustup.rs/).
2.  **`wasm-pack`:** For building the WASM module.
    ```bash
    cargo install wasm-pack
    ```
3.  **Node.js and pnpm:** For managing frontend dependencies and running scripts.
    ```bash
    # Install Node.js (v18+ recommended) from nodejs.org
    # Then install pnpm
    npm install -g pnpm
    ```

### Installation

1.  **Install Node.js dependencies:**
    ```bash
    pnpm install
    ```
2.  **Install Playwright browsers:**
    ```bash
    pnpm run install-playwright-browsers
    ```

### Development

To build the WASM application and start the local development server:

```bash
cargo run --bin cli -- dev
```

This will build the `app` crate in debug mode and serve the `static` directory on `http://localhost:8080`.

### Running Tests

- **Run all tests (base, App, E2E):**
  ```bash
  # Run base (native) tests
  cargo run --bin cli -- test --base

  # Run app (WASM) tests in headless browsers
  cargo run --bin cli -- test --app --headless

  # Run E2E (Playwright) tests
  cargo run --bin cli -- e2e
  ```

- **Run specific test suites:** See `cargo run --bin cli -- --help` for more options.

## Building for Production

To build the WASM application in release mode (optimized for size and speed):

```bash
cargo run --bin cli -- build
```

The optimized output will be placed in `static/pkg/`. You can then serve the `static` directory with any web server.
