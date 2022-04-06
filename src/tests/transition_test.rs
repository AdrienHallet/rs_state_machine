use crate::model::transition::Transition;

#[test]
pub fn should_compare_partially() {
    let transition = Transition { event: "EVENT".to_string(), state_in: "INPUT".to_string(), state_out: "OUTPUT".to_string() };
    
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