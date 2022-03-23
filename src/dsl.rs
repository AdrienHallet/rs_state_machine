#[macro_export]
macro_rules! define {
    ($in: literal -$transition: literal -> $out: literal) => {
        define!($crate::machine::Machine::new(), "test", "test", "test")
    };

    ($in: literal -$transition: literal -> $out: literal, $($in2: literal -$transition2: literal -> $out2: literal),*) => {{
        {
            let mut _var = $crate::machine::Machine::new();
            define!(
                 _var => "l"
            )
        }
    }};

    ($machine: ident => $lit: literal) => {{
        $machine.transitions.push("test");
        define!($machine, "one" -"two"-> "three")
    }};

    ($machine: expr, $in: literal -$transition: literal -> $out: literal) => {
        $machine
    };
}
