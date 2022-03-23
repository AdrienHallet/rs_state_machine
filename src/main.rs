mod dsl;
mod machine;
use machine::Machine;

fn main() {
    let machine: Machine = define!(
        "CLOSED" -"OPEN"-> "OPENED",
        "OPENED" -"CLOSE"-> "CLOSED",
        "CLOSED" -"CLOSE"-> "CLOSED"
    );
    // trace_macros!(true);
    println!("machine: {:?}", machine);
}