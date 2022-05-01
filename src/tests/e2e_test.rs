use crate::core::{machine::Machine, transition::Transition, transitionable::Transitionable};

struct Light<'l> {
    state: &'l str,
    toggled: bool,
}

struct StatefulLight {
    state: LightState,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum LightState {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum LightEvent {
    TurnOn,
    TurnOff,
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

impl Transitionable<LightState> for StatefulLight {
    fn get_state(&self) -> LightState {
        self.state
    }

    fn set_state(&mut self, new_state: LightState) {
        self.state = new_state
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

#[test]
fn integration_scenario_enums() {
    let mut enum_light_switch = Machine::new();
    enum_light_switch.add_transition(Transition::new(LightState::Off, LightEvent::TurnOn, LightState::On));
    enum_light_switch.add_transition(Transition::new(LightState::On, LightEvent::TurnOff, LightState::Off));
    let mut state_light = StatefulLight { state: LightState::Off };

    let applied = enum_light_switch.apply(&mut state_light, LightEvent::TurnOn);

    assert!(applied.is_ok());
    assert_eq!(LightState::On, *applied.unwrap());

}

fn guardian() -> bool {
    true
}