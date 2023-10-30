use crate::hash::{HashFunction, Hasher};

/// SHA256 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA256;

/// SHA256 partial hasher
pub struct SHA256Hasher;

/// The size of the output in bits
const BIT_SIZE: usize = 256;

/// The size of the output in bytes
const BYTE_SIZE: usize = BIT_SIZE / 8;

impl HashFunction for SHA256 {
    const NAME: &'static str = "SHA256";

    type Hasher = SHA256Hasher;

    fn begin_hash() -> Self::Hasher {
        SHA256Hasher
    }
}

impl Hasher for SHA256Hasher {
    const SIZE: usize = BYTE_SIZE;

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, _source: I) {
        todo!()
    }

    fn finalize_hash(self) -> [u8; Self::SIZE] {
        todo!()
    }
}
