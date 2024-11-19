use super::HashAlgorithm;

impl std::fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            HashAlgorithm::MD5 => "md5",
            HashAlgorithm::SHA1 => "sha1",
            HashAlgorithm::SHA224 => "sha224",
            HashAlgorithm::SHA256 => "sha256",
            HashAlgorithm::SHA384 => "sha384",
            HashAlgorithm::SHA512 => "sha512",
        })
    }
}
