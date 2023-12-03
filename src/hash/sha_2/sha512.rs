use crate::hash::{Hash, HashFunction, HashSize};

/// SHA512 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA512;

impl HashFunction for SHA512 {
    const NAME: &'static str = "SHA512";

    fn begin_hash() -> Self {
        SHA512
    }

    fn add_hash<I: IntoIterator<Item = u8>>(&mut self, _source: I) {}

    fn finalize_hash(self) -> Hash<Self> {
        Hash::default()
    }
}

impl HashSize for SHA512 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 512;
}
