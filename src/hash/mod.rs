//! # Hash Module
//! Contains a standard hash interface and some hash functions.
//!
//! The following hash functions have been implemented:
//!  - [`SHA224`]
//!  - [`SHA256`]
//!  - [`SHA384`]
//!  - [`SHA512`]

// Hash Algorithm modules
mod sha;

// Hash Algorithm re-exports
pub use sha::*;

/// A standard interface for hash functions.
pub trait HashFunction {
    /// Name for this hash function
    const NAME: &'static str;

    /// The underlying partial hasher.
    type Hasher: Hasher;

    /// Start a hash by returning a [`Hasher`]
    fn begin_hash() -> Self::Hasher;

    /// Hashes `source`
    fn hash<I: IntoIterator<Item = u8>>(source: I) -> [u8; Self::Hasher::SIZE] {
        let mut hasher = Self::begin_hash();
        hasher.add_hash(source);
        hasher.finalize_hash()
    }

    /// Returns the size of blocks produced
    fn size() -> usize {
        Self::Hasher::SIZE
    }
}

/// An interface for partial hashing.
pub trait Hasher {
    /// The size of the output in bytes
    const SIZE: usize;

    /// Add the bytes from `source` to hash
    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, source: I);

    /// Finalize and return the resulting hash
    fn finalize_hash(self) -> [u8; Self::SIZE];
}
