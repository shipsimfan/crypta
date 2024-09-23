use crate::hash::common::{zero_pad, BitLength, Block};

/// Pads `block` for MD5, SHA-1, and SHA-2
///
/// This function returns a tuple with two values:
///  1. The slice containing the padded block
///  2. An optional byte. If it is [`Some`], the padding couldn't fit in this block and another
///     round of hashing is needed with a block produced by [`zero_pad`]. The byte contained in the
///     [`Option`] is passed to the `first_byte` argument of [`zero_pad`].
pub(in crate::hash::common) fn pad<Length: BitLength, const SIZE: usize>(
    block: &mut Block<SIZE>,
    length: Length,
) -> (&[u8], Option<u8>)
where
    [u8; SIZE]: Sized,
    [u8; std::mem::size_of::<Length>()]: Sized,
{
    if block.remaining() >= 1 + std::mem::size_of::<Length>() {
        (zero_pad(block, length, 0x80), None)
    } else if block.remaining() >= 1 {
        block.push_byte(0x80);
        while block.remaining() > 0 {
            block.push_byte(0);
        }
        (block.consume(), Some(0))
    } else {
        (block.consume(), Some(0x80))
    }
}
