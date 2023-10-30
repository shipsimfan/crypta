use crate::hash::{HashFunction, Hasher};

/// SHA512 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA512;

/// SHA512 partial hasher
pub struct SHA512Hasher;

/// The size of the output in bits
const BIT_SIZE: usize = 512;

/// The size of the output in bytes
const BYTE_SIZE: usize = BIT_SIZE / 8;

impl HashFunction for SHA512 {
    const NAME: &'static str = "SHA512";

    type Hasher = SHA512Hasher;

    fn begin_hash() -> Self::Hasher {
        SHA512Hasher
    }
}

impl Hasher for SHA512Hasher {
    const SIZE: usize = BYTE_SIZE;

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, _source: I) {
        todo!()
    }

    fn finalize_hash(self) -> [u8; Self::SIZE] {
        todo!()
    }
}
