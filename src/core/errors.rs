use std::fmt;
use std::fmt::Debug;
use super::transition::Transition;

/// Represents an error with a Transition.
#[derive(Debug)]
pub struct TransitionError<S, E> 
where
    S: Eq,
    E: Eq,
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
    S: Eq + Copy,
    E: Eq + Copy,
{
    /// Creates a new [`TransitionError`] of type [`TransitionErrorType::AlreadyExists`] from the given:  
    /// * `input` - the input state of the duplicated transition
    /// * `event` - the event of the duplicated transition
    /// * `output` - the output state of the duplicated transition
    pub fn already_exists(input: S, event: E, output: S) -> TransitionError<S, E> {
        Self {
            error_type: TransitionErrorType::AlreadyExists,
            transition: Transition::new(input, event, output),
        }
    }

    /// Creates a new [`TransitionError`] of type [`TransitionErrorType::NondeterministicTransition`] from the given:  
    /// * `input` - the input state of the nondeterministic transition
    /// * `event` - the event of the nondeterministic transition
    /// * `output` - the output state of the nondeterministic transition
    pub fn nondeterministic(input: S, event: E, output: S) -> TransitionError<S, E> {
        Self {
            error_type: TransitionErrorType::NondeterministicTransition,
            transition: Transition::new(input, event, output),
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

    /// Creates a new [`TransitionError`] of type [`TransitionErrorType::NotAllowed`] from the given:  
    /// * `input` - the input state of the not allowed transition
    /// * `event` - the event of the not allowed transition
    /// * `output` - the output state of the not allowed transition
    pub fn not_allowed(input: S, event: E, output: S) -> TransitionError<S, E> {
        Self {
            error_type: TransitionErrorType::NotAllowed,
            transition: Transition::new(input, event, output),
        }
    }

}

impl<S, E> fmt::Display for TransitionError<S, E>
where
    S: Eq + Debug,
    E: Eq + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            TransitionErrorType::AlreadyExists => {
                write!(f, "Transition {:?} is already defined in this State Machine (no duplicates).", self.transition)
            }
            TransitionErrorType::NondeterministicTransition => {
                write!(f, "Two different transitions exist with [input={:?}, event={:?}], leading to non-deterministic state machine", self.transition.state_in, self.transition.event)
            }
            TransitionErrorType::CannotApply => {
                write!(f, "Cannot apply [event={:?}] on [input_state={:?}] (unknown transition)", self.transition.event, self.transition.state_in)
            }
            TransitionErrorType::NotAllowed => {
                write!(f, "Cannot apply [transition={:?}], the guard function does not allow it", self.transition)
            }
            //_ => write!(f, "Generic TransitionError with {:?}", self.transition)
        }
    }
}
