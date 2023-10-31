use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Shr};

pub(super) trait SHAWord:
    Sized
    + Clone
    + Copy
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
{
    const ZERO: Self;

    fn wrapping_add(self, rhs: Self) -> Self;

    fn wrapping_shr(self, rhs: u32) -> Self;

    fn rotr(self, rhs: u32) -> Self;
    fn rotl(self, rhs: u32) -> Self;
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub(super) struct Word<W: SHAWord>(pub W);

pub(super) struct SHAHasher<W: SHAWord, const MESSAGE_SCHEDULE_LEN: usize> {
    block: [Word<W>; 16],
    block_len: usize,

    hash_value: [Word<W>; 8],
}

fn as_bytes_mut<W: SHAWord>(block: &mut [Word<W>]) -> &mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut(
            block.as_mut_ptr().cast(),
            std::mem::size_of::<W>() * block.len(),
        )
    }
}

fn ch<W: SHAWord>(x: Word<W>, y: Word<W>, z: Word<W>) -> Word<W> {
    (x & y) ^ (!x & z)
}

fn maj<W: SHAWord>(x: Word<W>, y: Word<W>, z: Word<W>) -> Word<W> {
    (x & y) ^ (x & z) ^ (y & z)
}

impl<W: SHAWord, const MESSAGE_SCHEDULE_LEN: usize> SHAHasher<W, MESSAGE_SCHEDULE_LEN> {
    pub(super) fn new(initial_hash_value: [Word<W>; 8]) -> Self {
        SHAHasher {
            block: [Word(W::ZERO); 16],
            block_len: 0,

            hash_value: initial_hash_value,
        }
    }

    pub(super) fn add_hash(&mut self, source: &mut dyn Iterator<Item = u8>) {
        let mut block = as_bytes_mut(&mut self.block);

        for byte in source {
            if self.block_len == block.len() {
                self.hash_current_block();

                block = as_bytes_mut(&mut self.block);
                self.block_len = 0;
            }

            block[self.block_len] = byte;
            self.block_len += 1;
        }
    }

    pub(super) fn finalize_hash(self) -> [u8; std::mem::size_of::<W>() * 8] {
        todo!()
    }

    fn hash_current_block(&mut self) {
        // Prepare the message schedule
        let mut message_schedule = [Word(W::ZERO); MESSAGE_SCHEDULE_LEN];

        for i in 0..15 {
            message_schedule[i] = self.block[i];
        }

        for i in 16..MESSAGE_SCHEDULE_LEN {
            message_schedule[i] =
                todo!() + message_schedule[i - 7] + todo!() + message_schedule[i - 16];
        }

        // Initialize the working variables
        let mut a = self.hash_value[0];
        let mut b = self.hash_value[1];
        let mut c = self.hash_value[2];
        let mut d = self.hash_value[3];
        let mut e = self.hash_value[4];
        let mut f = self.hash_value[5];
        let mut g = self.hash_value[6];
        let mut h = self.hash_value[7];

        // Perform the main hash computation
        for i in 0..MESSAGE_SCHEDULE_LEN {
            let t1 = h + todo!() + ch(e, f, g) + todo!() + message_schedule[i];
            let t2 = todo!() + maj(a, b, c);

            h = g;
            g = f;
            f = e;
            e = d + t1;
            d = c;
            c = b;
            b = a;
            a = t1 + t2;
        }

        // Computer the intermediate hash value
        self.hash_value[0] = a + self.hash_value[0];
        self.hash_value[1] = b + self.hash_value[1];
        self.hash_value[2] = c + self.hash_value[2];
        self.hash_value[3] = d + self.hash_value[3];
        self.hash_value[4] = e + self.hash_value[4];
        self.hash_value[5] = f + self.hash_value[5];
        self.hash_value[6] = g + self.hash_value[6];
        self.hash_value[7] = h + self.hash_value[7];
    }
}

impl SHAWord for u32 {
    const ZERO: Self = 0;

    fn wrapping_add(self, rhs: Self) -> Self {
        u32::wrapping_add(self, rhs)
    }

    fn wrapping_shr(self, rhs: u32) -> Self {
        u32::wrapping_shr(self, rhs)
    }

    fn rotr(self, rhs: u32) -> Self {
        u32::rotate_right(self, rhs)
    }

    fn rotl(self, rhs: u32) -> Self {
        u32::rotate_left(self, rhs)
    }
}

impl SHAWord for u64 {
    const ZERO: Self = 0;

    fn wrapping_add(self, rhs: Self) -> Self {
        u64::wrapping_add(self, rhs)
    }

    fn wrapping_shr(self, rhs: u32) -> Self {
        u64::wrapping_shr(self, rhs)
    }

    fn rotr(self, rhs: u32) -> Self {
        u64::rotate_right(self, rhs)
    }

    fn rotl(self, rhs: u32) -> Self {
        u64::rotate_left(self, rhs)
    }
}

impl<W: SHAWord> BitAnd for Word<W> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Word(self.0.bitand(rhs.0))
    }
}

impl<W: SHAWord> BitOr for Word<W> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Word(self.0.bitor(rhs.0))
    }
}

impl<W: SHAWord> BitXor for Word<W> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Word(self.0.bitxor(rhs.0))
    }
}

impl<W: SHAWord> Not for Word<W> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Word(self.0.not())
    }
}

impl<W: SHAWord> Add for Word<W> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Word(self.0.wrapping_add(rhs.0))
    }
}

impl<W: SHAWord> Shr<u32> for Word<W> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Word(self.0.wrapping_shr(rhs))
    }
}
