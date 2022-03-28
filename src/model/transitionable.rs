pub trait Transitionable {
    fn get_state(&self) -> String;
    
    fn set_state(&mut self, new_state: String);
}