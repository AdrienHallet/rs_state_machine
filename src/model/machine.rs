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

    /// Registers a new [Transition] in the [Machine].
    /// 
    /// # Panics
    /// 
    /// Panics if the given [Transition] is already present in the [Machine].
    pub fn add_transition(&mut self, transition: Transition) {
        if self.transitions.contains(&transition) {
            panic!("{}", TransitionError::new(TransitionErrorType::AlreadyExists, transition))
        } else {
            self.transitions.push(transition);
        }
    }

    /// Returns the [String] `output_state` for the given `input_state` and `event` based on the [Transition]s
    /// registered in the [Machine].
    /// 
    /// # Errors
    /// 
    /// Errors if `event` cannot be aplied on `input_state` (no matching transition).
    /// 
    pub fn get_output(&self, input_state: String, event: String) -> Result<String, TransitionError> {
        for transition in &self.transitions {
            if transition.state_in == *input_state && transition.event == *event {
                return Ok(transition.state_out.clone());
            }
        }
        return Err(TransitionError::cannot_apply(input_state, stringify!(event).to_string()));
    }

    /// Returns the [String] `output_state` for the given:
    /// * `object` - a [Transitionable] with a current state
    /// * `event` - an event to apply
    /// 
    /// The new (output) state will be set in `object`.
    /// 
    /// # Errors
    /// 
    /// Errors if `event` cannot be applied on the current state of `object`
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
