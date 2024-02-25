//! Secure Hashing Algorithms Tests
//!
//! Sourced from [DI Management](https://www.di-mgt.com.au/sha_testvectors.html)

#[macro_use]
mod macros;

tests! {
    "Algorithms": [
        (sha224, SHA224),
        (sha256, SHA256),
        (sha384, SHA384),
        (sha512, SHA512)
    ],
    "Tests": [
        abc,
        empty,
        fox,
        len_448,
        len_896,
        (a_rep, 1_000_000),
        (long_rep, 16_777_216)
    ],
}
