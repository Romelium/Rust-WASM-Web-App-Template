# JavaScript Logic

## Purpose

The JavaScript code in this directory is the **orchestration layer** that loads the WASM module and connects it to the HTML document. It handles tasks that are either impossible or impractical to perform from within the WASM module itself.

## Philosophy

This layer should be as "thin" as possible. Its main job is to act as glue, not to contain significant application logic. The heavy lifting (state management, computation) is delegated to the Rust/WASM module.

### What Belongs Here?

- **WASM Initialization**: The initial code that imports and initializes the WASM module.
- **Starting the Application**: Calling a single entry-point function in Rust (e.g., `mount_app().start()`). The JavaScript layer is completely unaware of the DOM structure, as it doesn't need to find or pass any element IDs to the WASM module.

### What Does Not Belong Here?

- **The Render Loop**: The `requestAnimationFrame` loop is managed within the Rust/WASM module to keep rendering logic tightly coupled with the application state and to maximize performance. JavaScript's role is only to start this loop.
- **Complex State Management**: The application's "source of truth" should reside within the Rust `base` state, managed by the WASM module. JavaScript should query the WASM module for state, not maintain its own separate, complex state.
- **Heavy Computation**: Any performance-critical calculations or data processing should be implemented in Rust and executed in WASM.
- **DOM Event Listener Setup**: To keep logic tightly coupled with the state it modifies, event listeners (like `click` or `mousedown`) are set up within the Rust/WASM `app` crate.
