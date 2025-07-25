# Base Crate

## Purpose

This crate contains the application's core data structures, state management, and business logic. It is the "brain" of the application, designed to be completely independent of any web-specific or platform-specific code.

## Philosophy

The `base` crate is **pure Rust**. It has no dependencies on `wasm-bindgen`, `web-sys`, or any JavaScript-related libraries. All logic contained within this crate could be compiled and run natively on a server, a desktop application, or any other target that Rust supports.

### What Belongs Here?

- **Data Structures**: The fundamental `struct`s and `enum`s that model your application's domain (e.g., `AppState`, `User`, `Document`).
- **State Management**: Logic for creating, modifying, and querying the application state.
- **Business Logic & Algorithms**: Pure computations and rules that are central to what your application does. If a function only needs to operate on data from `base` and doesn't need to know about the browser, it belongs here.

### Benefits of This Approach

1.  **Testability**: All core logic can be tested with standard, fast, native Rust tests (`cargo test`). There is no need to spin up a browser or a WASM environment.
2.  **Reusability**: The core logic is portable. It can be reused in a different context (e.g., a native GUI, a command-line tool, or a server-side backend) without modification.
3.  **Separation of Concerns**: It creates a clean boundary between the application's fundamental logic and its presentation/integration layer.
