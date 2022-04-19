use crate::core::transition::Transition;

#[test]
pub fn should_compare_partially() {
    let transition = Transition::new("INPUT".to_string(), "EVENT".to_string(), "OUTPUT".to_string());
    
    assert!(transition.partial_compare(None, None, None));
    assert!(transition.partial_compare(None, None, Some(&"OUTPUT".to_string())));
    assert!(transition.partial_compare(None, Some(&"EVENT".to_string()), None));
    assert!(transition.partial_compare(Some(&"INPUT".to_string()), None, None));
    assert!(transition.partial_compare(Some(&"INPUT".to_string()), Some(&"EVENT".to_string()), None));
    assert!(transition.partial_compare(None, Some(&"EVENT".to_string()), Some(&"OUTPUT".to_string())));
    assert!(transition.partial_compare(Some(&"INPUT".to_string()), None, Some(&"OUTPUT".to_string())));
    assert!(transition.partial_compare(Some(&"INPUT".to_string()), Some(&"EVENT".to_string()), Some(&"OUTPUT".to_string())));

    assert!(!transition.partial_compare(None, None, Some(&"OTHER".to_string())));
    assert!(!transition.partial_compare(None, Some(&"OTHER".to_string()), None));
    assert!(!transition.partial_compare(Some(&"OTHER".to_string()), None, None));
}

#[test]
pub fn should_allow_without_guard() {
    let transition = Transition::new("INPUT".to_string(), "EVENT".to_string(), "OUTPUT".to_string());
    assert!(transition.is_allowed());
}

#[test]
pub fn should_guard() {
    let mut transition = Transition::new("INPUT".to_string(), "EVENT".to_string(), "OUTPUT".to_string());
    transition.guard = Some(|| true);
    assert!(transition.is_allowed());

    transition.guard = Some(|| false);
    assert!(!transition.is_allowed());
}