use super::common::Hasher;
use crate::hash::{Hash, HashFunction, HashSize};

/// SHA384 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA384(Hasher<u64>);

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

    fn begin_hash() -> Self {
        SHA384(Hasher::new(INITIAL_HASH_VALUE))
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, source: I) {
        for byte in source {
            self.0.add_byte(byte);
        }
    }

    fn finalize_hash(mut self) -> Hash<Self> {
        self.0.calculate_last_block();

        let full_hash = self.0.unwrap_hash();
        let mut hash = [0; 6];
        hash.copy_from_slice(&full_hash[..6]);

        Hash::new(unsafe { std::mem::transmute(hash) })
    }
}

impl HashSize for SHA384 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 384;
}
