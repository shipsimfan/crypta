use super::Command;

impl Command {
    pub fn execute(self) {
        match self {
            Command::Hash(hash) => hash.execute(),
        }
    }
}
