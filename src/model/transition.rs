#[derive(Debug)]
pub struct Transition<'trans> {
    pub event: &'trans str,
    pub state_in: &'trans str,
    pub state_out: &'trans str,
}

impl<'trans> Transition<'trans> {
    pub fn new(input: &'trans str, event: &'trans str, output: &'trans str) -> Transition<'trans> {
        Self {
            event,
            state_in: input,
            state_out:  output,
        }
    }
}

impl PartialEq for Transition<'_> {
    fn eq(&self, other: &Transition) -> bool {
        self.event == other.event
            && self.state_in == other.state_in
            && self.state_out == other.state_out
    }
}
