#[macro_export]
macro_rules! define {
    ($in: literal -$transition: literal -> $out: literal $(,)?) => {
        {
            let mut _var = $crate::machine::Machine::new();
            define!(_var => $in - $transition -> $out)
        }
    };

    ($in: literal -$transition: literal -> $out: literal $(, $in2: literal -$transition2: literal -> $out2: literal)* $(,)?) => {{
        {
            let mut _var = $crate::model::machine::Machine::new();
            define!(
                 _var => $in -$transition-> $out $(, $in2 -$transition2 -> $out2)+
            )
        }
    }};

    ($machine: ident => $(,)? $in: literal -$transition: literal -> $out: literal $(, $in2: literal -$transition2: literal -> $out2: literal)+) => {{
        {
            $machine.transitions.push($transition);
            define!($machine => $(, $in2 -$transition2 -> $out2)?)
        }
    }};

    ($machine: ident => $(,)? $in: literal -$transition: literal -> $out: literal) => {
        {
            $machine.transitions.push($transition);
            $machine
        }
    };
}