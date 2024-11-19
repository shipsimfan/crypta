use algorithm::HashAlgorithm;
use argparse::{
    ArgumentSource, Command, Error, Flag, FlagInfo, Positional, PositionalInfo, PositionalResult,
    Result,
};
use std::{num::NonZeroUsize, path::PathBuf};

mod algorithm;

mod execute;

/// Hash files using a given algorithm
pub struct HashCommand {
    /// The algorithm to use when hashing
    algorithm: HashAlgorithm,

    /// The set of files to hash
    files: Vec<PathBuf>,

    /// The number of times to read the input files into the hash
    count: NonZeroUsize,
}

impl Command for HashCommand {
    fn parse(source: &mut dyn ArgumentSource) -> Result<Self> {
        const ALGORITHM_INFO: &PositionalInfo<HashAlgorithm> = &PositionalInfo {
            value: "ALGORITHM",
            min_count: 0,
            max_count: 0,
            default: None,
        };
        const FILES_INFO: &PositionalInfo<Vec<PathBuf>> = &PositionalInfo {
            value: "FILE",
            min_count: 1,
            max_count: 0,
            default: None,
        };

        const COUNT_INFO: &FlagInfo<NonZeroUsize> = &FlagInfo {
            long_name: Some("--count"),
            short_name: Some("-c"),
            value: Some("COUNT"),
            min_count: 0,
            max_count: 0,
            default: Some(|| unsafe { NonZeroUsize::new_unchecked(1) }),
        };

        let mut algorithm = None;
        let mut files = None;

        let mut current_positional = 0;

        let mut count = None;

        while let Some(argument) = source.next() {
            if let Ok(argument) = argument.as_str() {
                if argument.starts_with("--") {
                    match &argument[2..] {
                        "count" => count = Some(Flag::parse(source, COUNT_INFO, true)?),
                        _ => return Err(Error::unknown_argument(argument.to_string())),
                    }

                    continue;
                } else if argument.starts_with('-') && argument.len() > 1 {
                    let mut chars = argument.chars();
                    chars.next();
                    for c in chars {
                        match c {
                            'c' => count = Some(Flag::parse(source, COUNT_INFO, false)?),
                            _ => return Err(argparse::Error::unknown_argument(format!("-{}", c))),
                        }
                    }

                    continue;
                }
            }

            let result = match current_positional {
                0 => Positional::parse(&mut algorithm, argument, ALGORITHM_INFO),
                1 => Positional::parse(&mut files, argument, FILES_INFO),
                _ => return Err(Error::unknown_argument(argument.to_string())),
            };

            match result {
                PositionalResult::Continue => {}
                PositionalResult::Next => current_positional += 1,
                PositionalResult::Sub => todo!(),
                PositionalResult::Error(error) => return Err(error),
            };
        }

        let algorithm = Positional::unwrap(algorithm, ALGORITHM_INFO)?;
        let files = Positional::unwrap(files, FILES_INFO)?;

        let count = Flag::unwrap(count, COUNT_INFO)?;

        Ok(HashCommand {
            algorithm,
            files,
            count,
        })
    }
}
