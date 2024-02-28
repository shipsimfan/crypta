use crate::hash::{common::Hasher, Hash, HashFunction, HashSize};
use hasher_state::SHA1HasherState;

mod hasher_state;

/// SHA1 hash function
///
/// Implemented as defined in [RFC 6234](https://doi.org/10.17487/RFC6234)
pub struct SHA1(Hasher<SHA1HasherState, u64>);

impl HashFunction for SHA1 {
    const NAME: &'static str = "SHA1";

    fn new() -> Self {
        SHA1(Hasher::new(SHA1HasherState::new()))
    }

    fn push(&mut self, bytes: impl AsRef<[u8]>) {
        self.0.push_slice(bytes)
    }

    fn push_iter(&mut self, bytes: impl Iterator<Item = u8>) {
        self.0.push_iter(bytes)
    }

    fn digest(self) -> Hash<Self> {
        Hash::new(self.0.digest().unwrap())
    }
}

impl HashSize for SHA1 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 160;
}
