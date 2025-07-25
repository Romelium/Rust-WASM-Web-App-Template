// app/src/app_handle.rs
use crate::{drawing_app::DrawingApp, rendering, utils::RenderLoopHandle};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{console, MouseEvent};

/// A handle to the application, designed to be held by JavaScript.
/// It owns the core application logic and all event listener/render loop closures.
/// When this handle is dropped (e.g., by JS garbage collection), the closures
/// are dropped too, automatically cleaning up the event listeners and stopping the loop.
#[wasm_bindgen]
pub struct AppHandle {
    // The core application logic, shared with closures.
    pub(crate) app: Rc<DrawingApp>,
    // Event listener closures are stored here to keep them alive.
    pub(crate) _on_mouse_down: Closure<dyn FnMut(MouseEvent)>,
    pub(crate) _on_clear: Closure<dyn FnMut()>,
    // This holds the handle to the render loop closure. It's in a RefCell
    // because we create it in `start()`, not in the constructor.
    pub(crate) _render_loop: RefCell<Option<RenderLoopHandle>>,
}

/// Public methods exposed to JavaScript via the `AppHandle`.
#[wasm_bindgen]
impl AppHandle {
    /// Initializes the WebGL context for rendering. This must be called after
    /// the canvas is available in the DOM.
    #[wasm_bindgen(js_name = initializeRenderer)]
    pub fn initialize_renderer(&self) -> Result<(), JsValue> {
        self.app.initialize_renderer()
    }

    /// Starts the render loop.
    #[wasm_bindgen(js_name = start)]
    pub fn start(&self) -> Result<(), JsValue> {
        console::log_1(&"Starting render loop...".into());
        let render_loop_handle = rendering::start_render_loop(self.app.clone())?;
        *self._render_loop.borrow_mut() = Some(render_loop_handle);
        Ok(())
    }

    /// Adds a circle to the state. Exposed for testing and potential external calls.
    #[wasm_bindgen(js_name = addCircleAtPoint)]
    pub fn add_circle_at_point(&self, x: f64, y: f64) {
        self.app.add_circle_at_point(x, y);
    }

    /// Clears all shapes. Exposed for testing and potential external calls.
    #[wasm_bindgen(js_name = clearCanvas)]
    pub fn clear_canvas(&self) {
        self.app.clear_canvas();
    }

    /// Serializes and returns the current application state to JavaScript.
    #[wasm_bindgen(js_name = getDrawingState)]
    pub fn get_drawing_state(&self) -> Result<JsValue, JsValue> {
        self.app.get_drawing_state()
    }
}
