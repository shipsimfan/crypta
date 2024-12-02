#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use argparse::Command;
use error::Error;
use i18n::translation::m;
use messages::ErrorPrefix;

mod command;
mod error;

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", m!(ErrorPrefix, error => &error));
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let command = command::Command::parse_env()?;

    command.execute()?;

    Ok(())
}

mod messages {
    i18n::include_fluent!("cli/fluent");
}
