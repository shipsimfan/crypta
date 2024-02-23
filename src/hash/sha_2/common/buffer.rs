use super::Word;
use std::ops::{Index, IndexMut};

/// A buffer of words for hashing
pub(super) union Buffer<W: Word, const SIZE: usize>
where
    [u8; SIZE * std::mem::size_of::<W>()]: Sized,
    [W; SIZE]: Sized,
{
    /// The buffer as bytes, for adding and padding
    bytes: [u8; SIZE * std::mem::size_of::<W>()],

    /// The buffer as words, for hashing
    words: [W; SIZE],
}

const fn to_be_index(index: usize, word_size: usize) -> usize {
    let word = index / word_size;
    let byte = index % word_size;

    (word * word_size) + (word_size - byte - 1)
}

impl<W: Word, const SIZE: usize> Buffer<W, SIZE>
where
    [u8; SIZE * std::mem::size_of::<W>()]: Sized,
    [W; SIZE]: Sized,
{
    /// Creates a new [`Buffer`]
    pub(super) fn new() -> Self {
        Buffer {
            words: [W::ZERO; SIZE],
        }
    }

    /// Gets the buffer as words
    pub(super) const fn as_words(&self) -> &[W] {
        unsafe { &self.words }
    }

    /// Gets the length of the contained buffer
    pub(super) const fn len(&self) -> usize {
        SIZE * std::mem::size_of::<W>()
    }
}

impl<W: Word, const SIZE: usize> Index<usize> for Buffer<W, SIZE>
where
    [u8; SIZE * std::mem::size_of::<W>()]: Sized,
    [W; SIZE]: Sized,
{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.bytes[to_be_index(index, std::mem::size_of::<W>())] }
    }
}

impl<W: Word, const SIZE: usize> IndexMut<usize> for Buffer<W, SIZE>
where
    [u8; SIZE * std::mem::size_of::<W>()]: Sized,
    [W; SIZE]: Sized,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.bytes[to_be_index(index, std::mem::size_of::<W>())] }
    }
}
