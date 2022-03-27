use core::fmt;

use super::transition::Transition;

#[derive(Debug)]
pub struct TransitionError<'err> {
    error_type: TransitionErrorType,
    pub transition: Transition<'err>,
}

#[derive(Debug)]
pub enum TransitionErrorType {
    AlreadyExists,
    CannotApply,
}

impl<'err> TransitionError<'err> {
    pub fn new(error_type: TransitionErrorType, transition: Transition<'err>) -> TransitionError<'err> {
        Self {
            error_type,
            transition,
        }
    }
    pub fn cannot_apply(input: &'err str, event: &'err str) -> TransitionError<'err> {
        Self {
            error_type: TransitionErrorType::CannotApply,
            transition: Transition::new(input, event, ""),
        }
    }

}

impl fmt::Display for TransitionError<'_> {
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
