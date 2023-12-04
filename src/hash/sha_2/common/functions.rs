use super::Word;

fn bsig<W: Word, const R1: u32, const R2: u32, const R3: u32>(x: W) -> W {
    x.rotr(R1) ^ x.rotr(R2) ^ x.rotr(R3)
}

fn ssig<W: Word, const R1: u32, const R2: u32, const S: u32>(x: W) -> W {
    x.rotr(R1) ^ x.rotr(R2) ^ x.shr(S)
}

pub(in crate::hash::sha_2) fn ch<W: Word>(x: W, y: W, z: W) -> W {
    (x & y) ^ ((!x) & z)
}

pub(in crate::hash::sha_2) fn maj<W: Word>(x: W, y: W, z: W) -> W {
    (x & y) ^ (x & z) ^ (y & z)
}

pub(in crate::hash::sha_2) fn short_bsig0(x: u32) -> u32 {
    bsig::<_, 2, 13, 22>(x)
}

pub(in crate::hash::sha_2) fn short_bsig1(x: u32) -> u32 {
    bsig::<_, 6, 11, 25>(x)
}

pub(in crate::hash::sha_2) fn long_bsig0(x: u64) -> u64 {
    bsig::<_, 28, 34, 39>(x)
}

pub(in crate::hash::sha_2) fn long_bsig1(x: u64) -> u64 {
    bsig::<_, 14, 18, 41>(x)
}

pub(in crate::hash::sha_2) fn short_ssig0(x: u32) -> u32 {
    ssig::<_, 7, 18, 3>(x)
}

pub(in crate::hash::sha_2) fn short_ssig1(x: u32) -> u32 {
    ssig::<_, 17, 19, 10>(x)
}

pub(in crate::hash::sha_2) fn long_ssig0(x: u64) -> u64 {
    ssig::<_, 1, 8, 7>(x)
}

pub(in crate::hash::sha_2) fn long_ssig1(x: u64) -> u64 {
    ssig::<_, 19, 61, 6>(x)
}
