use algorithm::HashAlgorithm;
use argparse::Command;
use std::{num::NonZeroUsize, path::PathBuf};

mod algorithm;

mod execute;

/// Hash files using a given algorithm
#[derive(Command)]
#[command(version)]
pub struct HashCommand {
    /// The algorithm to use when hashing
    algorithm: HashAlgorithm,

    /// The set of files to hash
    #[arg(min = 1)]
    files: Vec<PathBuf>,

    /// The number of times to read the input files into the hash
    #[flag(short_name, default = DEFAULT_COUNT)]
    count: NonZeroUsize,
}

const DEFAULT_COUNT: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };
