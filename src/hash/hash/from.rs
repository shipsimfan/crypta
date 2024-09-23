use crate::hash::{Hash, HashFunction};

impl<H: HashFunction> From<[u8; H::BYTE_SIZE]> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn from(value: [u8; H::BYTE_SIZE]) -> Self {
        Hash(value)
    }
}
