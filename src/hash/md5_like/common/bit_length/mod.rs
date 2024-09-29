mod u32;
mod u64;

/// A type which can be used for the bit-length at the end of a hash input
pub(in crate::hash::md5_like) trait BitLength:
    Sized + Clone + Copy
{
    /// The initial count for length
    const ZERO: Self;

    /// Increase the length by `count` bytes
    fn add_bytes(&mut self, count: usize);

    /// Convert the bit-length to big-endian ordered bytes
    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()];

    /// Convert the bit-length to little-endian ordered bytes
    fn to_le_bytes(self) -> [u8; std::mem::size_of::<Self>()];
}
