use crate::hash::common::Block;

impl<const SIZE: usize> Block<SIZE>
where
    [u8; SIZE]: Sized,
{
    /// Pushes `byte` to the end of the block
    ///
    /// # Panic
    /// This function will panic if the block already contains `SIZE` elements
    pub(in crate::hash::common) fn push_byte(&mut self, byte: u8) {
        debug_assert!(self.remaining() > 0);
        self.block[self.index] = byte;
        self.index += 1;
    }

    /// Pushes bytes from `bytes` to the end of the block until either `bytes` returns [`None`] or
    /// the block is filled
    ///
    /// Returns the number of bytes pushed
    pub(in crate::hash::common) fn push_bytes(
        &mut self,
        bytes: &mut dyn Iterator<Item = u8>,
    ) -> usize {
        let mut count = 0;
        while self.remaining() > 0 {
            let byte = match bytes.next() {
                Some(byte) => byte,
                None => break,
            };

            self.push_byte(byte);
            count += 1;
        }

        count
    }

    /// Pushes a slice of bytes to the end of the block
    ///
    /// # Panic
    /// This function will panic if the block already contains `SIZE` elements
    pub(in crate::hash::common) fn push_slice(&mut self, slice: &[u8]) {
        debug_assert!(self.remaining() >= slice.len());
        self.block[self.index..self.index + slice.len()].copy_from_slice(slice);
        self.index += slice.len();
    }

    /// Consumes the underlying block, returning a [`u8`] slice and setting the index back to 0
    ///
    /// # Panic
    /// This function will panic if the block is not filled
    pub(in crate::hash::common) fn consume(&mut self) -> &[u8] {
        assert_eq!(self.index, SIZE);
        self.index = 0;
        &self.block
    }
}
