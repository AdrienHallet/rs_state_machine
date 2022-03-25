mod dsl;
mod model;
use model::machine::Machine;

fn main() {
    let machine: Machine = define!(
        "CLOSED" -"OPEN"-> "OPENED",
        "OPENED" -"CLOSE"-> "CLOSED",
        "CLOSED" -"CLOSE2"-> "CLOSED",
        "CLOSED" -"CLOSE3"-> "CLOSED"
    );
    println!("machine: {:?}", machine);
}