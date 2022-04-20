use std::fmt::Debug;

/// Represents a Transition on the [Machine](super::machine::Machine).
pub struct Transition<S, E> 
where 
    S: PartialEq, // Type of the State(s)
    E: PartialEq, // Type of the Event(s)
{
     /// The event (i.e.: action) to apply to make the transition effective.
    pub event: E,
    /// The input (ingoing) state before the event is applied.
    pub state_in: S,
    /// The output (outgoing) state after the event is applied.
    pub state_out: S,
    /// An optional guard function that allows the transition to happen (if return value is true)
    pub guard: Option<fn() -> bool>,
}

impl<S, E> Transition<S, E> 
where
    S: PartialEq,
    E: PartialEq,
{
    /// Creates a new [`Transition`] from the given:
    /// * `input` - the state before the event
    /// * `event` - the event
    /// * `output`- the state after the event
    pub fn new(input: S, event: E, output: S) -> Transition<S, E> {
        Self {
            event,
            state_in: input,
            state_out:  output,
            guard: None
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
    /// let transition = Transition::new("INPUT", "EVENT", "OUTPUT")
    ///                             .with_guard(Some(|| true));
    /// assert!(transition.is_allowed())                     
    /// ```
    pub fn with_guard(mut self, guard: Option<fn() -> bool>) -> Transition<S, E> {
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
    /// let transition = Transition::new("INPUT", "EVENT", "OUTPUT");
    /// assert!(transition.partial_compare(None, Some(&"EVENT"), None))
    /// ```
    pub fn partial_compare(&self, input: Option<&S>, event: Option<&E>, output: Option<&S>) -> bool {
        (event.is_none() || &self.event == event.unwrap())
        && (input.is_none() || &self.state_in == input.unwrap())
        && (output.is_none() || &self.state_out == output.unwrap())
    }
}

impl<S, E> PartialEq for Transition<S, E> 
where
    S: PartialEq + Debug,
    E: PartialEq + Debug,
{
    fn eq(&self, other: &Transition<S, E>) -> bool {
        self.event == other.event
            && self.state_in == other.state_in
            && self.state_out == other.state_out
    }
}

impl<S, E> Debug for Transition<S, E> 
where 
    S: PartialEq + Debug,
    E: PartialEq + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transition")
        .field("event", &self.event)
        .field("state_in", &self.state_in)
        .field("state_out", &self.state_out)
        // .field("guard", &self.guard)
        .finish()
    }
}
