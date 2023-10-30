use crate::hash::{HashFunction, Hasher};

/// SHA224 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA224;

/// SHA224 partial hasher
pub struct SHA224Hasher;

/// The size of the output in bits
const BIT_SIZE: usize = 224;

/// The size of the output in bytes
const BYTE_SIZE: usize = BIT_SIZE / 8;

impl HashFunction for SHA224 {
    const NAME: &'static str = "SHA224";

    type Hasher = SHA224Hasher;

    fn begin_hash() -> Self::Hasher {
        SHA224Hasher
    }
}

impl Hasher for SHA224Hasher {
    const SIZE: usize = BYTE_SIZE;

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, _source: I) {
        todo!()
    }

    fn finalize_hash(self) -> [u8; Self::SIZE] {
        todo!()
    }
}
