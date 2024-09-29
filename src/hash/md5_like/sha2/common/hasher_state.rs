use super::Word;
use crate::hash::md5_like::common::HasherState;

/// The state used by SHA-2 hash functions
pub struct SHA2HasherState<W: Word>([W; 8])
where
    [u8; W::ROUNDS]: Sized;

impl<W: Word> SHA2HasherState<W>
where
    [u8; W::ROUNDS]: Sized,
{
    /// Creates a new [`SHA2HasherState`]
    pub(in crate::hash::md5_like::sha2) fn new(initial_state: [W; 8]) -> Self {
        SHA2HasherState(initial_state)
    }

    /// Unwraps the state to the contained hash
    pub(in crate::hash::md5_like::sha2) fn unwrap<const COUNT: usize>(
        self,
    ) -> [u8; COUNT * core::mem::size_of::<W>()] {
        let mut output = [0; COUNT * core::mem::size_of::<W>()];

        for i in 0..COUNT {
            let base = i * core::mem::size_of::<W>();

            output[base..base + core::mem::size_of::<W>()]
                .copy_from_slice(&self.0[i].to_be_bytes());
        }

        output
    }
}

impl<W: Word> HasherState for SHA2HasherState<W>
where
    [u8; W::ROUNDS]: Sized,
{
    const BLOCK_SIZE: usize = W::BLOCK_SIZE;

    fn compress(&mut self, block: &[u8]) {
        let mut w = [W::ZERO; W::ROUNDS];

        // Prepare the message schedule
        for t in 0..16 {
            w[t] = W::from_be_bytes(block, t);
        }
        for t in 16..W::ROUNDS {
            w[t] = (w[t - 2].ssig1())
                .add(w[t - 7])
                .add(w[t - 15].ssig0())
                .add(w[t - 16]);
        }

        // Initialize the working variables
        let mut a = self.0[0];
        let mut b = self.0[1];
        let mut c = self.0[2];
        let mut d = self.0[3];
        let mut e = self.0[4];
        let mut f = self.0[5];
        let mut g = self.0[6];
        let mut h = self.0[7];

        // Perform the main hash computation
        for t in 0..W::ROUNDS {
            let t1 = h.add(e.bsig1()).add(W::ch(e, f, g)).add(W::K[t]).add(w[t]);
            let t2 = a.bsig0().add(W::maj(a, b, c));
            h = g;
            g = f;
            f = e;
            e = d.add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.add(t2);
        }

        // Compute the intermediate hash value
        self.0[0] = a.add(self.0[0]);
        self.0[1] = b.add(self.0[1]);
        self.0[2] = c.add(self.0[2]);
        self.0[3] = d.add(self.0[3]);
        self.0[4] = e.add(self.0[4]);
        self.0[5] = f.add(self.0[5]);
        self.0[6] = g.add(self.0[6]);
        self.0[7] = h.add(self.0[7]);
    }
}
