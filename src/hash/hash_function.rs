use super::Hash;

/// A trait for holding the size of a resulting hash.
///
/// This is required to avoid cycles in the [`HashFunction`] trait.
pub trait HashSize {
    /// The size of the output in bits
    const BIT_SIZE: usize;

    /// The size of the output in bytes
    const BYTE_SIZE: usize = (Self::BIT_SIZE + 7) / 8;
}

/// A standard interface for hash functions.
pub trait HashFunction: HashSize + Sized
where
    [u8; Self::BYTE_SIZE]: Sized,
{
    /// Name for this hash function
    const NAME: &'static str;

    /// Hashes `bytes`
    fn hash(bytes: impl AsRef<[u8]>) -> Hash<Self> {
        let mut hasher = Self::new();
        hasher.push(bytes);
        hasher.digest()
    }

    /// Hashes `bytes`
    fn hash_iter(bytes: impl Iterator<Item = u8>) -> Hash<Self> {
        let mut hasher = Self::new();
        hasher.push_iter(bytes);
        hasher.digest()
    }

    /// Start a hash by returning a [`HashFunction`]
    fn new() -> Self;

    /// Adds `bytes` to the hash
    fn push(&mut self, bytes: impl AsRef<[u8]>);

    /// Adds `bytes` to the hash
    fn push_iter(&mut self, bytes: impl Iterator<Item = u8>);

    /// Finalize and return the resulting [`Hash`]
    fn digest(self) -> Hash<Self>;
}
