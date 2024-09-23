use crate::hash::{Hash, HashFunction};

impl<H: HashFunction> Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    /// A hash filled with 0
    pub const ZERO: Self = Self::new([0; H::BYTE_SIZE]);

    /// Creates a new hash
    pub const fn new(hash: [u8; H::BYTE_SIZE]) -> Self {
        Hash(hash)
    }

    /// Hashes `bytes` and returns the resulting hash
    pub fn hash(bytes: impl AsRef<[u8]>) -> Self {
        H::hash(bytes)
    }

    /// Hashes `bytes` and returns the resulting hash
    pub fn hash_iter(bytes: impl Iterator<Item = u8>) -> Self {
        H::hash_iter(bytes)
    }
}
