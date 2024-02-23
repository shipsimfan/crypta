use super::{LONG_K, SHORT_K};
use std::{
    fmt::{Display, UpperHex},
    ops::{AddAssign, BitAnd, BitOr, BitXor, Not, Shr},
};

pub(in crate::hash::sha_2) trait HashLength:
    Sized + AddAssign + Clone + Copy
{
    const ZERO: Self;
    const ONE: Self;

    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()];
}

pub(in crate::hash::sha_2) trait Word:
    'static
    + Sized
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + Not<Output = Self>
    + Shr<u32, Output = Self>
    + Clone
    + Copy
    + Display
    + UpperHex
{
    type Length: HashLength;

    const K: &'static [Self];
    const ZERO: Self;

    fn swap_endian(self) -> Self;

    fn add(self, other: Self) -> Self;

    fn rotr(self, n: u32) -> Self;
    fn rotl(self, n: u32) -> Self;

    fn ch(x: Self, y: Self, z: Self) -> Self {
        (x & y) ^ ((!x) & z)
    }

    fn maj(x: Self, y: Self, z: Self) -> Self {
        (x & y) ^ (x & z) ^ (y & z)
    }

    fn bsig<const R1: u32, const R2: u32, const R3: u32>(self) -> Self {
        self.rotr(R1) ^ self.rotr(R2) ^ self.rotr(R3)
    }

    fn bsig0(self) -> Self;
    fn bsig1(self) -> Self;

    fn ssig<const R1: u32, const R2: u32, const S: u32>(self) -> Self {
        self.rotr(R1) ^ self.rotr(R2) ^ self.shr(S)
    }

    fn ssig0(self) -> Self;
    fn ssig1(self) -> Self;
}

impl Word for u32 {
    type Length = u64;

    const K: &'static [Self] = &SHORT_K;
    const ZERO: Self = 0;

    fn swap_endian(self) -> Self {
        u32::from_be_bytes(self.to_le_bytes())
    }

    fn add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }

    fn rotr(self, n: u32) -> Self {
        self.rotate_right(n)
    }

    fn rotl(self, n: u32) -> Self {
        self.rotate_left(n)
    }

    fn bsig0(self) -> Self {
        self.bsig::<2, 13, 22>()
    }

    fn bsig1(self) -> Self {
        self.bsig::<6, 11, 25>()
    }

    fn ssig0(self) -> Self {
        self.ssig::<7, 18, 3>()
    }

    fn ssig1(self) -> Self {
        self.ssig::<17, 19, 10>()
    }
}

impl Word for u64 {
    type Length = u128;

    const K: &'static [Self] = &LONG_K;
    const ZERO: Self = 0;

    fn swap_endian(self) -> Self {
        u64::from_be_bytes(self.to_le_bytes())
    }

    fn add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }

    fn rotr(self, n: u32) -> Self {
        self.rotate_right(n)
    }

    fn rotl(self, n: u32) -> Self {
        self.rotate_left(n)
    }

    fn bsig0(self) -> Self {
        self.bsig::<28, 34, 39>()
    }

    fn bsig1(self) -> Self {
        self.bsig::<14, 18, 41>()
    }

    fn ssig0(self) -> Self {
        self.ssig::<1, 8, 7>()
    }

    fn ssig1(self) -> Self {
        self.ssig::<19, 61, 6>()
    }
}

impl HashLength for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        self.to_be_bytes()
    }
}

impl HashLength for u128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        self.to_be_bytes()
    }
}
