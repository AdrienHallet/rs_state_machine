use crate::{define, model::machine::Machine};

#[test]
fn should_parse_one_line() {
    let machine: Machine = define!("A" -"B"->"C");
    assert_eq!(1, machine.transitions.len(), "Machine should have exactly 1 transition");
}

#[test]
fn should_parse_multiple_lines() {
    let machine: Machine = define!(
        "A" -"B"->"C",
        "D" -"E"->"F",
        "G" -"H"->"I"
    );
    assert_eq!(3, machine.transitions.len(), "Machine should have exactly 3 transitions")
}