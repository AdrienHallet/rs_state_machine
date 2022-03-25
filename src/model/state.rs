#[derive(Debug)]
pub struct State {
    pub name: &'static str,
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.name == other.name
    }
}