// base/tests/base_tests.rs
use base::state::DrawingState;

#[test]
fn test_drawing_state_new() {
    let state = DrawingState::new();
    assert!(
        state.shapes.is_empty(),
        "A new DrawingState should have no shapes"
    );
}

#[test]
fn test_add_shape() {
    let mut state = DrawingState::new();
    assert_eq!(state.shapes.len(), 0);

    state.add_shape(10.0, 20.0);
    assert_eq!(state.shapes.len(), 1, "Should have one shape after adding");
    let shape = &state.shapes[0];
    assert_eq!(shape.x, 10.0);
    assert_eq!(shape.y, 20.0);
    assert!(shape.radius >= 10.0 && shape.radius <= 50.0);
    assert!(shape.color.starts_with("rgb("));

    state.add_shape(30.0, 40.0);
    assert_eq!(
        state.shapes.len(),
        2,
        "Should have two shapes after adding another"
    );
}

#[test]
fn test_clear_shapes() {
    let mut state = DrawingState::new();
    state.add_shape(1.0, 1.0);
    state.add_shape(2.0, 2.0);
    assert_eq!(state.shapes.len(), 2);

    state.clear_shapes();
    assert_eq!(
        state.shapes.len(),
        0,
        "Should have zero shapes after clearing"
    );
}

#[test]
fn test_drawing_state_serialization() {
    let mut state = DrawingState::new();
    state.add_shape(10.0, 20.0);

    let json_string = serde_json::to_string(&state).expect("Serialization should succeed");
    let deserialized_state: DrawingState =
        serde_json::from_str(&json_string).expect("Deserialization should succeed");

    assert_eq!(
        state, deserialized_state,
        "State should be equal after a serialization round-trip"
    );
}
