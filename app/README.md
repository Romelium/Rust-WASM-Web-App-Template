# App (WASM) Crate

## Purpose

This crate serves as the **bridge** between the pure Rust `base` logic and the browser's JavaScript environment. Its primary responsibility is to expose the application's functionality to the web, and it is compiled to WebAssembly (WASM).

## Philosophy

The `app` crate consumes the `base` crate as a dependency. It contains the logic that **cannot** be placed in `base` because it requires direct interaction with web APIs. This is where `wasm-bindgen` and `web-sys` are used to communicate with the browser.

### What Belongs Here?

- **The Main App Struct**: A primary struct (e.g., `WebApp`) that holds an instance of the `base` state (e.g., `base::AppState`).
- **WASM-Exported Functions**: Functions decorated with `#[wasm_bindgen]` that serve as the public API for the JavaScript side. The main entry point function (e.g., `mount_app`) is responsible for finding its container element in the DOM and mounting the entire application, making the JavaScript loader completely agnostic to the page structure.
- **Browser API Interaction**: Logic that directly uses `web-sys` to interact with browser features. Examples include:
    - Accessing `window`, `document`, or `console`.
    - Performing rendering operations on an HTML `<canvas>`, setting up DOM event listeners (e.g., for clicks), and managing the `requestAnimationFrame` render loop using `wasm-bindgen`'s `Closure` API.
    - Handling JavaScript-specific data types like `JsValue`.
- **Serialization**: Handling the serialization of `base` data structures to be passed to JavaScript, and deserialization of data received from JavaScript.

### What Does Not Belong Here?

- **Pure Business Logic**: If a piece of logic doesn't need web APIs, it should be moved to the `base` crate.
