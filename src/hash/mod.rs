//! Contains a standard hash interface and some hash functions.
//!
//! The following hash functions have been implemented:
//!  - [`SHA1`]
//!  - [`SHA224`]
//!  - [`SHA256`]
//!  - [`SHA384`]
//!  - [`SHA512`]

mod hash;
mod hash_function;

// Hash Algorithm modules
mod md5_like;

pub use hash::{Hash, ParseHashError};
pub use hash_function::{HashFunction, HashSize};

// Hash Algorithm re-exports
pub use md5_like::{MD5, SHA1, SHA224, SHA256, SHA384, SHA512};

#[cfg(test)]
mod tests;
