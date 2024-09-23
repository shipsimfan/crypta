use crate::hash::common::BitLength;

impl BitLength for u128 {
    const ZERO: Self = 0;

    fn add_bytes(&mut self, count: usize) {
        *self += (count as u128) * 8;
    }

    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        u128::to_be_bytes(self)
    }
}
