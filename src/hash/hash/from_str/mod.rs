use crate::hash::{Hash, HashFunction};
use core::str::FromStr;

mod error;

pub use error::ParseHashError;

impl<H: HashFunction> FromStr for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    type Err = ParseHashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut hash = [0; H::BYTE_SIZE];

        for i in 0..H::BYTE_SIZE {
            let mut byte = 0;
            for _ in 0..2 {
                byte *= 16;
                byte += chars
                    .next()
                    .ok_or(ParseHashError::TooShort)?
                    .to_digit(16)
                    .ok_or(ParseHashError::InvalidHex)? as u8;
            }

            hash[i] = byte;
        }

        if chars.next().is_none() {
            Ok(Hash(hash))
        } else {
            Err(ParseHashError::TooLong)
        }
    }
}

#[cfg(feature = "alloc")]
impl<H: HashFunction> TryFrom<alloc::string::String> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    type Error = ParseHashError;

    fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<H: HashFunction> TryFrom<&str> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    type Error = ParseHashError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
