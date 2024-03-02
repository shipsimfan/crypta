use crate::hash::common::HasherState;

/// The state used by MD5 hash function
pub struct MD5HasherState([u32; 4]);

const INITIAL_HASH_VALUE: [u32; 4] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476];

fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}

#[inline(always)]
fn ff<F: Fn(u32, u32, u32) -> u32>(
    a: &mut u32,
    b: u32,
    c: u32,
    d: u32,
    x: u32,
    s: u32,
    ac: u32,
    f: F,
) {
    *a = a.wrapping_add(f(b, c, d).wrapping_add(x).wrapping_add(ac));
    *a = a.rotate_left(s);
    *a = a.wrapping_add(b);
}

impl MD5HasherState {
    /// Creates a new [`SHA2HasherState`]
    pub(super) fn new() -> Self {
        MD5HasherState(INITIAL_HASH_VALUE)
    }

    /// Unwraps the state to the contained hash
    pub(super) fn unwrap(self) -> [u8; 16] {
        let mut output = [0; 16];

        for i in 0..4 {
            let base = i * 4;

            output[base..base + 4].copy_from_slice(&self.0[i].to_le_bytes());
        }

        output
    }
}

impl HasherState for MD5HasherState {
    const BLOCK_SIZE: usize = 64;

    fn compress(&mut self, block: &[u8]) {
        let mut a = self.0[0];
        let mut b = self.0[1];
        let mut c = self.0[2];
        let mut d = self.0[3];

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

        ff(&mut a, b, c, d, x[0], 7, 0xD76AA478, f); // 1
        ff(&mut d, a, b, c, x[1], 12, 0xE8C7B756, f); // 2
        ff(&mut c, d, a, b, x[2], 17, 0x242070DB, f); // 3
        ff(&mut b, c, d, a, x[3], 22, 0xC1BDCEEE, f); // 4
        ff(&mut a, b, c, d, x[4], 7, 0xF57C0FAF, f); // 5
        ff(&mut d, a, b, c, x[5], 12, 0x4787C62A, f); // 6
        ff(&mut c, d, a, b, x[6], 17, 0xA8304613, f); // 7
        ff(&mut b, c, d, a, x[7], 22, 0xFD469501, f); // 8
        ff(&mut a, b, c, d, x[8], 7, 0x698098D8, f); // 9
        ff(&mut d, a, b, c, x[9], 12, 0x8B44F7AF, f); // 10
        ff(&mut c, d, a, b, x[10], 17, 0xFFFF5BB1, f); // 11
        ff(&mut b, c, d, a, x[11], 22, 0x895CD7BE, f); // 12
        ff(&mut a, b, c, d, x[12], 7, 0x6B901122, f); // 13
        ff(&mut d, a, b, c, x[13], 12, 0xFD987193, f); // 14
        ff(&mut c, d, a, b, x[14], 17, 0xA679438E, f); // 15
        ff(&mut b, c, d, a, x[15], 22, 0x49B40821, f); // 16

        ff(&mut a, b, c, d, x[1], 5, 0xF61E2562, g); // 17
        ff(&mut d, a, b, c, x[6], 9, 0xC040B340, g); // 18
        ff(&mut c, d, a, b, x[11], 14, 0x265E5A51, g); // 19
        ff(&mut b, c, d, a, x[0], 20, 0xE9B6C7AA, g); // 20
        ff(&mut a, b, c, d, x[5], 5, 0xD62F105D, g); // 21
        ff(&mut d, a, b, c, x[10], 9, 0x2441453, g); // 22
        ff(&mut c, d, a, b, x[15], 14, 0xD8A1E681, g); // 23
        ff(&mut b, c, d, a, x[4], 20, 0xE7D3FBC8, g); // 24
        ff(&mut a, b, c, d, x[9], 5, 0x21E1CDE6, g); // 25
        ff(&mut d, a, b, c, x[14], 9, 0xC33707D6, g); // 26
        ff(&mut c, d, a, b, x[3], 14, 0xF4D50D87, g); // 27
        ff(&mut b, c, d, a, x[8], 20, 0x455A14ED, g); // 28
        ff(&mut a, b, c, d, x[13], 5, 0xA9E3E905, g); // 29
        ff(&mut d, a, b, c, x[2], 9, 0xFCEFA3F8, g); // 30
        ff(&mut c, d, a, b, x[7], 14, 0x676F02D9, g); // 31
        ff(&mut b, c, d, a, x[12], 20, 0x8D2A4C8A, g); // 32

        ff(&mut a, b, c, d, x[5], 4, 0xFFFA3942, h); // 33
        ff(&mut d, a, b, c, x[8], 11, 0x8771F681, h); // 34
        ff(&mut c, d, a, b, x[11], 16, 0x6D9D6122, h); // 35
        ff(&mut b, c, d, a, x[14], 23, 0xFDE5380C, h); // 36
        ff(&mut a, b, c, d, x[1], 4, 0xA4BEEA44, h); // 37
        ff(&mut d, a, b, c, x[4], 11, 0x4BDECFA9, h); // 38
        ff(&mut c, d, a, b, x[7], 16, 0xF6BB4B60, h); // 39
        ff(&mut b, c, d, a, x[10], 23, 0xBEBFBC70, h); // 40
        ff(&mut a, b, c, d, x[13], 4, 0x289B7EC6, h); // 41
        ff(&mut d, a, b, c, x[0], 11, 0xEAA127FA, h); // 42
        ff(&mut c, d, a, b, x[3], 16, 0xD4EF3085, h); // 43
        ff(&mut b, c, d, a, x[6], 23, 0x4881D05, h); // 44
        ff(&mut a, b, c, d, x[9], 4, 0xD9D4D039, h); // 45
        ff(&mut d, a, b, c, x[12], 11, 0xE6DB99E5, h); // 46
        ff(&mut c, d, a, b, x[15], 16, 0x1FA27CF8, h); // 47
        ff(&mut b, c, d, a, x[2], 23, 0xC4AC5665, h); // 48

        ff(&mut a, b, c, d, x[0], 6, 0xF4292244, i); // 49
        ff(&mut d, a, b, c, x[7], 10, 0x432AFF97, i); // 50
        ff(&mut c, d, a, b, x[14], 15, 0xAB9423A7, i); // 51
        ff(&mut b, c, d, a, x[5], 21, 0xFC93A039, i); // 52
        ff(&mut a, b, c, d, x[12], 6, 0x655B59C3, i); // 53
        ff(&mut d, a, b, c, x[3], 10, 0x8F0CCC92, i); // 54
        ff(&mut c, d, a, b, x[10], 15, 0xFFEFF47D, i); // 55
        ff(&mut b, c, d, a, x[1], 21, 0x85845DD1, i); // 56
        ff(&mut a, b, c, d, x[8], 6, 0x6FA87E4F, i); // 57
        ff(&mut d, a, b, c, x[15], 10, 0xFE2CE6E0, i); // 58
        ff(&mut c, d, a, b, x[6], 15, 0xA3014314, i); // 59
        ff(&mut b, c, d, a, x[13], 21, 0x4E0811A1, i); // 60
        ff(&mut a, b, c, d, x[4], 6, 0xF7537E82, i); // 61
        ff(&mut d, a, b, c, x[11], 10, 0xBD3AF235, i); // 62
        ff(&mut c, d, a, b, x[2], 15, 0x2AD7D2BB, i); // 63
        ff(&mut b, c, d, a, x[9], 21, 0xEB86D391, i); // 64

        self.0[0] = self.0[0].wrapping_add(a);
        self.0[1] = self.0[1].wrapping_add(b);
        self.0[2] = self.0[2].wrapping_add(c);
        self.0[3] = self.0[3].wrapping_add(d);
    }
}
