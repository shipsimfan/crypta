use crate::hash::{Hash, HashFunction};

impl<H: HashFunction> std::fmt::Display for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}
