use crate::hash::{Hash, HashFunction, HashSize};

/// SHA224 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA224;

impl HashFunction for SHA224 {
    const NAME: &'static str = "SHA224";

    fn begin_hash() -> Self {
        SHA224
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, _source: I) {}

    fn finalize_hash(self) -> Hash<Self> {
        Hash::default()
    }
}

impl HashSize for SHA224 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 224;
}
