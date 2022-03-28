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