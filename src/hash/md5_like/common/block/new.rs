use crate::hash::md5_like::common::Block;

impl<const SIZE: usize> Block<SIZE>
where
    [u8; SIZE]: Sized,
{
    /// Creates a new [`Block`] filled with 0s and with 0 length
    pub(in crate::hash::md5_like::common) fn new() -> Self {
        Block {
            block: [0; SIZE],
            index: 0,
        }
    }
}
