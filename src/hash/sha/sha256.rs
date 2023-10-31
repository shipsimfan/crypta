use super::SHAHasher;
use crate::hash::{Hash, HashFunction, HashSize};

/// SHA256 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA256(SHAHasher<u32>);

/// The size of the output in bits
const BIT_SIZE: usize = 256;

/// The size of the output in bytes
const BYTE_SIZE: usize = BIT_SIZE / 8;

impl HashFunction for SHA256 {
    const NAME: &'static str = "SHA256";

    fn begin_hash() -> Self {
        SHA256(SHAHasher::new())
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, source: I) {
        self.0.add_hash(&mut source.into_iter())
    }

    fn finalize_hash(self) -> Hash<Self> {
        Hash::new(self.0.finalize_hash())
    }
}

impl HashSize for SHA256 {
    const SIZE: usize = BYTE_SIZE;
}
