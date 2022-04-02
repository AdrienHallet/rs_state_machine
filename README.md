# Rust State Machine
![Build](https://github.com/AdrienHallet/rs_state_machine/actions/workflows/rust.yml/badge.svg)
> A (Finite) State Machine library in the works

This Rust Library aims to provide a developer-friendly, Business-Oriented, (Finite) State Machine.

## Features
* Define State Machines, even easier with the library's DSL
* Apply events on states to know the output

_More features (hopefully) coming Soon™_

## Example

Quickly define your State Machine using the integrated DSL `define!()` macro:
```rust
fn main() {
    let light_switch = define!(
        "OFF" - "TURN_ON"  -> "ON",
        "ON"  - "TURN_OFF" -> "OFF"
    );
}
``` 
