use crate::hash::md5_like::common::HasherState;
use compress::compress;

mod compress;

/// The state used by MD5 hash function
pub struct MD5HasherState([u32; 4]);

const INITIAL_HASH_VALUE: [u32; 4] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476];

impl MD5HasherState {
    /// Creates a new [`MD5HasherState`]
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

    const BIG_ENDIAN_LENGTH: bool = false;

    fn compress(&mut self, block: &[u8]) {
        let (a, b, c, d) = compress(block, self.0[0], self.0[1], self.0[2], self.0[3]);

        self.0[0] = self.0[0].wrapping_add(a);
        self.0[1] = self.0[1].wrapping_add(b);
        self.0[2] = self.0[2].wrapping_add(c);
        self.0[3] = self.0[3].wrapping_add(d);
    }
}
