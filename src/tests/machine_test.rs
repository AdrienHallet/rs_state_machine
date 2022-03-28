use crate::{Machine, model::errors::TransitionError};
use super::transitionable_test::DummyTransitionable;

#[test]
fn should_not_apply_unexisting_event() {
    let empty_machine = Machine::new();
    let mut transitionable = DummyTransitionable{ state: "INPUT".to_string() };
    
    let result = empty_machine.apply(&mut transitionable, "NON_EXISTING_TRANSITION".to_string());
    assert!(result.is_err());
}