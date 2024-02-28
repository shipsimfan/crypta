/// A type which can be used for the bit-length at the end of a hash input
pub(in crate::hash) trait BitLength: Sized + Clone + Copy {
    /// The initial count for length
    const ZERO: Self;

    /// Increase the length by one byte
    fn add_byte(&mut self);

    /// Increase the length by `count` bytes
    fn add_bytes(&mut self, count: usize);

    /// Convert the bit-length to big-endian ordered bytes
    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()];
}

impl BitLength for u64 {
    const ZERO: Self = 0;

    fn add_byte(&mut self) {
        *self += 8;
    }

    fn add_bytes(&mut self, count: usize) {
        *self += (count as u64) * 8;
    }

    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        u64::to_be_bytes(self)
    }
}

impl BitLength for u128 {
    const ZERO: Self = 0;

    fn add_byte(&mut self) {
        *self += 8;
    }

    fn add_bytes(&mut self, count: usize) {
        *self += (count as u128) * 8;
    }

    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        u128::to_be_bytes(self)
    }
}
