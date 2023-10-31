use std::marker::PhantomData;

pub(super) trait SHAWord {}

pub(super) struct SHAHasher<Word: SHAWord> {
    phantom: PhantomData<Word>,
}

impl<Word: SHAWord> SHAHasher<Word> {
    pub(super) fn new() -> Self {
        SHAHasher {
            phantom: PhantomData,
        }
    }

    pub(super) fn add_hash(&mut self, source: &mut dyn Iterator<Item = u8>) {
        let mut i = 0;
        for byte in source {
            if i % 1_000 == 0 {
                println!("{} ({})", i, byte as char);
            }

            i += 1;
        }
    }

    pub(super) fn finalize_hash(self) -> [u8; std::mem::size_of::<Word>() * 8] {
        todo!()
    }
}

impl SHAWord for u32 {}

impl SHAWord for u64 {}
