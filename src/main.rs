mod dsl;
mod machine;
use machine::Machine;

fn main() {
    let machine: Machine = define!(
        "CLOSED" -"OPEN"-> "OPENED",
        "OPENED" -"CLOSE"-> "CLOSED"
    );

    println!("machine: {:?}", machine);
}