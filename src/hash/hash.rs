use crate::hash::HashFunction;
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Represents a hash of a given hash function
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hash<H: HashFunction>([u8; H::SIZE])
where
    [u8; H::SIZE]: Sized;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParseHashError {
    TooShort,
    InvalidHex,
    TooLong,
}

impl<H: HashFunction> Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    /// Creates a new hash
    pub const fn new(hash: [u8; H::SIZE]) -> Self {
        Hash(hash)
    }

    /// Hashes source and returns the resulting hash
    pub fn create<I: IntoIterator<Item = u8>>(source: I) -> Self {
        H::hash(source)
    }

    /// Returns the underlying hash
    pub const fn inner(&self) -> &[u8; H::SIZE] {
        &self.0
    }

    /// Returns the underlying hash mutably
    pub fn inner_mut(&mut self) -> &mut [u8; H::SIZE] {
        &mut self.0
    }

    /// Unwraps this type into the underlying hash
    pub const fn into_inner(self) -> [u8; H::SIZE] {
        self.0
    }
}

impl<H: HashFunction> FromStr for Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    type Err = ParseHashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut hash = [0; H::SIZE];

        for i in 0..H::SIZE {
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

impl<H: HashFunction> TryFrom<String> for Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    type Error = ParseHashError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<H: HashFunction> TryFrom<&str> for Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    type Error = ParseHashError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<H: HashFunction> Deref for Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    type Target = [u8; H::SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<H: HashFunction> DerefMut for Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<H: HashFunction> Into<[u8; H::SIZE]> for Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    fn into(self) -> [u8; H::SIZE] {
        self.0
    }
}

impl<H: HashFunction> From<[u8; H::SIZE]> for Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    fn from(value: [u8; H::SIZE]) -> Self {
        Hash(value)
    }
}

impl<H: HashFunction> Display for Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{:X}", byte)?;
        }

        Ok(())
    }
}

impl<H: HashFunction> Debug for Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<H: HashFunction> PartialEq<&[u8]> for Hash<H>
where
    [u8; H::SIZE]: Sized,
{
    fn eq(&self, other: &&[u8]) -> bool {
        self.0.eq(other)
    }
}

impl std::error::Error for ParseHashError {}

impl Display for ParseHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseHashError::TooShort => write!(f, "hash is too short"),
            ParseHashError::InvalidHex => {
                write!(f, "hash contains characters which are not hex digits")
            }
            ParseHashError::TooLong => write!(f, "hash is too long"),
        }
    }
}

impl Debug for ParseHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
