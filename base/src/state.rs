// base/src/state.rs
use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// A small tolerance for comparing floating-point numbers.
const FLOAT_COMPARISON_EPSILON: f64 = 1e-9;

/// Represents a single shape to be drawn on the canvas.
/// PartialEq is no longer derived, it will be implemented manually.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shape {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: String,
}

/// Manual implementation of PartialEq to handle floating-point comparisons.
impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        // Compare floating-point fields within a small tolerance (epsilon).
        (self.x - other.x).abs() < FLOAT_COMPARISON_EPSILON
            && (self.y - other.y).abs() < FLOAT_COMPARISON_EPSILON
            && (self.radius - other.radius).abs() < FLOAT_COMPARISON_EPSILON
            // Compare the string field directly.
            && self.color == other.color
    }
}

/// Represents the entire state of the drawing application.
/// This can still derive PartialEq, as it will now use the custom
/// implementation for the `Shape` struct within its `shapes` vector.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DrawingState {
    pub shapes: Vec<Shape>,
}

impl DrawingState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new shape with a random color and radius at the given coordinates.
    pub fn add_shape(&mut self, x: f64, y: f64) {
        let mut rng = thread_rng();
        let radius = rng.gen_range(10.0..=50.0);
        let r: u8 = rng.gen_range(100..=255);
        let g: u8 = rng.gen_range(100..=255);
        let b: u8 = rng.gen_range(100..=255);
        let color = format!("rgb({}, {}, {})", r, g, b);

        self.shapes.push(Shape {
            x,
            y,
            radius,
            color,
        });
    }

    /// Removes all shapes from the state.
    pub fn clear_shapes(&mut self) {
        self.shapes.clear();
    }
}
