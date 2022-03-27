pub trait Transitionable<'obj> {
    fn get_state(&'obj self) -> &str;
    
    fn set_state(&'obj mut self, new_state: &'obj str);
}