/// A trait for holding the size of a resulting hash.
///
/// This is required to avoid cycles in the [`HashFunction`](crate::hash::HashFunction) trait,
/// specifically with the [`Sized`] constraint including `Self::BYTE_SIZE`.
pub trait HashSize {
    /// The size of the output in bits
    const BIT_SIZE: usize;

    /// The size of the output in bytes
    const BYTE_SIZE: usize = (Self::BIT_SIZE + 7) / 8;
}
