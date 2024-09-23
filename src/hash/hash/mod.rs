use crate::hash::HashFunction;

mod default;
mod deref;
mod display;
mod eq;
mod from;
mod from_str;
mod index;
mod into;
mod new;

pub use from_str::ParseHashError;

/// Represents a hash of a given hash function
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash<H: HashFunction>(pub [u8; H::BYTE_SIZE])
where
    [u8; H::BYTE_SIZE]: Sized;
