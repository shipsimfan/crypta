#[inline(always)]
pub(super) fn f1(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (!b & d)
}

#[inline(always)]
pub(super) fn f2(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

#[inline(always)]
pub(super) fn f3(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (b & d) | (c & d)
}
