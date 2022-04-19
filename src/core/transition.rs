use std::fmt::Debug;

/// Represents a Transition on the [Machine](super::machine::Machine).
#[derive(Clone)]
pub struct Transition {
    /// The event (i.e.: action) to apply to make the transition effective.
    pub event: String,
    /// The input (ingoing) state before the event is applied.
    pub state_in: String,
    /// The output (outgoing) state after the event is applied.
    pub state_out: String,
    /// An optional guard function that allows the transition to happen (if return value is true)
    pub guard: Option<fn() -> bool>,
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
            guard: None,
        }
    }

    /// Adds a guard to the [`Transition`], which is itself returned.
    /// 
    /// * `guard` - An [`Option`] containing a function returning `true` if the transition should be allowed
    /// 
    /// # Example
    /// ```rust
    /// use rs_state_machine::core::transition::Transition;
    /// 
    /// let transition = Transition::new("INPUT".to_string(), "EVENT".to_string(), "OUTPUT".to_string())
    ///                             .with_guard(Some(|| true));
    /// assert!(transition.is_allowed())                     
    /// ```
    pub fn with_guard(mut self, guard: Option<fn() -> bool>) -> Transition {
        self.guard = guard;
        self
    }

    /// Returns the evaluation of ´guard´ function, 
    /// or `true` if there is no guard to the transaction.
    pub fn is_allowed(&self) -> bool {
        match &self.guard {
            None => true,
            Some(function) => function(),
        }
    }

    /// Partially compares the current [`Transition`] to another.
    /// A partial comparison ignores [`None`] values 
    /// 
    /// # Example
    /// ```rust
    /// use rs_state_machine::core::transition::Transition;
    /// 
    /// let transition = Transition::new("INPUT".to_string(), "EVENT".to_string(), "OUTPUT".to_string());
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

impl Debug for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transition")
        .field("event", &self.event)
        .field("state_in", &self.state_in)
        .field("state_out", &self.state_out)
        // .field("guard", &self.guard)
        .finish()
    }
}
