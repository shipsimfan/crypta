use crate::hash::md5_like::common::BitLength;

impl BitLength for u64 {
    const ZERO: Self = 0;

    fn add_bytes(&mut self, count: usize) {
        *self += (count as u64) * 8;
    }

    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        u64::to_be_bytes(self)
    }

    fn to_le_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        u64::to_le_bytes(self)
    }
}
