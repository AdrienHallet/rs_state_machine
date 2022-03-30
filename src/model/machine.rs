use crate::model::transition::Transition;
use crate::model::transitionable::Transitionable;
use crate::model::errors::*;

/// Defines the Machine.
#[derive(Debug)]
pub struct Machine {
    /// The transitions of the machine.
    pub transitions: Vec<Transition>,
}

impl Machine {
    /// Creates an empty machine.
    pub fn new() -> Machine {
        Self {
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

    pub fn get_output(&self, state_in: String, event: String) -> Result<String, TransitionError> {
        for transition in &self.transitions {
            if transition.state_in == *state_in && transition.event == *event {
                return Ok(transition.state_out.clone());
            }
        }
        return Err(TransitionError::cannot_apply(state_in, stringify!(event).to_string()));
    }

    pub fn apply(&self, object: &mut dyn Transitionable, event: String) -> Result<String, TransitionError> {
        let output = self.get_output(object.get_state(), event);
        match output {
            Ok(state) => {
                object.set_state(state.clone());
                Ok(state.clone())
            },
            Err(error) => Err(error),
        }
    }
}
