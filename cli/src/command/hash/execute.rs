use super::{HashAlgorithm, HashCommand};
use crate::Error;
use crypta::hash::HashFunction;
use std::num::NonZeroUsize;

fn hash_content<T: HashFunction>(content: &[u8], count: NonZeroUsize)
where
    [u8; T::BYTE_SIZE]: Sized,
{
    let mut hasher = T::new();
    for _ in 0..count.get() {
        hasher.push(content);
    }

    println!("{}", hasher.digest());
}

impl HashCommand {
    pub fn execute(self) -> Result<(), Error> {
        // Load files
        let mut contents = Vec::new();
        for file in self.files {
            let content =
                std::fs::read(&file).map_err(|error| Error::OpenFileError(file, error))?;
            contents.extend_from_slice(&content);
        }

        // Hash files
        match self.algorithm {
            HashAlgorithm::MD5 => hash_content::<crypta::hash::MD5>(&contents, self.count),
            HashAlgorithm::SHA1 => hash_content::<crypta::hash::SHA1>(&contents, self.count),
            HashAlgorithm::SHA224 => hash_content::<crypta::hash::SHA224>(&contents, self.count),
            HashAlgorithm::SHA256 => hash_content::<crypta::hash::SHA256>(&contents, self.count),
            HashAlgorithm::SHA384 => hash_content::<crypta::hash::SHA384>(&contents, self.count),
            HashAlgorithm::SHA512 => hash_content::<crypta::hash::SHA512>(&contents, self.count),
        }

        Ok(())
    }
}
