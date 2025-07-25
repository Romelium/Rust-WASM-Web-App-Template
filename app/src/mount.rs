// app/src/mount.rs
use crate::{app_handle::AppHandle, dom_creation, drawing_app::DrawingApp, event_listeners};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// The main entry point from JavaScript. It finds the container element,
/// creates the application's DOM structure inside it, sets up event listeners,
/// and returns a handle to the running application.
#[wasm_bindgen]
pub fn mount_app() -> Result<AppHandle, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    // 1. Create the DOM elements for the app.
    let (canvas, clear_button) = dom_creation::create_app_dom(&document)?;

    // 2. Initialize the core application state.
    let app = Rc::new(DrawingApp::new(canvas.clone()));

    // 3. Set up event listeners and get the closures to keep them alive.
    let (on_mouse_down, on_clear) =
        event_listeners::setup_event_listeners(app.clone(), &canvas, &clear_button)?;

    // 4. Create the handle that will be returned to JavaScript.
    let app_handle = AppHandle {
        app,
        _on_mouse_down: on_mouse_down,
        _on_clear: on_clear,
        _render_loop: RefCell::new(None),
    };

    Ok(app_handle)
}
