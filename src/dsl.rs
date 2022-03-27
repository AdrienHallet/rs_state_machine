#[macro_export]
macro_rules! define {
    ($input: literal -$transition: literal -> $output: literal $(,)?) => {
        {
            let mut _var = $crate::model::machine::Machine::new();
            define!(_var => $input - $transition -> $output)
        }
    };

    ($input: literal -$transition: literal -> $output: literal $(, $input2: literal -$transition2: literal -> $output2: literal)* $(,)?) => {{
        {
            let mut _var = $crate::model::machine::Machine::new();
            define!(
                 _var => $input -$transition-> $output $(, $input2 -$transition2 -> $output2)+
            )
        }
    }};

    ($machine: ident => $(,)? $input: literal -$transition: literal -> $output: literal $(, $input2: literal -$transition2: literal -> $output2: literal)+) => {{
        {
            $machine.add_transition($crate::model::transition::Transition::new($input, $transition, $output));
            define!($machine => $(, $input2 -$transition2 -> $output2)?)
        }
    }};

    ($machine: ident => $(,)? $input: literal -$transition: literal -> $output: literal) => {
        {
            $machine.add_transition($crate::model::transition::Transition::new($input, $transition, $output));
            $machine
        }
    };
}