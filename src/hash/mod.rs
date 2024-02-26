//! Contains a standard hash interface and some hash functions.
//!
//! The following hash functions have been implemented:
//!  - [`SHA224`]
//!  - [`SHA256`]
//!  - [`SHA384`]
//!  - [`SHA512`]

// Hash Algorithm modules
mod sha_2;

mod hash;
mod hash_function;

mod common;

#[cfg(test)]
mod tests;

// Hash Algorithm re-exports
pub use sha_2::{SHA224, SHA256, SHA384, SHA512};

pub use hash::{Hash, ParseHashError};
pub use hash_function::{HashFunction, HashSize};
