//! Secure Hashing Algorithms Tests
//!
//! Sourced from [DI Management](https://www.di-mgt.com.au/sha_testvectors.html)

/// Produces a test function for the given input and algorithm
macro_rules! test {
    ($(#[$attr:meta])* $fn_name: ident, $algorithm: ident, $algo_length: literal, $test_name: literal) => {
        $(#[$attr])*
        #[test]
        fn $fn_name() {
            use $crate::hash::HashFunction;

            const INPUT: &[u8] = include_bytes!(concat!("tests/", $test_name, ".in"));
            const OUTPUT: &[u8] = include_bytes!(concat!("tests/", $test_name, ".", stringify!($algo_length)));

            let hash = $crate::hash::$algorithm::hash(INPUT.into_iter().map(|value| *value));

            assert_eq!(hash, OUTPUT);
        }
    };
}

macro_rules! test_repeated {
    ($(#[$attr:meta])* $fn_name: ident, $algorithm: ident, $algo_length: literal, $test_name: literal, $count: literal) => {
        $(#[$attr])*
        #[test]
        fn $fn_name() {
            use $crate::hash::{HashFunction, Hasher};

            const INPUT: &[u8] = include_bytes!(concat!("tests/", $test_name, ".inr"));
            const OUTPUT: &[u8] = include_bytes!(concat!("tests/", $test_name, ".", stringify!($algo_length)));

            let mut hasher = $crate::hash::$algorithm::begin_hash();
            let iter = INPUT.into_iter().map(|value| *value);
            for _ in 0..$count {
                hasher.add_hash(iter.clone());
            }
            let hash = hasher.finalize_hash();

            assert_eq!(hash, OUTPUT);
        }
    };
}

test!(sha256_abc, SHA256, 256, "abc");
test!(sha384_abc, SHA384, 384, "abc");
test!(sha512_abc, SHA512, 512, "abc");

test!(sha256_empty, SHA256, 256, "empty");
test!(sha384_empty, SHA384, 384, "empty");
test!(sha512_empty, SHA512, 512, "empty");

test!(sha256_len_448, SHA256, 256, "len_448");
test!(sha384_len_448, SHA384, 384, "len_448");
test!(sha512_len_448, SHA512, 512, "len_448");

test!(sha256_len_896, SHA256, 256, "len_896");
test!(sha384_len_896, SHA384, 384, "len_896");
test!(sha512_len_896, SHA512, 512, "len_896");

test_repeated!(sha256_a_repeated, SHA256, 256, "a_rep", 1_000_000);
test_repeated!(sha384_a_repeated, SHA384, 384, "a_rep", 1_000_000);
test_repeated!(sha512_a_repeated, SHA512, 512, "a_rep", 1_000_000);

test_repeated!(sha256_long_repeated, SHA256, 256, "long_rep", 16_777_216);
test_repeated!(sha384_long_repeated, SHA384, 384, "long_rep", 16_777_216);
test_repeated!(sha512_long_repeated, SHA512, 512, "long_rep", 16_777_216);
