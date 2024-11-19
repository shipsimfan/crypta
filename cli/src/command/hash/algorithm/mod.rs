mod display;
mod positional;

/// A selected algorithm to hash data with
pub enum HashAlgorithm {
    /// Use [`crypta::hash::MD5`]
    MD5,

    /// Use [`crypta::hash::SHA1`]
    SHA1,

    /// Use [`crypta::hash::SHA224`]
    SHA224,

    /// Use [`crypta::hash::SHA256`]
    SHA256,

    /// Use [`crypta::hash::SHA384`]
    SHA384,

    /// Use [`crypta::hash::SHA512`]
    SHA512,
}
