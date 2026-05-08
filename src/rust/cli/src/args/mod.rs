mod help;
mod types;

use clap::{Command, CommandFactory};

pub use types::{CliArgs, OutputTypeArg};

pub fn build_command() -> Command {
    help::configure(CliArgs::command())
}

#[cfg(test)]
mod tests;
