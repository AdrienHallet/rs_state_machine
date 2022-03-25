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
