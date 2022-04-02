/// Represents a Transition on the [Machine](super::machine::Machine).
#[derive(Debug, Clone)]
pub struct Transition {
    /// The event (i.e.: action) to apply to make the transition effective.
    pub event: String,
    /// The input (ingoing) state before the event is applied.
    pub state_in: String,
    /// The output (outgoing) state after the event is applied.
    pub state_out: String,
}

impl Transition {
    /// Creates a new [`Transition`] from the given:
    /// * `input` - the state before the event
    /// * `event` - the event
    /// * `output`- the state after the event
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
