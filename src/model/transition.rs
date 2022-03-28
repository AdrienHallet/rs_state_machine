#[derive(Debug, Clone)]
pub struct Transition {
    pub event: String,
    pub state_in: String,
    pub state_out: String,
}

impl Transition {
    pub fn new(input: String, event: String, output: String) -> Transition {
        Self {
            event,
            state_in: input,
            state_out:  output,
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
