use crate::{model::{machine::Machine, transitionable::Transitionable}, define};
use super::transitionable_test::DummyTransitionable;

#[test]
fn should_not_apply_unexisting_event() {
    let empty_machine = Machine::new();
    let mut transitionable = DummyTransitionable{ state: "INPUT".to_string() };
    
    let result = empty_machine.apply(&mut transitionable, "NON_EXISTING_TRANSITION".to_string());
    assert!(result.is_err());
}

#[test]
fn should_get_output() {
    let light_switch = define!(
        "OFF" - "TURN_ON"  -> "ON",
        "ON"  - "TURN_OFF" -> "OFF"
    );

    let output = light_switch.get_output("OFF".to_string(), "TURN_ON".to_string());

    assert!(output.is_ok());
    assert_eq!("ON", output.unwrap());
}

#[test]
fn should_apply() {
    let light_switch = define!(
        "OFF" - "TURN_ON"  -> "ON",
        "ON"  - "TURN_OFF" -> "OFF"
    );

    let mut light: DummyTransitionable = DummyTransitionable { state: "OFF".to_string() };

    let output = light_switch.apply(&mut light, "TURN_ON".to_string());

    assert!(output.is_ok());
    assert_eq!("ON", output.unwrap());
    assert_eq!("ON", light.get_state());
}

#[test]
#[should_panic]
fn should_not_allow_twice_same_transition() {
    let _machine_with_duplicate = define!(
        "OFF" - "TURN_ON" -> "ON",
        "OFF" - "TURN_ON" -> "ON"
    );
}

#[test]
#[should_panic]
fn should_not_allow_nondeterministic_transitions() {
    let _machine_with_nondeterminism = define!(
        "OFF" - "TURN_ON" -> "ON",
        "OFF" - "TURN_ON" -> "OFF"
    );
}