use core::fmt;

use super::transition::Transition;

#[derive(Debug)]
pub struct TransitionError {
    error_type: TransitionErrorType,
    pub transition: Transition,
}

#[derive(Debug)]
pub enum TransitionErrorType {
    AlreadyExists,
    CannotApply,
}

impl TransitionError {
    pub fn new(error_type: TransitionErrorType, transition: Transition) -> TransitionError {
        Self {
            error_type,
            transition,
        }
    }
    pub fn cannot_apply(input: &'static str, event: &'static str) -> TransitionError {
        Self {
            error_type: TransitionErrorType::CannotApply,
            transition: Transition::new(input, event, ""),
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
