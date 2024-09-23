use crate::hash::{Hash, HashFunction};

impl<H: HashFunction> Into<[u8; H::BYTE_SIZE]> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn into(self) -> [u8; H::BYTE_SIZE] {
        self.0
    }
}
