use crate::hash::{common::Hasher, Hash, HashFunction, HashSize};
use hasher_state::MD5HasherState;

mod hasher_state;

/// MD5 hash function
///
/// Implemented as defined in [RFC 1321](https://doi.org/10.17487/RFC1321)
pub struct MD5(Hasher<MD5HasherState, u64>);

impl HashFunction for MD5 {
    const NAME: &'static str = "MD5";

    fn new() -> Self {
        MD5(Hasher::new(MD5HasherState::new()))
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

impl HashSize for MD5 {
    /// The size of the output in bits
    const BIT_SIZE: usize = 128;
}
