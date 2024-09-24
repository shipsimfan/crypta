use ff::ff;
use round::round;
use step_fns::{f, g, h, i};
use t::T;

mod ff;
mod round;
mod step_fns;
mod t;

#[inline(always)]
pub(super) fn compress(
    block: &[u8],
    mut a: u32,
    mut b: u32,
    mut c: u32,
    mut d: u32,
) -> (u32, u32, u32, u32) {
    let mut x = [0; 16];
    for i in 0..16 {
        let base = i * 4;
        x[i] = u32::from_le_bytes([
            block[base],
            block[base + 1],
            block[base + 2],
            block[base + 3],
        ]);
    }

    // F

    round!(f, a, b, c, d, 0, 7, 0, x);
    round!(f, d, a, b, c, 1, 12, 1, x);
    round!(f, c, d, a, b, 2, 17, 2, x);
    round!(f, b, c, d, a, 3, 22, 3, x);

    round!(f, a, b, c, d, 4, 7, 4, x);
    round!(f, d, a, b, c, 5, 12, 5, x);
    round!(f, c, d, a, b, 6, 17, 6, x);
    round!(f, b, c, d, a, 7, 22, 7, x);

    round!(f, a, b, c, d, 8, 7, 8, x);
    round!(f, d, a, b, c, 9, 12, 9, x);
    round!(f, c, d, a, b, 10, 17, 10, x);
    round!(f, b, c, d, a, 11, 22, 11, x);

    round!(f, a, b, c, d, 12, 7, 12, x);
    round!(f, d, a, b, c, 13, 12, 13, x);
    round!(f, c, d, a, b, 14, 17, 14, x);
    round!(f, b, c, d, a, 15, 22, 15, x);

    // G

    round!(g, a, b, c, d, 1, 5, 16, x);
    round!(g, d, a, b, c, 6, 9, 17, x);
    round!(g, c, d, a, b, 11, 14, 18, x);
    round!(g, b, c, d, a, 0, 20, 19, x);

    round!(g, a, b, c, d, 5, 5, 20, x);
    round!(g, d, a, b, c, 10, 9, 21, x);
    round!(g, c, d, a, b, 15, 14, 22, x);
    round!(g, b, c, d, a, 4, 20, 23, x);

    round!(g, a, b, c, d, 9, 5, 24, x);
    round!(g, d, a, b, c, 14, 9, 25, x);
    round!(g, c, d, a, b, 3, 14, 26, x);
    round!(g, b, c, d, a, 8, 20, 27, x);

    round!(g, a, b, c, d, 13, 5, 28, x);
    round!(g, d, a, b, c, 2, 9, 29, x);
    round!(g, c, d, a, b, 7, 14, 30, x);
    round!(g, b, c, d, a, 12, 20, 31, x);

    // H

    round!(h, a, b, c, d, 5, 4, 32, x);
    round!(h, d, a, b, c, 8, 11, 33, x);
    round!(h, c, d, a, b, 11, 16, 34, x);
    round!(h, b, c, d, a, 14, 23, 35, x);

    round!(h, a, b, c, d, 1, 4, 36, x);
    round!(h, d, a, b, c, 4, 11, 37, x);
    round!(h, c, d, a, b, 7, 16, 38, x);
    round!(h, b, c, d, a, 10, 23, 39, x);

    round!(h, a, b, c, d, 13, 4, 40, x);
    round!(h, d, a, b, c, 0, 11, 41, x);
    round!(h, c, d, a, b, 3, 16, 42, x);
    round!(h, b, c, d, a, 6, 23, 43, x);

    round!(h, a, b, c, d, 9, 4, 44, x);
    round!(h, d, a, b, c, 12, 11, 45, x);
    round!(h, c, d, a, b, 15, 16, 46, x);
    round!(h, b, c, d, a, 2, 23, 47, x);

    // I

    round!(i, a, b, c, d, 0, 6, 48, x);
    round!(i, d, a, b, c, 7, 10, 49, x);
    round!(i, c, d, a, b, 14, 15, 50, x);
    round!(i, b, c, d, a, 5, 21, 51, x);

    round!(i, a, b, c, d, 12, 6, 52, x);
    round!(i, d, a, b, c, 3, 10, 53, x);
    round!(i, c, d, a, b, 10, 15, 54, x);
    round!(i, b, c, d, a, 1, 21, 55, x);

    round!(i, a, b, c, d, 8, 6, 56, x);
    round!(i, d, a, b, c, 15, 10, 57, x);
    round!(i, c, d, a, b, 6, 15, 58, x);
    round!(i, b, c, d, a, 13, 21, 59, x);

    round!(i, a, b, c, d, 4, 6, 60, x);
    round!(i, d, a, b, c, 11, 10, 61, x);
    round!(i, c, d, a, b, 2, 15, 62, x);
    round!(i, b, c, d, a, 9, 21, 63, x);

    (a, b, c, d)
}
