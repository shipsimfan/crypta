use super::SHAHasher;
use crate::hash::{Hash, HashFunction, HashSize};

/// SHA384 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA384(SHAHasher<u64>);

/// The size of the output in bits
const BIT_SIZE: usize = 384;

/// The size of the output in bytes
const BYTE_SIZE: usize = BIT_SIZE / 8;

impl HashFunction for SHA384 {
    const NAME: &'static str = "SHA384";

    fn begin_hash() -> Self {
        SHA384(SHAHasher::new())
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, source: I) {
        self.0.add_hash(&mut source.into_iter())
    }

    fn finalize_hash(self) -> Hash<Self> {
        let mut result = [0; Self::SIZE];
        result.copy_from_slice(&self.0.finalize_hash()[..Self::SIZE]);
        Hash::new(result)
    }
}

impl HashSize for SHA384 {
    const SIZE: usize = BYTE_SIZE;
}
