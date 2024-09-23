use crate::hash::{Hash, HashFunction};

impl<H: HashFunction> Default for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn default() -> Self {
        Self::ZERO
    }
}
