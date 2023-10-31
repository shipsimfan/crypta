use super::{SHAHasher, Word};
use crate::hash::{Hash, HashFunction, HashSize};

/// SHA384 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA384(SHAHasher<u64, 80>);

/// The size of the output in bits
const BIT_SIZE: usize = 384;

/// The size of the output in bytes
const BYTE_SIZE: usize = BIT_SIZE / 8;

const INITIAL_HASH_VALUE: [Word<u64>; 8] = [
    Word(0xCBBB9D5DC1059ED8),
    Word(0x629A292A367CD507),
    Word(0x9159015A3070DD17),
    Word(0x152FECD8F70E5939),
    Word(0x67332667FFC00B31),
    Word(0x8EB44A8768581511),
    Word(0xDB0C2E0D64F98FA7),
    Word(0x47B5481DBEFA4FA4),
];

impl HashFunction for SHA384 {
    const NAME: &'static str = "SHA384";

    fn begin_hash() -> Self {
        SHA384(SHAHasher::new(INITIAL_HASH_VALUE))
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
