use super::common::SHA2HasherState;
use crate::hash::{md5_like::common::Hasher, Hash, HashFunction, HashSize};

/// SHA384 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA384(Hasher<SHA2HasherState<u64>, u128>);

const INITIAL_HASH_VALUE: [u64; 8] = [
    0xCBBB9D5DC1059ED8,
    0x629A292A367CD507,
    0x9159015A3070DD17,
    0x152FECD8F70E5939,
    0x67332667FFC00B31,
    0x8EB44A8768581511,
    0xDB0C2E0D64F98FA7,
    0x47B5481DBEFA4FA4,
];

impl HashFunction for SHA384 {
    const NAME: &'static str = "SHA384";

    fn new() -> Self {
        SHA384(Hasher::new(SHA2HasherState::new(INITIAL_HASH_VALUE)))
    }

    fn push(&mut self, bytes: impl AsRef<[u8]>) {
        self.0.push_slice(bytes)
    }

    fn push_iter(&mut self, bytes: impl Iterator<Item = u8>) {
        self.0.push_iter(bytes)
    }

    fn digest(self) -> Hash<Self> {
        Hash::new(self.0.digest().unwrap::<6>())
    }
}

impl HashSize for SHA384 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 384;
}
