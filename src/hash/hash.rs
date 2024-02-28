use super::HashFunction;
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeTo},
    str::FromStr,
};

/// Represents a hash of a given hash function
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hash<H: HashFunction>([u8; H::BYTE_SIZE])
where
    [u8; H::BYTE_SIZE]: Sized;

/// An error while parsing a [`Hash`] from a string
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParseHashError {
    /// The provided string is too short
    TooShort,

    /// The provided string contains invalid hex characters
    InvalidHex,

    /// The provided string is too long
    TooLong,
}

impl<H: HashFunction> Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    /// Creates a new hash
    pub const fn new(hash: [u8; H::BYTE_SIZE]) -> Self {
        Hash(hash)
    }

    /// Hashes `bytes` and returns the resulting hash
    pub fn create(bytes: impl AsRef<[u8]>) -> Self {
        H::hash(bytes)
    }

    /// Hashes `bytes` and returns the resulting hash
    pub fn create_iter(bytes: impl Iterator<Item = u8>) -> Self {
        H::hash_iter(bytes)
    }
    /// Returns the underlying hash
    pub const fn inner(&self) -> &[u8; H::BYTE_SIZE] {
        &self.0
    }

    /// Returns the underlying hash mutably
    pub fn inner_mut(&mut self) -> &mut [u8; H::BYTE_SIZE] {
        &mut self.0
    }

    /// Unwraps this type into the underlying hash
    pub const fn into_inner(self) -> [u8; H::BYTE_SIZE] {
        self.0
    }
}

impl<H: HashFunction> Default for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn default() -> Self {
        Hash([0; H::BYTE_SIZE])
    }
}

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

impl<H: HashFunction> PartialEq<&[u8]> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn eq(&self, other: &&[u8]) -> bool {
        self.0.eq(other)
    }
}

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

impl<H: HashFunction> TryFrom<String> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    type Error = ParseHashError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
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

impl<H: HashFunction> Into<[u8; H::BYTE_SIZE]> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn into(self) -> [u8; H::BYTE_SIZE] {
        self.0
    }
}

impl<H: HashFunction> From<[u8; H::BYTE_SIZE]> for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn from(value: [u8; H::BYTE_SIZE]) -> Self {
        Hash(value)
    }
}

impl<H: HashFunction> Display for Hash<H>
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

impl<H: HashFunction> Debug for Hash<H>
where
    [u8; H::BYTE_SIZE]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
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
