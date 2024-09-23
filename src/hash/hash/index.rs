use crate::hash::{Hash, HashFunction};
use std::ops::{Index, IndexMut, Range, RangeFrom, RangeFull, RangeTo};

impl<H: HashFunction> Index<usize> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<H: HashFunction> IndexMut<usize> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<H: HashFunction> Index<Range<usize>> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<H: HashFunction> IndexMut<Range<usize>> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<H: HashFunction> Index<RangeFrom<usize>> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<H: HashFunction> IndexMut<RangeFrom<usize>> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<H: HashFunction> Index<RangeTo<usize>> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<H: HashFunction> IndexMut<RangeTo<usize>> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<H: HashFunction> Index<RangeFull> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    type Output = [u8];

    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

impl<H: HashFunction> IndexMut<RangeFull> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        &mut self.0[index]
    }
}
