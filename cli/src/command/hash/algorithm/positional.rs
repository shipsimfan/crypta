use super::HashAlgorithm;
use argparse::{Argument, Error, Positional, PositionalInfo, PositionalResult};

#[derive(Debug)]
struct UnknownHashAlgorithm(String);

impl Positional for HashAlgorithm {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        *this = Some(
            match match argument.as_str() {
                Ok(argument) => argument,
                Err(error) => return PositionalResult::Error(error),
            } {
                "md5" => HashAlgorithm::MD5,
                "sha1" => HashAlgorithm::SHA1,
                "sha224" => HashAlgorithm::SHA224,
                "sha256" => HashAlgorithm::SHA256,
                "sha384" => HashAlgorithm::SHA384,
                "sha512" => HashAlgorithm::SHA512,
                _ => {
                    return PositionalResult::Error(Error::invalid_positional_value(
                        info.value,
                        UnknownHashAlgorithm(argument.to_string()),
                    ));
                }
            },
        );

        PositionalResult::Next
    }
}

impl std::error::Error for UnknownHashAlgorithm {}

impl std::fmt::Display for UnknownHashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown hash algorithm \"{}\"", self.0)
    }
}
