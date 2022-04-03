use crate::model::transitionable::Transitionable;

pub struct DummyTransitionable {
    pub state: String,
}

impl Transitionable for DummyTransitionable {
    fn get_state(&self) -> String {
        self.state.to_string()
    }

    fn set_state(&mut self, new_state: String) {
        self.state = new_state;   
    }
}

#[test]
fn should_get_state() {
    let state = "INPUT";
    let transitionable = DummyTransitionable{ state: state.to_string() };
    assert_eq!(state.to_string(), transitionable.get_state());
}

#[test]
fn should_set_state() {
    let state_before = "INPUT";
    let state_after  = "OUTPUT";
    let mut transitionable = DummyTransitionable{ state: state_before.to_string() };

    assert_eq!(state_before.to_string(), transitionable.get_state());
    transitionable.set_state(state_after.to_string());
    assert_eq!(state_after.to_string(), transitionable.get_state());
}