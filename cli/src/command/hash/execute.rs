use super::HashCommand;

impl HashCommand {
    pub fn execute(self) {
        println!("Hash Command:");
        println!(" - Algorithm: {}", self.algorithm);
        println!(" - Count: {}", self.count);
        print!(" - Files: ");
        for file in self.files {
            print!("{}, ", file.display());
        }
        println!();
    }
}
