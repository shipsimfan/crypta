use round::round;
use step_fns::{f1, f2, f3};

mod round;
mod step_fns;

const K: [u32; 4] = [0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6];

#[inline(always)]
fn s(n: u32, x: u32) -> u32 {
    x.rotate_left(n)
}

#[inline(always)]
pub(super) fn compress(
    block: &[u8],
    mut a: u32,
    mut b: u32,
    mut c: u32,
    mut d: u32,
    mut e: u32,
) -> (u32, u32, u32, u32, u32) {
    // Prepare the message schedule
    let mut w = [0; 80];
    for t in 0..16 {
        w[t] = u32::from_be_bytes([
            block[t * 4],
            block[t * 4 + 1],
            block[t * 4 + 2],
            block[t * 4 + 3],
        ]);
    }
    for t in 16..80 {
        w[t] = s(1, w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16]);
    }

    // Perform the main hash computation
    for t in 0..20 {
        round(&mut a, &mut b, &mut c, &mut d, &mut e, f1, w[t], K[0]);
    }
    for t in 20..40 {
        round(&mut a, &mut b, &mut c, &mut d, &mut e, f2, w[t], K[1]);
    }
    for t in 40..60 {
        round(&mut a, &mut b, &mut c, &mut d, &mut e, f3, w[t], K[2]);
    }
    for t in 60..80 {
        round(&mut a, &mut b, &mut c, &mut d, &mut e, f2, w[t], K[3]);
    }

    (a, b, c, d, e)
}
