use std::fmt;

#[derive(Debug)]
pub struct Machine {
    pub states: Vec<&'static str>,
    pub transitions: Vec<&'static str>,
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "blah")
    }
}

impl Machine {
    pub fn new() -> Machine {
        Self {
            states: vec![],
            transitions: vec![],
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub name: &'static str,
}

#[derive(Debug)]
pub struct Transition {
    name: &'static str,
    state_in: State,
    state_out: State,
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.name == other.name
    }
}
