# Rust State Machine
![Build](https://github.com/AdrienHallet/rs_state_machine/actions/workflows/rust.yml/badge.svg)
> A (Finite) State Machine library

This Rust Library aims to provide a developer-friendly, Business-Oriented, (Finite) State Machine.

## Features
* Use any type for your States and Events
* Define State Machines, even easier with the library's DSL
* Apply events on states to know the output
* Guard functions on transitions

_More features coming Soon™_

## Examples

Quickly define your State Machine using the integrated DSL `define!()` macro:
```rust
fn main() {
    let light_switch = define!(
        "OFF" - "TURN_ON"  -> "ON",
        "ON"  - "TURN_OFF" -> "OFF"
    );
}
``` 

Use any type to define your `State`s and `Event`s, why not enums:
```rust
fn main() {
    let mut enum_light_switch = Machine::new();
    enum_light_switch.add_transition(Transition::new(LightState::Off, LightEvent::TurnOn, LightState::On));
    enum_light_switch.add_transition(Transition::new(LightState::On, LightEvent::TurnOff, LightState::Off));
    let mut state_light = StatefulLight { state: LightState::Off };

    enum_light_switch.apply(&mut state_light, LightEvent::TurnOn);
}
```
