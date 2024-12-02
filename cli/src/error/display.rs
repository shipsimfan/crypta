use crate::{messages::*, Error};
use i18n::translation::m;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::OpenFileError(path, error) => {
                let path = path.display();
                m!(OpenFileError, path => &path, error).fmt(f)
            }
        }
    }
}
