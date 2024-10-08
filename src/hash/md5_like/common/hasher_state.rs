// rustdoc imports
#[allow(unused_imports)]
use crate::hash::md5_like::common::Hasher;

/// A state for a [`Hasher`]
pub(in crate::hash::md5_like) trait HasherState {
    /// The size of the block to be given to `Self::compress()`
    const BLOCK_SIZE: usize;

    /// Should the final bit-length that gets appended be in big-endian bit order?
    const BIG_ENDIAN_LENGTH: bool = true;

    /// Compress `block` into `self`
    ///
    /// `block` will always be exactly `Self::BLOCK_SIZE` bytes long
    fn compress(&mut self, block: &[u8]);
}
