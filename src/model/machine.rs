use crate::model::transition::Transition;
use crate::model::errors::*;

/// Defines the Machine.
#[derive(Debug)]
pub struct Machine {
    /// The states of the machine.
    pub states: Vec<&'static str>,
    /// The transitions of the machine.
    pub transitions: Vec<Transition>,
}

impl Machine {
    /// Creates an empty machine.
    pub fn new() -> Machine {
        Self {
            states: vec![],
            transitions: vec![],
        }
    }

    pub fn add_transition(&mut self, transition: Transition) {
        if self.transitions.contains(&transition) {
            panic!("{}", TransitionError::new(TransitionErrorType::AlreadyExists, transition))
        } else {
            self.transitions.push(transition);
        }
    }
}
