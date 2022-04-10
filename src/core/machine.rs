use std::ops::Index;

use super::transition::Transition;
use super::transitionable::Transitionable;
use super::errors::*;
use super::graph::Graph;

/// Defines the Machine.
#[derive(Debug, Default)]
pub struct Machine {
    /// The transitions of the machine.
    pub graph: Graph,
}

impl Machine {

    /// Creates an empty machine.
    pub fn new() -> Machine {
        Self {
            graph: Graph::new(),
        }
    }

    /// Registers a new [Transition] in the [Machine].
    /// 
    /// # Panics
    /// 
    /// Panics if the given [Transition]:
    /// * is already present in the [Machine]
    /// * has the same input and event as another, preventing to decide which output is selected
    pub fn add_transition(&mut self, transition: Transition) {
        // if self.graph.contains(&transition) {
        //     panic!("{}", TransitionError::new(TransitionErrorType::AlreadyExists, transition))
        // } else if self.transitions.iter().any(|trans| trans.partial_compare(Some(&transition.state_in), Some(&transition.event), None)) {
        //     panic!("{}", TransitionError::new(TransitionErrorType::NondeterministicTransition, transition))
        // } else {
        //     self.transitions.push(transition);
        // }
        self.graph.add_edge(transition.event, transition.state_in, transition.state_out);
    }

    /// Returns the [String] `output_state` for the given `input_state` and `event` based on the [Transition]s
    /// registered in the [Machine].
    /// 
    /// # Errors
    /// 
    /// Errors if `event` cannot be aplied on `input_state` (no matching transition).
    /// 
    pub fn get_output(&self, input_state: String, event: String) -> Result<String, TransitionError> {
        // for transition in &self.transitions {
        //     if transition.state_in == *input_state && transition.event == *event {
        //         return Ok(transition.state_out.clone());
        //     }
        // }
        // Err(TransitionError::cannot_apply(input_state, stringify!(event).to_string()))
        let index = *self.graph.vertices.get(&input_state).unwrap();
        let transitions = &self.graph.adj_matrix.matrix[index];
        let output_index = transitions.iter().position(|el| *el == Some(event.to_string())).unwrap();
        Ok(self.graph.vertices.iter().find(|(key, &val)| val == output_index).unwrap().0.to_string())
    }

    /// Returns the [String] `output_state` for the given:
    /// * `object` - a [Transitionable] with a current state
    /// * `event` - an event to apply
    /// 
    /// The new (output) state will be set in `object`.
    /// 
    /// # Errors
    /// 
    /// Errors if `event` cannot be applied on the current state of `object`
    pub fn apply(&self, object: &mut dyn Transitionable, event: String) -> Result<String, TransitionError> {
        let output = self.get_output(object.get_state(), event);
        match output {
            Ok(state) => {
                object.set_state(state.clone());
                Ok(state)
            },
            Err(error) => Err(error),
        }
    }
}
