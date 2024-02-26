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
    /// Creates a new [`Block`] filled with 0s and with 0 length
    pub(super) fn new() -> Self {
        Block {
            block: [0; SIZE],
            index: 0,
        }
    }

    /// Gets the number of free bytes remaining in the block
    pub(super) fn remaining(&self) -> usize {
        SIZE - self.index
    }

    /// Pushes `byte` to the end of the block
    ///
    /// # Panic
    /// This function will panic if the block already contains `SIZE` elements
    pub(super) fn push_byte(&mut self, byte: u8) {
        debug_assert!(self.remaining() > 0);
        self.block[self.index] = byte;
        self.index += 1;
    }

    /// Pushes bytes from `bytes` to the end of the block until either `bytes` returns [`None`] or
    /// the block is filled
    pub(super) fn push_bytes(&mut self, bytes: &mut dyn Iterator<Item = u8>) {
        while self.remaining() > 0 {
            let byte = match bytes.next() {
                Some(byte) => byte,
                None => return,
            };

            self.push_byte(byte);
        }
    }

    /// Pushes a slice of bytes to the end of the block
    ///
    /// # Panic
    /// This function will panic if the block already contains `SIZE` elements
    pub(super) fn push_slice(&mut self, slice: &[u8]) {
        debug_assert!(self.remaining() >= slice.len());
        self.block[self.index..self.index + slice.len()].copy_from_slice(slice);
        self.index += slice.len();
    }

    /// Consumes the underlying block, returning a [`u8`] slice and setting the index back to 0
    ///
    /// # Panic
    /// This function will panic if the block is not filled
    pub(super) fn consume(&mut self) -> &[u8] {
        assert_eq!(self.index, SIZE);
        self.index = 0;
        &self.block
    }
}
