use crate::model::state::State;

#[derive(Debug)]
pub struct Transition {
    event: &'static str,
    state_in: State,
    state_out: State,
}

impl Transition {
    pub fn new(input: &'static str, event: &'static str, output: &'static str) -> Transition {
        Self {
            event,
            state_in: State { name: input },
            state_out:  State { name: output },
        }
    }
}

impl PartialEq for Transition {
    fn eq(&self, other: &Transition) -> bool {
        self.event == other.event
            && self.state_in == other.state_in
            && self.state_out == other.state_out
    }
}
