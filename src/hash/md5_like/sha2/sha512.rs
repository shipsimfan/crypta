use super::common::SHA2HasherState;
use crate::hash::{md5_like::common::Hasher, Hash, HashFunction, HashSize};

/// SHA512 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA512(Hasher<SHA2HasherState<u64>, u128>);

const INITIAL_HASH_VALUE: [u64; 8] = [
    0x6A09E667F3BCC908,
    0xBB67AE8584CAA73B,
    0x3C6EF372FE94F82B,
    0xA54FF53A5F1D36F1,
    0x510E527FADE682D1,
    0x9B05688C2B3E6C1F,
    0x1F83D9ABFB41BD6B,
    0x5BE0CD19137E2179,
];

impl HashFunction for SHA512 {
    const NAME: &'static str = "SHA512";

    fn new() -> Self {
        SHA512(Hasher::new(SHA2HasherState::new(INITIAL_HASH_VALUE)))
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

impl HashSize for SHA512 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 512;
}
