use clap::{CommandFactory, Parser};
use httpgenerator_cli::args::CliArgs;

fn main() {
    if std::env::args_os().len() == 1 {
        let mut command = CliArgs::command();
        command
            .print_help()
            .expect("help output should be printable");
        println!();
        return;
    }

    let _args = CliArgs::parse();
    eprintln!("Rust rewrite in progress: CLI execution is not implemented yet.");
    std::process::exit(1);
}
