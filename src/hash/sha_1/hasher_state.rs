use crate::hash::common::HasherState;

/// The state used by SHA-1 hash functions
pub struct SHA1HasherState([u32; 5]);

const INITIAL_HASH_VALUE: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
const K: [u32; 4] = [0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6];

fn s(n: u32, x: u32) -> u32 {
    x.rotate_left(n)
}

fn f1(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (!b & d)
}

fn f2(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

fn f3(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (b & d) | (c & d)
}

#[inline(always)]
fn round(
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

impl SHA1HasherState {
    /// Creates a new [`SHA2HasherState`]
    pub(super) fn new() -> Self {
        SHA1HasherState(INITIAL_HASH_VALUE)
    }

    /// Unwraps the state to the contained hash
    pub(super) fn unwrap(self) -> [u8; 20] {
        let mut output = [0; 20];

        for i in 0..5 {
            let base = i * 4;

            output[base..base + 4].copy_from_slice(&self.0[i].to_be_bytes());
        }

        output
    }
}

impl HasherState for SHA1HasherState {
    const BLOCK_SIZE: usize = 64;

    fn compress(&mut self, block: &[u8]) {
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

        // Initialize the working variables
        let mut a = self.0[0];
        let mut b = self.0[1];
        let mut c = self.0[2];
        let mut d = self.0[3];
        let mut e = self.0[4];

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

        // Compute the intermediate hash value
        self.0[0] = a.wrapping_add(self.0[0]);
        self.0[1] = b.wrapping_add(self.0[1]);
        self.0[2] = c.wrapping_add(self.0[2]);
        self.0[3] = d.wrapping_add(self.0[3]);
        self.0[4] = e.wrapping_add(self.0[4]);
    }
}
