use crate::core::transition::Transition;
use crate::core::transitionable::Transitionable;
use crate::core::errors::*;

/// Defines the Machine.
#[derive(Debug, Default)]
pub struct Machine<S, E> 
where 
    S: PartialEq + Clone,
    E: PartialEq + Clone,
{
    /// The transitions of the machine.
    pub transitions: Vec<Transition<S, E>>,
}

impl<S, E> Machine<S, E> 
where 
    S: PartialEq + Clone,
    E: PartialEq + Clone,
{

    /// Creates an empty machine.
    pub fn new() -> Machine<S, E> {
        Self {
            transitions: vec![],
        }
    }

    /// Registers a new [Transition] in the [Machine].
    /// 
    /// # Panics
    /// 
    /// Panics if the given [Transition]:
    /// * is already present in the [Machine]
    /// * has the same input and event as another, preventing to decide which output is selected
    pub fn add_transition(&mut self, transition: Transition<S, E>) {
        if self.transitions.contains(&transition) {
            panic!("Transition already exists") // todo: TransitionError::new(TransitionErrorType::AlreadyExists, transition))
        } else if self.transitions.iter().any(|trans| trans.partial_compare(Some(&transition.state_in), Some(&transition.event), None)) {
            panic!("Non-deterministic transition") // todo TransitionError::new(TransitionErrorType::NondeterministicTransition, transition))
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
    pub fn get_output(&self, input_state: &S, event: &E) -> Result<&S, &str> {
        for transition in &self.transitions {
            if transition.state_in == *input_state && transition.event == *event {
                if transition.is_allowed() {
                    return Ok(&transition.state_out);
                } else {
                    return Err("Transition not allowed") // Todo better error description
                }
                
            }
        }
        Err("Cannot apply") // todo better error description
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
    pub fn apply(&self, object: &mut impl Transitionable<S>, event: E) -> Result<&S, &str> {
        let output = self.get_output(&object.get_state(), &event);
        match output {
            Ok(state) => {
                object.set_state(state.clone());
                Ok(state)
            },
            Err(error) => Err(error),
        }
    }
}
