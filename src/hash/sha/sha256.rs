use crate::hash::{Hash, HashFunction, HashSize};

/// SHA256 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA256;

/// The size of the output in bits
const BIT_SIZE: usize = 256;

/// The size of the output in bytes
const BYTE_SIZE: usize = BIT_SIZE / 8;

impl HashFunction for SHA256 {
    const NAME: &'static str = "SHA256";

    fn begin_hash() -> Self {
        SHA256
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, _source: I) {
        todo!()
    }

    fn finalize_hash(self) -> Hash<Self> {
        todo!()
    }
}

impl HashSize for SHA256 {
    const SIZE: usize = BYTE_SIZE;
}
