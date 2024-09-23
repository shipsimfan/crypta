mod new;
mod push;

/// A block of data to be hashed
pub(super) struct Block<const SIZE: usize>
where
    [u8; SIZE]: Sized,
{
    /// The block of data itself
    block: [u8; SIZE],

    /// The index of the next byte of data
    index: usize,
}

impl<const SIZE: usize> Block<SIZE>
where
    [u8; SIZE]: Sized,
{
    /// Gets the number of free bytes remaining in the block
    pub(super) fn remaining(&self) -> usize {
        SIZE - self.index
    }

    /// Have any bytes been pushed to this block yet?
    pub(super) fn is_empty(&self) -> bool {
        self.index == 0
    }
}
