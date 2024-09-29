#[inline(always)]
pub(super) fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

#[inline(always)]
pub(super) fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

#[inline(always)]
pub(super) fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[inline(always)]
pub(super) fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}
