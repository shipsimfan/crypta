use super::common::Hasher;
use crate::hash::{Hash, HashFunction, HashSize};

/// SHA256 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA256(Hasher<u32>);

const INITIAL_HASH_VALUE: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

impl HashFunction for SHA256 {
    const NAME: &'static str = "SHA256";

    fn begin_hash() -> Self {
        SHA256(Hasher::new(INITIAL_HASH_VALUE))
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

impl HashSize for SHA256 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 256;
}
