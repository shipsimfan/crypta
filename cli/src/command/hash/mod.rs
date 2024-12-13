use crate::messages::*;
use algorithm::HashAlgorithm;
use argparse::Command;
use i18n::translation::m;
use std::{num::NonZeroUsize, path::PathBuf};

mod algorithm;

mod execute;

/// Hash files using a given algorithm
#[derive(Command)]
#[command(version, help, description = m!(ProgramDescription))]
pub struct HashCommand {
    /// The algorithm to use when hashing
    #[arg(description = m!(HashAlgorithmDescription))]
    algorithm: HashAlgorithm,

    /// The set of files to hash
    #[arg(min = 1, description = m!(HashFilesDescription))]
    files: Vec<PathBuf>,

    /// The number of times to read the input files into the hash
    #[flag(short_name, default = DEFAULT_COUNT, description = m!(HashCountDescription))]
    count: NonZeroUsize,
}

const DEFAULT_COUNT: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };
