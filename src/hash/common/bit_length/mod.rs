mod u32;
mod u64;

/// A type which can be used for the bit-length at the end of a hash input
pub(in crate::hash) trait BitLength: Sized + Clone + Copy {
    /// The initial count for length
    const ZERO: Self;

    /// Increase the length by `count` bytes
    fn add_bytes(&mut self, count: usize);

    /// Convert the bit-length to big-endian ordered bytes
    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()];
}
