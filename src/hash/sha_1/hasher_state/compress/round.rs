use super::s;

#[inline(always)]
pub(super) fn round(
    a: &mut u32,
    b: &mut u32,
    c: &mut u32,
    d: &mut u32,
    e: &mut u32,
    f: fn(u32, u32, u32) -> u32,
    w: u32,
    k: u32,
) {
    let temp = s(5, *a)
        .wrapping_add(f(*b, *c, *d))
        .wrapping_add(*e)
        .wrapping_add(w)
        .wrapping_add(k);
    *e = *d;
    *d = *c;
    *c = s(30, *b);
    *b = *a;
    *a = temp;
}
