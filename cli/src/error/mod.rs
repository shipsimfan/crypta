use std::path::PathBuf;

mod display;

/// An error that can occur while running a command
#[derive(Debug)]
pub enum Error {
    /// Unable to open a file
    OpenFileError(PathBuf, std::io::Error),
}

impl std::error::Error for Error {}
