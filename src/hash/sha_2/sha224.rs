use super::common::SHA2HasherState;
use crate::hash::{common::Hasher, Hash, HashFunction, HashSize};

/// SHA224 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA224(Hasher<SHA2HasherState<u32>, u64>);

const INITIAL_HASH_VALUE: [u32; 8] = [
    0xC1059ED8, 0x367CD507, 0x3070DD17, 0xF70E5939, 0xFFC00B31, 0x68581511, 0x64F98FA7, 0xBEFA4FA4,
];

impl HashFunction for SHA224 {
    const NAME: &'static str = "SHA224";

    fn new() -> Self {
        SHA224(Hasher::new(SHA2HasherState::new(INITIAL_HASH_VALUE)))
    }

    fn push(&mut self, bytes: impl AsRef<[u8]>) {
        self.0.push_slice(bytes)
    }

    fn push_iter(&mut self, bytes: impl Iterator<Item = u8>) {
        self.0.push_iter(bytes)
    }

    fn digest(self) -> Hash<Self> {
        Hash::new(self.0.digest().unwrap::<7>())
    }
}

impl HashSize for SHA224 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 224;
}
