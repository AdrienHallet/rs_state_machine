use crate::core::{machine::Machine, transition::StringTransition, transitionable::Transitionable};

struct Light {
    state: String,
    toggled: bool,
}

impl Transitionable for Light {
    fn get_state(&self) -> String {
        self.state.to_string()
    }

    fn set_state(&mut self, new_state: String) {
        self.state = new_state;
        self.toggled = true;
    }
}

#[test]
fn integration_scenario_one() {
    let mut room_light = Light { state: "OFF".to_string(), toggled: false };
    let mut light_switch = Machine::new();
    let turning_on = StringTransition::new("OFF".to_string(), "TURN_ON".to_string(), "ON".to_string())
                                            .with_guard(Some(guardian));

    light_switch.add_transition(turning_on);
    let applied = light_switch.apply(&mut room_light, "TURN_ON".to_string());

    assert!(applied.is_ok());
    assert_eq!("ON", applied.unwrap());
    
}

fn guardian() -> bool {
    true
}