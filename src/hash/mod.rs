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

mod hash;

// Hash Algorithm re-exports
pub use sha::*;

pub use hash::*;

/// A trait for holding the size of a resulting hash.
///
/// This is required to avoid cycles in the [`HashFunction`] trait.
pub trait HashSize {
    /// The size of the output in bytes
    const SIZE: usize;
}

/// A standard interface for hash functions.
pub trait HashFunction: HashSize + Sized
where
    [u8; Self::SIZE]: Sized,
{
    /// Name for this hash function
    const NAME: &'static str;

    /// Start a hash by returning a [`Hasher`]
    fn begin_hash() -> Self;

    /// Hashes `source`
    fn hash<I: IntoIterator<Item = u8>>(source: I) -> Hash<Self> {
        let mut hasher = Self::begin_hash();
        hasher.add_hash(source);
        hasher.finalize_hash()
    }

    /// Add the bytes from `source` to hash
    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, source: I);

    /// Finalize and return the resulting hash
    fn finalize_hash(self) -> Hash<Self>;
}
