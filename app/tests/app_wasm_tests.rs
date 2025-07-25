#![cfg(target_arch = "wasm32")]

use app::{mount_app, AppHandle};
use base::DrawingState;
use wasm_bindgen_test::*;

// Configure wasm-bindgen-test to run in a browser environment.
wasm_bindgen_test_configure!(run_in_browser);

// Helper to create a dummy DOM for tests.
fn setup_dom_and_app() -> AppHandle {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // The ID must match the one hardcoded in `app_wasm.rs`.
    const CONTAINER_ID: &str = "main-app-container";

    // Clean up previous test elements if any
    if let Some(el) = document.get_element_by_id(CONTAINER_ID) {
        el.remove();
    }

    let container = document.create_element("div").unwrap();
    container.set_id(CONTAINER_ID);
    body.append_child(&container).unwrap();

    // The mount_app function will find the container by its hardcoded ID
    // and create the canvas and button inside it.
    mount_app().expect("App mounting should succeed")
}

#[wasm_bindgen_test]
fn test_app_creation_and_initial_state() {
    let app = setup_dom_and_app();
    let state_js = app.get_drawing_state().expect("Should get initial state");
    let state: DrawingState =
        serde_wasm_bindgen::from_value(state_js).expect("Should deserialize initial state");

    assert_eq!(
        state,
        DrawingState::default(),
        "Initial state should be the default (empty)"
    );
    assert!(state.shapes.is_empty());
}

#[wasm_bindgen_test]
fn test_add_circle_modifies_state() {
    let app = setup_dom_and_app();

    // Check initial state
    let initial_state: DrawingState =
        serde_wasm_bindgen::from_value(app.get_drawing_state().unwrap()).unwrap();
    assert_eq!(initial_state.shapes.len(), 0);

    // Simulate an interaction
    app.add_circle_at_point(50.0, 50.0); // Call method directly on the handle

    // Check the new state
    let new_state: DrawingState =
        serde_wasm_bindgen::from_value(app.get_drawing_state().unwrap()).unwrap();
    assert_eq!(
        new_state.shapes.len(),
        1,
        "Shape count should be 1 after adding a circle"
    );
    assert_eq!(new_state.shapes[0].x, 50.0);
    assert_eq!(new_state.shapes[0].y, 50.0);
}

#[wasm_bindgen_test]
fn test_clear_canvas_modifies_state() {
    let app = setup_dom_and_app();

    // Add some shapes first
    app.add_circle_at_point(10.0, 10.0);
    app.add_circle_at_point(20.0, 20.0);

    let state_before_clear: DrawingState =
        serde_wasm_bindgen::from_value(app.get_drawing_state().unwrap()).unwrap();
    assert_eq!(state_before_clear.shapes.len(), 2);

    // Clear the canvas
    app.clear_canvas();

    // Check the final state
    let state_after_clear: DrawingState =
        serde_wasm_bindgen::from_value(app.get_drawing_state().unwrap()).unwrap();
    assert_eq!(
        state_after_clear.shapes.len(),
        0,
        "Shape count should be 0 after clearing canvas"
    );
}
