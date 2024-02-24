use super::{Buffer, HashLength, Word};

pub(in crate::hash::sha_2) struct Hasher<W: Word>
where
    [u8; 16 * std::mem::size_of::<W>()]: Sized,
    [u8; std::mem::size_of::<W::Length>()]: Sized,
    [W; W::ROUNDS]: Sized,
{
    buffer: Buffer<W, 16>,
    index: usize,
    total_length: W::Length,
    hash: [W; 8],
}

impl<W: Word> Hasher<W>
where
    [u8; 16 * std::mem::size_of::<W>()]: Sized,
    [u8; std::mem::size_of::<W::Length>()]: Sized,
    [W; W::ROUNDS]: Sized,
{
    pub(in crate::hash::sha_2) fn new(initial_hash_value: [W; 8]) -> Self {
        Hasher {
            buffer: Buffer::new(),
            index: 0,
            total_length: W::Length::ZERO,
            hash: initial_hash_value,
        }
    }

    /// Adds a byte to the buffer, computing the next block of the hash if the end of the buffer is
    /// reached
    pub(in crate::hash::sha_2) fn add_byte(&mut self, byte: u8) {
        self.buffer[self.index] = byte;
        self.index += 1;
        self.total_length += W::Length::EIGHT;

        if self.index == self.buffer.len() {
            self.calculate_block();
        }
    }

    /// Pads and calculates the hash for the current and last block
    pub(in crate::hash::sha_2) fn calculate_last_block(&mut self) {
        self.buffer[self.index] = 0x80;
        self.index += 1;

        if self.index >= self.buffer.len() - std::mem::size_of::<W::Length>() {
            while self.index < self.buffer.len() {
                self.buffer[self.index] = 0;
                self.index += 1;
            }

            self.calculate_block();
        }

        self.calculate_true_last_block();
    }

    /// Unwraps the hash as currently computed
    pub(in crate::hash::sha_2) fn unwrap_hash(mut self) -> [W; 8] {
        for i in 0..8 {
            self.hash[i] = self.hash[i].swap_endian();
        }

        self.hash
    }

    /// Fills in remaining zeros, the true length, and then calculates the true last block
    fn calculate_true_last_block(&mut self) {
        while self.index < self.buffer.len() - std::mem::size_of::<W::Length>() {
            self.buffer[self.index] = 0;
            self.index += 1;
        }

        for byte in self.total_length.to_be_bytes() {
            self.buffer[self.index] = byte;
            self.index += 1;
        }

        self.calculate_block();
    }

    /// Calculates the current hash block
    fn calculate_block(&mut self) {
        assert_eq!(self.index, self.buffer.len());

        let mut w = [W::ZERO; W::ROUNDS];
        let m = self.buffer.as_words();

        // Prepare the message schedule
        for t in 0..16 {
            w[t] = m[t];
        }
        for t in 16..W::ROUNDS {
            w[t] = (w[t - 2].ssig1())
                .add(w[t - 7])
                .add(w[t - 15].ssig0())
                .add(w[t - 16]);
        }

        // Initialize the working variables
        let mut a = self.hash[0];
        let mut b = self.hash[1];
        let mut c = self.hash[2];
        let mut d = self.hash[3];
        let mut e = self.hash[4];
        let mut f = self.hash[5];
        let mut g = self.hash[6];
        let mut h = self.hash[7];

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
        self.hash[0] = a.add(self.hash[0]);
        self.hash[1] = b.add(self.hash[1]);
        self.hash[2] = c.add(self.hash[2]);
        self.hash[3] = d.add(self.hash[3]);
        self.hash[4] = e.add(self.hash[4]);
        self.hash[5] = f.add(self.hash[5]);
        self.hash[6] = g.add(self.hash[6]);
        self.hash[7] = h.add(self.hash[7]);

        self.index = 0;
    }
}
