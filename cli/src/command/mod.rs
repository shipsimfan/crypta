use argparse::Error;
use hash::HashCommand;

mod hash;

mod execute;

/// The requested command to be run
pub enum Command {
    /// Hash input files with a specified hash algorithm
    Hash(HashCommand),
}

impl argparse::Command for Command {
    fn parse(
        source: &mut dyn argparse::ArgumentSource,
        command_list: String,
    ) -> argparse::Result<Option<Self>> {
        let argument = source
            .next()
            .ok_or(Error::missing_positional_value("COMMAND"))?;

        match argument.as_str()? {
            "hash" => HashCommand::parse(source, format!("{}hash ", command_list))
                .map(|command| command.map(Command::Hash)),
            _ => Err(Error::unknown_argument(argument.to_string())),
        }
    }
}
