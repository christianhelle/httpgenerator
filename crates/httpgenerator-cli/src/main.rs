use clap::FromArgMatches;
use httpgenerator_cli::{
    args::{CliArgs, build_command},
    execute,
};
use std::ffi::OsString;

fn main() {
    let mut raw_args: Vec<OsString> = std::env::args_os().collect();
    if raw_args.len() == 1 {
        raw_args.push(OsString::from("--help"));
    }

    let matches = build_command().get_matches_from(raw_args);
    let args = CliArgs::from_arg_matches(&matches)
        .expect("clap should only return matches that satisfy CliArgs");

    match execute(args) {
        Ok(summary) => {
            println!(
                "Generated {} file(s) in {}",
                summary.files.len(),
                summary.output_folder.display()
            );
            for path in summary.files {
                println!("{}", path.display());
            }
        }
        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    }
}
