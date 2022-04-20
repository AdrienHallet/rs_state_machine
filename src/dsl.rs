/// # Defines state machines using plain strings 
/// 
/// This macro helps you quickly define a State Machine with a visually meaningful syntax.
/// 
/// # Syntax:
/// ```markdown
/// transition  := input_state - transition_event -> output_state [,]
/// machine     := { Transition }-
/// ```
/// * Transitions are separated by commas
/// * Trailing (i.e.: last transition) commas do not matter
/// * Whitespaces do not matter
/// 
/// # Example:
/// ```rust
/// # #[macro_use] extern crate rs_state_machine; fn main() {
/// let light_switch = define!(
///     "OFF" - "TURN_ON"  -> "ON",
///     "ON"  - "TURN_OFF" -> "OFF"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! define {
    // One-transition defined machines
    ($input: literal -$transition: literal -> $output: literal $(,)?) => {
        {
            let mut _var: Machine<&str, &str> = $crate::core::machine::Machine::new();
            define!(_var => $input - $transition -> $output)
        }
    };

    // Multiple-transitions defined machines
    ($input: literal -$transition: literal -> $output: literal $(, $input2: literal -$transition2: literal -> $output2: literal)* $(,)?) => {{
        {
            let mut _var = $crate::core::machine::Machine::new();
            define!(
                 _var => $input -$transition-> $output $(, $input2 -$transition2 -> $output2)+
            )
        }
    }};

    // Parsing one transition with remaining transitions
    ($machine: ident => $(,)? $input: literal -$transition: literal -> $output: literal $(, $input2: literal -$transition2: literal -> $output2: literal)+) => {{
        {
            $machine.add_transition($crate::core::transition::Transition::new($input, $transition, $output));
            define!($machine => $(, $input2 -$transition2 -> $output2)?)
        }
    }};

    // Parsing one transition with no remaining transition
    ($machine: ident => $(,)? $input: literal -$transition: literal -> $output: literal) => {
        {
            $machine.add_transition($crate::core::transition::Transition::new($input, $transition, $output));
            $machine
        }
    };
}