// app/src/drawing_app.rs
use base::DrawingState;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

/// The internal struct containing the application's state and core logic.
/// It is not exposed directly to wasm-bindgen.
pub struct DrawingApp {
    pub(crate) state: RefCell<DrawingState>,
    pub(crate) canvas: HtmlCanvasElement,
    pub(crate) context: RefCell<Option<CanvasRenderingContext2d>>,
    // We can store the animation frame ID to be able to cancel it if needed.
    pub(crate) _animation_frame_id: RefCell<Option<i32>>,
}

/// Implementation of the core application logic. These methods are not
/// directly exposed to wasm-bindgen; they are called by the `AppHandle` or render loop.
impl DrawingApp {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        console::log_1(&"DrawingApp instance created.".into());
        Self {
            state: RefCell::new(DrawingState::new()),
            canvas,
            context: RefCell::new(None),
            _animation_frame_id: RefCell::new(None),
        }
    }

    pub fn initialize_renderer(&self) -> Result<(), JsValue> {
        let context = self
            .canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Failed to get 2D context"))?
            .dyn_into::<CanvasRenderingContext2d>()?;

        *self.context.borrow_mut() = Some(context);
        Ok(())
    }

    /// Resizes the canvas's drawing buffer to match its CSS-defined client dimensions.
    /// This should be called every frame to handle responsive layout changes.
    pub fn resize_canvas(&self) {
        let rect = self.canvas.get_bounding_client_rect();
        let new_width = rect.width().round() as u32;
        let new_height = rect.height().round() as u32;

        if self.canvas.width() != new_width || self.canvas.height() != new_height {
            self.canvas.set_width(new_width);
            self.canvas.set_height(new_height);
        }
    }

    pub fn render_frame(&self) -> Result<(), JsValue> {
        let context_opt = self.context.borrow();
        let context = context_opt
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Rendering context not initialized"))?;

        // Clear the canvas
        context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        // Draw all shapes from the base state
        let state = self.state.borrow();
        for shape in &state.shapes {
            context.begin_path();
            context.arc(
                shape.x,
                shape.y,
                shape.radius,
                0.0,
                2.0 * std::f64::consts::PI,
            )?;
            context.set_fill_style_str(&shape.color);
            context.fill();
        }

        Ok(())
    }

    pub fn add_circle_at_point(&self, x: f64, y: f64) {
        console::log_1(&format!("Adding circle at ({}, {})", x, y).into());
        self.state.borrow_mut().add_shape(x, y);
    }

    pub fn clear_canvas(&self) {
        console::log_1(&"Clearing all shapes.".into());
        self.state.borrow_mut().clear_shapes();
    }

    pub fn get_drawing_state(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&*self.state.borrow())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
