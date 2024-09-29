use super::common::SHA2HasherState;
use crate::hash::{md5_like::common::Hasher, Hash, HashFunction, HashSize};

/// SHA256 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA256(Hasher<SHA2HasherState<u32>, u64>);

const INITIAL_HASH_VALUE: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

impl HashFunction for SHA256 {
    const NAME: &'static str = "SHA256";

    fn new() -> Self {
        SHA256(Hasher::new(SHA2HasherState::new(INITIAL_HASH_VALUE)))
    }

    fn push(&mut self, bytes: impl AsRef<[u8]>) {
        self.0.push_slice(bytes)
    }

    fn push_iter(&mut self, bytes: impl Iterator<Item = u8>) {
        self.0.push_iter(bytes)
    }

    fn digest(self) -> Hash<Self> {
        Hash::new(self.0.digest().unwrap::<8>())
    }
}

impl HashSize for SHA256 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 256;
}
