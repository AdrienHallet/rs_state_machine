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
}

impl TransitionError {
    pub fn new(error_type: TransitionErrorType, transition: Transition) -> TransitionError {
        Self {
            error_type,
            transition,
        }
    }
}

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            TransitionErrorType::AlreadyExists => {
                write!(f, "Transition {:?} is already defined in this State Machine (no duplicates).", self.transition)
            }
            // _ => write!(f, "Generic TransitionError with {:?}", self.transition)
        }
    }
}
