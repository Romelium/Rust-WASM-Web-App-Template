// base/src/lib.rs
// This is the base library, independent of web-specifics.
// It contains data structures, algorithms, and the main application logic.

// Module declarations
pub mod state;
// ... other domain-specific modules

// Re-export key types for easier access from other crates
pub use state::DrawingState;
