use std::ops::{BitAnd, BitOr, BitXor, Not, Shr};

pub(in crate::hash::sha_2) trait Word:
    Sized
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + Not<Output = Self>
    + Shr<u32, Output = Self>
    + Clone
    + Copy
{
    fn add(self, other: Self) -> Self;

    fn rotr(self, n: u32) -> Self;
    fn rotl(self, n: u32) -> Self;
}

impl Word for u32 {
    fn add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }

    fn rotr(self, n: u32) -> Self {
        self.rotate_right(n)
    }

    fn rotl(self, n: u32) -> Self {
        self.rotate_left(n)
    }
}

impl Word for u64 {
    fn add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }

    fn rotr(self, n: u32) -> Self {
        self.rotate_right(n)
    }

    fn rotl(self, n: u32) -> Self {
        self.rotate_left(n)
    }
}
