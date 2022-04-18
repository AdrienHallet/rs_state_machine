use core::fmt;
use super::transition::Transition;

/// Represents an error with a Transition.
#[derive(Debug)]
pub struct TransitionError<S, E> 
where
    S: PartialEq,
    E: PartialEq,
{
    /// The type of the error.
    error_type: TransitionErrorType,
    /// The transition that caused the error.
    pub transition: Transition<S, E>,
}

/// Classifies the types of [TransitionError]s that can happen in a [Machine](super::machine::Machine).
/// 
/// Types are currently only used to specify the display information of an error.
#[derive(Debug, Clone)]
pub enum TransitionErrorType {
    // Definition Errors

    /// The transition already exists in the [Machine](super::machine::Machine).
    AlreadyExists,
    /// Another transition with the same input & event exists, thus preventing to ensure which output state will be selected.
    NondeterministicTransition,

    // Running Errors

    /// The transition cannot be applied. Can be explained by a missing transition definition, wrong event, wrong input state.
    CannotApply,
    /// The transition's guard does not allow the transition to be applied.
    NotAllowed,
}

impl<S, E> TransitionError<S, E>
where
    S: PartialEq + Copy,
    E: PartialEq + Copy,
{
    
    /// Creates a new [`TransitionError`] from the given:  
    /// * `error_type` - type of the error  
    /// * `transition` - the transition from which the error originates
    pub fn new(error_type: TransitionErrorType, transition: Transition<S, E>) -> TransitionError<S, E> {
        Self {
            error_type,
            transition,
        }
    }

    /// Creates a new [`TransitionError`] of type [`TransitionErrorType::CannotApply`] from the given:
    /// * `input` - input state of the error'd transaction
    /// * `event` - event of the error'd transaction
    pub fn cannot_apply(input: S, event: E) -> TransitionError<S, E> {
        Self {
            error_type: TransitionErrorType::CannotApply,
            transition: Transition::new(input, event, input), // todo should not re-use input
        }
    }

}

impl<S, E> fmt::Display for TransitionError<S, E>
where
    S: PartialEq,
    E: PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            TransitionErrorType::AlreadyExists => {
                write!(f, "Transition {:?} is already defined in this State Machine (no duplicates).", "TODO") // todo self.transition)
            }
            TransitionErrorType::NondeterministicTransition => {
                write!(f, "Two different transitions exist with [input={:?}, event={:?}], leading to non-deterministic state machine", "TODO", "TODO") // todo  self.transition.state_in, self.transition.event)
            }
            TransitionErrorType::CannotApply => {
                write!(f, "Cannot apply [event={:?}] on [input_state={:?}] (unknown transition)", "TODO", "TODO") // todo  self.transition.event, self.transition.state_in)
            }
            TransitionErrorType::NotAllowed => {
                write!(f, "Cannot apply [transition={:?}], the guard function does not allow it", "TODO") // todo  self.transition)
            }
            //_ => write!(f, "Generic TransitionError with {:?}", self.transition)
        }
    }
}
