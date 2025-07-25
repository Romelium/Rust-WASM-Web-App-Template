// app/src/event_listeners.rs
use crate::drawing_app::DrawingApp;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlCanvasElement, MouseEvent};

// Type alias for the tuple of event listener closures to improve readability.
type EventListenerClosures = (Closure<dyn FnMut(MouseEvent)>, Closure<dyn FnMut()>);

/// Sets up the mousedown and click event listeners for the canvas and clear button.
/// Returns the closures to be stored in the AppHandle, ensuring they are not dropped.
pub(crate) fn setup_event_listeners(
    app: Rc<DrawingApp>,
    canvas: &HtmlCanvasElement,
    clear_button: &HtmlButtonElement,
) -> Result<EventListenerClosures, JsValue> {
    // --- Mouse Down Listener ---
    let mouse_app_clone = app.clone();
    let on_mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
        let rect = mouse_app_clone.canvas.get_bounding_client_rect();
        // Scale mouse coordinates from CSS pixels to canvas buffer pixels.
        let scale_x = mouse_app_clone.canvas.width() as f64 / rect.width();
        let scale_y = mouse_app_clone.canvas.height() as f64 / rect.height();
        let x = (event.client_x() as f64 - rect.left()) * scale_x;
        let y = (event.client_y() as f64 - rect.top()) * scale_y;
        mouse_app_clone.add_circle_at_point(x, y);
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousedown", on_mouse_down.as_ref().unchecked_ref())?;

    // --- Clear Button Listener ---
    let clear_app_clone = app;
    let on_clear = Closure::wrap(Box::new(move || {
        clear_app_clone.clear_canvas();
    }) as Box<dyn FnMut()>);

    clear_button.add_event_listener_with_callback("click", on_clear.as_ref().unchecked_ref())?;

    Ok((on_mouse_down, on_clear))
}
