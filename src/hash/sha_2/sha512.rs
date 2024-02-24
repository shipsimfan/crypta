use super::common::Hasher;
use crate::hash::{Hash, HashFunction, HashSize};

/// SHA512 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA512(Hasher<u64>);

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

    fn begin_hash() -> Self {
        SHA512(Hasher::new(INITIAL_HASH_VALUE))
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, source: I) {
        for byte in source {
            self.0.add_byte(byte);
        }
    }

    fn finalize_hash(mut self) -> Hash<Self> {
        self.0.calculate_last_block();

        let hash = self.0.unwrap_hash();

        Hash::new(unsafe { std::mem::transmute(hash) })
    }
}

impl HashSize for SHA512 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 512;
}
