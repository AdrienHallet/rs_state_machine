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

    /// Partially compares the current [`Transition`] to another.
    /// A partial comparison ignores [`None`] values 
    /// 
    /// # Example
    /// ```rust
    /// use rs_state_machine::core::transition::Transition;
    /// 
    /// let transition = Transition { event: "EVENT".to_string(), state_in: "INPUT".to_string(), state_out: "OUTPUT".to_string() };
    /// assert!(transition.partial_compare(None, Some(&"EVENT".to_string()), None))
    /// ```
    pub fn partial_compare(&self, input: Option<&String>, event: Option<&String>, output: Option<&String>) -> bool {
        (event.is_none() || &self.event == event.unwrap())
        && (input.is_none() || &self.state_in == input.unwrap())
        && (output.is_none() || &self.state_out == output.unwrap())
    }

}

impl PartialEq for Transition {
    fn eq(&self, other: &Transition) -> bool {
        self.event == other.event
            && self.state_in == other.state_in
            && self.state_out == other.state_out
    }
}
