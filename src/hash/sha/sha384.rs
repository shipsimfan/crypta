use crate::hash::{Hash, HashFunction, HashSize};

/// SHA384 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA384;

/// The size of the output in bits
const BIT_SIZE: usize = 384;

/// The size of the output in bytes
const BYTE_SIZE: usize = BIT_SIZE / 8;

impl HashFunction for SHA384 {
    const NAME: &'static str = "SHA384";

    fn begin_hash() -> Self {
        SHA384
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, _source: I) {
        todo!()
    }

    fn finalize_hash(self) -> Hash<Self> {
        todo!()
    }
}

impl HashSize for SHA384 {
    const SIZE: usize = BYTE_SIZE;
}
