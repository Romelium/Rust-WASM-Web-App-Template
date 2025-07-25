// app/src/rendering.rs
use crate::{
    drawing_app::DrawingApp,
    utils::{request_animation_frame, RenderLoopHandle},
};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console;

/// Creates the render loop closure and kicks off the first animation frame.
/// Returns the handle to the closure so it can be stored and kept alive.
pub(crate) fn start_render_loop(app: Rc<DrawingApp>) -> Result<RenderLoopHandle, JsValue> {
    // This is a common pattern for creating a recursive closure for requestAnimationFrame.
    // 1. Create a `Rc<RefCell<Option<Closure>>>` that will hold the closure.
    let f: RenderLoopHandle = Rc::new(RefCell::new(None));
    let g = f.clone();

    // 2. Create the closure, which captures the `App` state and the `Rc` of itself.
    let app_clone = app;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_timestamp: f64| {
        app_clone.resize_canvas();
        if let Err(e) = app_clone.render_frame() {
            console::error_1(&e);
        }

        // Schedule the next frame. The closure is re-used.
        // The unwrap is safe because `f` is never set to `None` after this block.
        let handle = request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
        *app_clone._animation_frame_id.borrow_mut() = Some(handle);
    }) as Box<dyn FnMut(f64)>));

    // 3. Kick off the first frame.
    request_animation_frame(g.borrow().as_ref().unwrap())?;

    // 4. Return the handle to the AppHandle to keep it and the closure alive.
    Ok(g)
}
