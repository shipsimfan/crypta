#[inline(always)]
pub(super) fn ff<F: Fn(u32, u32, u32) -> u32>(
    a: &mut u32,
    b: u32,
    c: u32,
    d: u32,
    x: u32,
    s: u32,
    ac: u32,
    f: F,
) {
    *a =
        b.wrapping_add((a.wrapping_add(f(b, c, d)).wrapping_add(x).wrapping_add(ac)).rotate_left(s))
}
