use super::{LONG_K, SHORT_K};
use std::{
    fmt::{Display, UpperHex},
    ops::{BitAnd, BitOr, BitXor, Not, Shr},
};

pub(in crate::hash::md5_like::sha2) trait Word:
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
    const K: &'static [Self];
    const ROUNDS: usize;
    const BLOCK_SIZE: usize;

    const ZERO: Self;

    fn from_be_bytes(bytes: &[u8], index: usize) -> Self;
    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()];

    fn add(self, other: Self) -> Self;

    fn rotr(self, n: u32) -> Self;

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
    const K: &'static [Self] = &SHORT_K;
    const ROUNDS: usize = 64;
    const BLOCK_SIZE: usize = 64;

    const ZERO: Self = 0;

    fn from_be_bytes(bytes: &[u8], index: usize) -> Self {
        let index = index * 4;
        u32::from_be_bytes([
            bytes[index],
            bytes[index + 1],
            bytes[index + 2],
            bytes[index + 3],
        ])
    }

    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        self.to_be_bytes()
    }

    fn add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }

    fn rotr(self, n: u32) -> Self {
        self.rotate_right(n)
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
    const K: &'static [Self] = &LONG_K;
    const ROUNDS: usize = 80;
    const BLOCK_SIZE: usize = 128;

    const ZERO: Self = 0;

    fn from_be_bytes(bytes: &[u8], index: usize) -> Self {
        let index = index * 8;
        u64::from_be_bytes([
            bytes[index],
            bytes[index + 1],
            bytes[index + 2],
            bytes[index + 3],
            bytes[index + 4],
            bytes[index + 5],
            bytes[index + 6],
            bytes[index + 7],
        ])
    }

    fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        self.to_be_bytes()
    }

    fn add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }

    fn rotr(self, n: u32) -> Self {
        self.rotate_right(n)
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
