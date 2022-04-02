use core::fmt;
use super::transition::Transition;

/// Represents an error with a Transition.
#[derive(Debug, Clone)]
pub struct TransitionError {
    /// The type of the error.
    error_type: TransitionErrorType,
    /// The transition that caused the error.
    pub transition: Transition,
}

/// Classifies the types of [TransitionError]s that can happen in a [Machine](super::machine::Machine).
/// 
/// Types are currently only used to specify the display information of an error.
#[derive(Debug, Clone)]
pub enum TransitionErrorType {
    AlreadyExists,
    CannotApply,
}

impl TransitionError {
    
    /// Creates a new [`TransitionError`] from the given:  
    /// * `error_type` - type of the error  
    /// * `transition` - the transition from which the error originates
    pub fn new(error_type: TransitionErrorType, transition: Transition) -> TransitionError {
        Self {
            error_type,
            transition,
        }
    }
    /// Creates a new [`TransitionError`] of type [`TransitionErrorType::CannotApply`] from the given:
    /// * `input` - input state of the error'd transaction
    /// * `event` - event of the error'd transaction
    pub fn cannot_apply(input: String, event: String) -> TransitionError {
        Self {
            error_type: TransitionErrorType::CannotApply,
            transition: Transition::new(input, event, "".to_string()),
        }
    }

}

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            TransitionErrorType::AlreadyExists => {
                write!(f, "Transition {:?} is already defined in this State Machine (no duplicates).", self.transition)
            }
            TransitionErrorType::CannotApply => {
                write!(f, "Cannot apply [event={:?}] on [input_state={:?}] (unknown transition)", self.transition.event, self.transition.state_in)
            }
            //_ => write!(f, "Generic TransitionError with {:?}", self.transition)
        }
    }
}
