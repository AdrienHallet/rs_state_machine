use crate::model::state::State;

#[derive(Debug)]
pub struct Transition {
    name: &'static str,
    state_in: State,
    state_out: State,
}