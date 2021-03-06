/// Implement the [Transitionable] trait to use the [Machine](super::machine::Machine) directly with your structures.
pub trait Transitionable<S> {

    /// Returns the current state of a structure.
    fn get_state(&self) -> S;
    
    /// Sets a new state in the structure.
    fn set_state(&mut self, new_state: S);
}