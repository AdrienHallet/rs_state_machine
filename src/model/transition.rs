use crate::model::state::State;

#[derive(Debug)]
pub struct Transition {
    name: &'static str,
    state_in: State,
    state_out: State,
}

impl Transition {
    pub fn new(input: &'static str, transition: &'static str, output: &'static str) -> Transition {
        Self {
            name: transition,
            state_in: State { name: input },
            state_out:  State { name: output },
        }
    }
}

impl PartialEq for Transition {
    fn eq(&self, other: &Transition) -> bool {
        self.name == other.name
            && self.state_in == other.state_in
            && self.state_out == other.state_out
    }
}
