use crate::hash::{Hash, HashFunction};
use std::ops::{Deref, DerefMut};

impl<H: HashFunction> Deref for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    type Target = [u8; H::BYTE_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<H: HashFunction> DerefMut for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<H: HashFunction> AsRef<[u8]> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<H: HashFunction> AsMut<[u8]> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
