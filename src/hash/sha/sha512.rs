use super::{SHAHasher, Word};
use crate::hash::{Hash, HashFunction, HashSize};

/// SHA512 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA512(SHAHasher<u64, 80>);

/// The size of the output in bits
const BIT_SIZE: usize = 512;

/// The size of the output in bytes
const BYTE_SIZE: usize = BIT_SIZE / 8;

const INITIAL_HASH_VALUE: [Word<u64>; 8] = [
    Word(0x6A09E667F3BCC908),
    Word(0xBB67AE8584CAA73B),
    Word(0x3C6EF372FE94F82B),
    Word(0xA54FF53A5F1D36F1),
    Word(0x510E527FADE682D1),
    Word(0x9B05688C2B3E6C1F),
    Word(0x1F83D9ABFB41BD6B),
    Word(0x5BE0CD19137E2179),
];

impl HashFunction for SHA512 {
    const NAME: &'static str = "SHA512";

    fn begin_hash() -> Self {
        SHA512(SHAHasher::new(INITIAL_HASH_VALUE))
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, source: I) {
        self.0.add_hash(&mut source.into_iter())
    }

    fn finalize_hash(self) -> Hash<Self> {
        Hash::new(self.0.finalize_hash())
    }
}

impl HashSize for SHA512 {
    const SIZE: usize = BYTE_SIZE;
}
