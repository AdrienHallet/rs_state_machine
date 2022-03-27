use crate::model::transitionable::Transitionable;

pub struct DummyTransitionable<'obj> {
    pub state: &'obj str,
}

impl<'obj> Transitionable<'obj> for DummyTransitionable<'obj> {
    fn get_state(&self) -> &str {
        self.state
    }

    fn set_state(&mut self, new_state: &'obj str) {
        self.state = new_state;   
    }
}