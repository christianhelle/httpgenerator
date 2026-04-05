use clap::FromArgMatches;
use httpgenerator_cli::args::{CliArgs, build_command};
use std::ffi::OsString;

fn main() {
    let mut raw_args: Vec<OsString> = std::env::args_os().collect();
    if raw_args.len() == 1 {
        raw_args.push(OsString::from("--help"));
    }

    let matches = build_command().get_matches_from(raw_args);
    let _args = CliArgs::from_arg_matches(&matches)
        .expect("clap should only return matches that satisfy CliArgs");
    eprintln!("Rust rewrite in progress: CLI execution is not implemented yet.");
    std::process::exit(1);
}
