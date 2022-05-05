use crate::core::transition::Transition;
use crate::core::transitionable::Transitionable;
use crate::core::errors::*;
use std::fmt::Debug;

/// Defines the Machine.
#[derive(Debug, Default)]
pub struct Machine<S, E> 
where 
    S: Eq + Clone + Debug, // Type of the State(s)
    E: Eq + Clone + Debug, // Type of the Event(s)
{
    /// The transitions of the machine.
    pub transitions: Vec<Transition<S, E>>,
}

impl<S, E> Machine<S, E> 
where 
    S: Eq + Clone + Copy + Debug,
    E: Eq + Clone + Copy + Debug,
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
            panic!("{}", TransitionError::already_exists(transition.state_in, transition.event, transition.state_out))
        } else if self.transitions.iter().any(|trans| trans.partial_compare(Some(&transition.state_in), Some(&transition.event), None)) {
            panic!("{}", TransitionError::nondeterministic(transition.state_in, transition.event, transition.state_out))
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
    pub fn get_output(&self, input_state: &S, event: &E) -> Result<&S, TransitionError<S, E>> {
        for transition in &self.transitions {
            if transition.state_in == *input_state && transition.event == *event {
                if transition.is_allowed() {
                    return Ok(&transition.state_out);
                } else {
                    return Err(TransitionError::not_allowed(transition.state_in, transition.event, transition.state_out))
                }
                
            }
        }
        Err(TransitionError::cannot_apply(*input_state, *event))
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
    pub fn apply(&self, object: &mut impl Transitionable<S>, event: E) -> Result<&S, TransitionError<S, E>> {
        let output = self.get_output(&object.get_state(), &event);
        match output {
            Ok(state) => {
                object.set_state(*state);
                Ok(state)
            },
            Err(error) => Err(error),
        }
    }
}