use crate::core::{machine::Machine, transition::Transition, transitionable::Transitionable};

struct Light<'l> {
    state: &'l str,
    toggled: bool,
}

impl<'l> Transitionable<&'l str> for Light<'l> {
    fn get_state(&self) -> &'l str {
        self.state
    }

    fn set_state(&mut self, new_state: &'l str) {
        self.state = new_state;
        self.toggled = true;
    }
}

#[test]
fn integration_scenario_one() {
    let mut room_light = Light { state: "OFF", toggled: false };
    let mut light_switch: Machine<&str, &str> = Machine::new();
    let turning_on = Transition::new("OFF", "TURN_ON", "ON")
                                            .with_guard(Some(guardian));

    light_switch.add_transition(turning_on);
    let applied = light_switch.apply(&mut room_light, "TURN_ON");

    assert!(applied.is_ok());
    assert_eq!("ON", *applied.unwrap());
    
}

fn guardian() -> bool {
    true
}