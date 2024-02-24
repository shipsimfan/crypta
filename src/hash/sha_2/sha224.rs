use super::common::Hasher;
use crate::hash::{Hash, HashFunction, HashSize};

/// SHA224 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA224(Hasher<u32>);

const INITIAL_HASH_VALUE: [u32; 8] = [
    0xC1059ED8, 0x367CD507, 0x3070DD17, 0xF70E5939, 0xFFC00B31, 0x68581511, 0x64F98FA7, 0xBEFA4FA4,
];

impl HashFunction for SHA224 {
    const NAME: &'static str = "SHA224";

    fn begin_hash() -> Self {
        SHA224(Hasher::new(INITIAL_HASH_VALUE))
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, source: I) {
        for byte in source {
            self.0.add_byte(byte);
        }
    }

    fn finalize_hash(mut self) -> Hash<Self> {
        self.0.calculate_last_block();

        let full_hash = self.0.unwrap_hash();
        let mut hash = [0; 7];
        hash.copy_from_slice(&full_hash[..7]);

        Hash::new(unsafe { std::mem::transmute(hash) })
    }
}

impl HashSize for SHA224 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 224;
}
