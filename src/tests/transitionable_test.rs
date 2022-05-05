use crate::core::transitionable::Transitionable;

pub struct DummyTransitionable<'dt> {
    pub state: &'dt str,
}

impl<'dt> Transitionable<&'dt str> for DummyTransitionable<'dt> {
    fn get_state(&self) -> &'dt str {
        self.state
    }

    fn set_state(&mut self, new_state: &'dt str) {
        self.state = new_state;   
    }
}

#[test]
fn should_get_state() {
    let state = "INPUT";
    let transitionable = DummyTransitionable{ state };
    assert_eq!(state.to_string(), transitionable.get_state());
}

#[test]
fn should_set_state() {
    let state_before = "INPUT";
    let state_after  = "OUTPUT";
    let mut transitionable = DummyTransitionable{ state: state_before };

    assert_eq!(state_before.to_string(), transitionable.get_state());
    transitionable.set_state(state_after);
    assert_eq!(state_after.to_string(), transitionable.get_state());
}