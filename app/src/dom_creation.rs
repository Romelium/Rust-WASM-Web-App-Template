// app/src/dom_creation.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlButtonElement, HtmlCanvasElement};

/// The ID of the container element in `index.html` where the app will be mounted.
const CONTAINER_ID: &str = "main-app-container";

/// Creates the application's DOM structure inside the main container.
/// Returns the created canvas and clear button elements.
pub(crate) fn create_app_dom(
    document: &Document,
) -> Result<(HtmlCanvasElement, HtmlButtonElement), JsValue> {
    let container = document.get_element_by_id(CONTAINER_ID).ok_or_else(|| {
        JsValue::from_str(&format!("Container element '{}' not found", CONTAINER_ID))
    })?;

    // --- Create DOM elements ---
    let toolbar = document.create_element("div")?.dyn_into::<Element>()?;
    toolbar.set_id("toolbar");

    let clear_button = document
        .create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;
    clear_button.set_id("clear-btn");
    clear_button.set_text_content(Some("Clear Canvas"));
    toolbar.append_child(&clear_button)?;

    let canvas_container = document.create_element("div")?.dyn_into::<Element>()?;
    canvas_container.set_id("canvas-container");

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_id("drawing-canvas");
    canvas_container.append_child(&canvas)?;

    // Append created elements to the main container
    container.append_child(&toolbar)?;
    container.append_child(&canvas_container)?;

    Ok((canvas, clear_button))
}
