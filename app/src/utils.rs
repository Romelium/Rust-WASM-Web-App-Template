// app/src/utils.rs
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Type alias for the render loop closure to improve readability.
pub type RenderLoopClosure = Closure<dyn FnMut(f64)>;
// Type alias for the handle that owns the closure, allowing for recursion.
pub type RenderLoopHandle = Rc<RefCell<Option<RenderLoopClosure>>>;

// Helper to request the next animation frame.
pub fn request_animation_frame(f: &RenderLoopClosure) -> Result<i32, JsValue> {
    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
}
