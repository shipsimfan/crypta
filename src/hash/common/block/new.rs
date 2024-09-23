use crate::hash::common::Block;

impl<const SIZE: usize> Block<SIZE>
where
    [u8; SIZE]: Sized,
{
    /// Creates a new [`Block`] filled with 0s and with 0 length
    pub(in crate::hash::common) fn new() -> Self {
        Block {
            block: [0; SIZE],
            index: 0,
        }
    }
}
