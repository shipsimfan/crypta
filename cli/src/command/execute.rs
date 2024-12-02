use super::Command;
use crate::Error;

impl Command {
    pub fn execute(self) -> Result<(), Error> {
        match self {
            Command::Hash(hash) => hash.execute(),
        }
    }
}
