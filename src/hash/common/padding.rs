use super::{BitLength, Block};

/// Pads `block` for MD5, SHA-1, and SHA-2
///
/// This function returns a tuple with two values:
///  1. The slice containing the padded block
///  2. An optional byte. If it is [`Some`], the padding couldn't fit in this block and another
///     round of hashing is needed with a block produced by [`zero_pad`]. The byte contained in the
///     [`Option`] is passed to the `first_byte` argument of [`zero_pad`].
pub(super) fn pad<Length: BitLength, const SIZE: usize>(
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

/// Pads the block for MD5, SHA-1, and SHA-2
///
/// # Panic
/// This function will panic if the block does not contain enough space to hold the padding
pub(super) fn zero_pad<Length: BitLength, const SIZE: usize>(
    block: &mut Block<SIZE>,
    length: Length,
    first_byte: u8,
) -> &[u8]
where
    [u8; SIZE]: Sized,
    [u8; std::mem::size_of::<Length>()]: Sized,
{
    // Make sure the block is long enough
    debug_assert!(block.remaining() >= 1 + std::mem::size_of::<Length>());

    // Push the first padding byte
    block.push_byte(first_byte);

    // Push zeros until near the end
    while block.remaining() > std::mem::size_of::<Length>() {
        block.push_byte(0);
    }

    // Push the bit-length
    block.push_slice(&length.to_be_bytes());

    // Return the padded block
    block.consume()
}
