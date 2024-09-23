/// An error while parsing a [`Hash`](crate::hash::Hash) from a string
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseHashError {
    /// The provided string is too short
    TooShort,

    /// The provided string contains invalid hex characters
    InvalidHex,

    /// The provided string is too long
    TooLong,
}

impl std::error::Error for ParseHashError {}

impl std::fmt::Display for ParseHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseHashError::TooShort => write!(f, "hash is too short"),
            ParseHashError::InvalidHex => {
                write!(f, "hash contains characters which are not hex digits")
            }
            ParseHashError::TooLong => write!(f, "hash is too long"),
        }
    }
}
