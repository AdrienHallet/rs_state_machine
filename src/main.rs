mod dsl;
mod model;
use model::machine::Machine;

/// This function will disappear once released as library
fn main() {
    let light_switch = define!(
        "OFF" - "TURN_ON"  -> "ON",
        "ON"  - "TURN_OFF" -> "OFF"
    );
    let machine: Machine = define!(
        "CLOSED" -"OPEN"-> "OPENED",
        "OPENED" -"CLOSE"-> "CLOSED",
        "CLOSED" -"CLOSE2"-> "CLOSED",
        "CLOSED" -"CLOSE3"-> "CLOSED"
    );
    println!("machine: {:?}", machine);

    let output = machine.get_output("OPENED".to_string(), "CLOSE".to_string());
    println!("output: {:?}", output)
}

/// Tests
#[cfg(test)]
mod tests;