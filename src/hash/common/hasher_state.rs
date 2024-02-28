// rustdoc imports
#[allow(unused_imports)]
use super::Hasher;

/// A state for a [`Hasher`]
pub(in crate::hash) trait HasherState {
    /// The size of the block to be given to `Self::compress()`
    const BLOCK_SIZE: usize;

    /// Compress `block` into `self`
    ///
    /// `block` will always be exactly `Self::BLOCK_SIZE` bytes long
    fn compress(&mut self, block: &[u8]);
}
