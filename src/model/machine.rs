use crate::model::transition::Transition;
use crate::model::transitionable::Transitionable;
use crate::model::errors::*;

/// Defines the Machine.
#[derive(Debug)]
pub struct Machine<'machine> {
    /// The states of the machine.
    pub states: Vec<&'machine str>,
    /// The transitions of the machine.
    pub transitions: Vec<Transition<'machine>>,
}

impl<'machine> Machine<'machine> {
    /// Creates an empty machine.
    pub fn new() -> Machine<'machine> {
        Self {
            states: vec![],
            transitions: vec![],
        }
    }

    pub fn add_transition(&mut self, transition: Transition<'machine>) {
        if self.transitions.contains(&transition) {
            panic!("{}", TransitionError::new(TransitionErrorType::AlreadyExists, transition))
        } else {
            self.transitions.push(transition);
        }
    }

    pub fn get_output<'a>(&self, state_in: &'a str, event: &'a str) -> Result<&'a str, TransitionError<'a>> {
        for transition in &self.transitions {
            if transition.state_in == state_in && transition.event == event {
                return Ok(stringify!(transition.state_out));
            }
        }
        return Err(TransitionError::cannot_apply(state_in, stringify!(event)));
    }

    pub fn apply<'a>(&'machine self, object: &'a mut dyn Transitionable<'a>, event: &'a str) -> Result<&'a str, TransitionError<'a>> {
        let output = self.get_output(stringify!(object.get_state()), event);
        if output.is_ok() {
            object.set_state(stringify!(output.unwrap()));
        }
        return output;
    }
}
