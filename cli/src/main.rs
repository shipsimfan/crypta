use argparse::Command;

mod command;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let command = command::Command::parse_env()?;

    command.execute();

    Ok(())
}
