use super::SHAHasher;
use crate::hash::{Hash, HashFunction, HashSize};

/// SHA512 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA512(SHAHasher<u64>);

/// The size of the output in bits
const BIT_SIZE: usize = 512;

/// The size of the output in bytes
const BYTE_SIZE: usize = BIT_SIZE / 8;

impl HashFunction for SHA512 {
    const NAME: &'static str = "SHA512";

    fn begin_hash() -> Self {
        SHA512(SHAHasher::new())
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, source: I) {
        self.0.add_hash(&mut source.into_iter())
    }

    fn finalize_hash(self) -> Hash<Self> {
        Hash::new(self.0.finalize_hash())
    }
}

impl HashSize for SHA512 {
    const SIZE: usize = BYTE_SIZE;
}
