use crate::hash::{Hash, HashFunction};

impl<H: HashFunction> PartialEq<[u8]> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn eq(&self, other: &[u8]) -> bool {
        self.0.eq(other)
    }
}
