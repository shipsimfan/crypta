use crate::hash::common::{BitLength, Block};

/// Pads the block for MD5, SHA-1, and SHA-2
///
/// # Panic
/// This function will panic if the block does not contain enough space to hold the padding
pub(in crate::hash::common) fn zero_pad<Length: BitLength, const SIZE: usize>(
    block: &mut Block<SIZE>,
    length: Length,
    first_byte: u8,
    big_endian_length: bool,
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
    block.push_slice(&if big_endian_length {
        length.to_be_bytes()
    } else {
        length.to_le_bytes()
    });

    // Return the padded block
    block.consume()
}
