macro_rules! hash {
    ($algorithm: ident, $input: ident) => {
        $crate::hash::$algorithm::hash($input)
    };
    ($algorithm: ident, $input: ident, $count: literal) => {{
        let mut hasher = $crate::hash::$algorithm::new();
        for _ in 0..$count {
            hasher.push($input);
        }
        hasher.digest()
    }};
}

macro_rules! test {
    {$output_extension: ident, $algorithm: ident, $test_name: ident} => {
        test!{$output_extension, $algorithm, $test_name, ".in", hash!($algorithm, INPUT)}
    };
    {$output_extension: ident, $algorithm: ident, ($test_name: ident, $count: literal)} => {
        test!{$output_extension, $algorithm, $test_name, ".inr", hash!($algorithm, INPUT, $count)}
    };
    {$output_extension: ident, $algorithm: ident, $test_name: ident, $input_extension: literal, $hash: expr} => {
        #[test]
        fn $test_name() {
            use $crate::hash::HashFunction;

            const INPUT: &[u8] = include_bytes!(concat!("inputs/", stringify!($test_name), $input_extension));
            const OUTPUT: &str = include_str!(concat!("outputs/", stringify!($test_name), ".", stringify!($output_extension)));

            let hash = $hash;

            assert_eq!(&hash.to_string(), OUTPUT);
        }
    };
}

macro_rules! algorithm_tests {
    {
        "Algorithm": ($test_mod: ident, $algorithm: ident),
        "Tests": [$($test: tt),*],
    } => {
        mod $test_mod {
            $(test! {
                $test_mod,
                $algorithm,
                $test
            })*
        }
    };
}

macro_rules! tests {
    {
        "Algorithms": [$(($test_mod: ident, $algorithm: ident)),*],
        "Tests": $tests: tt,
     } => {
        $(algorithm_tests! {
            "Algorithm": ($test_mod, $algorithm),
            "Tests": $tests,
        })*
     };
}
