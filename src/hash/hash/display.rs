use crate::hash::{Hash, HashFunction};

impl<H: HashFunction> core::fmt::Display for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}
