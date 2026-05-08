mod ui;

use clap::FromArgMatches;
use httpgenerator_cli::{
    NoopTelemetrySink, TelemetryRecorder,
    args::{CliArgs, build_command},
    execute_with_observer,
};
use std::{ffi::OsString, time::Instant};
use ui::CliPresenter;

fn main() {
    let raw_args = raw_args_with_help();
    let args = parse_args(&raw_args);
    let mut telemetry = TelemetryRecorder::from_cli_args(&raw_args, &args, NoopTelemetrySink);
    let started_at = Instant::now();
    let mut presenter = CliPresenter::detect();
    presenter.print_header(args.no_logging);

    match execute_with_observer(args.clone(), &mut presenter) {
        Ok(_summary) => {
            telemetry.record_feature_usage(&args);
            presenter.print_success(started_at.elapsed());
        }
        Err(error) => {
            telemetry.record_error(&args, error.telemetry_name(), &error.to_string());
            presenter.print_error(&error);
            std::process::exit(1);
        }
    }
}

fn raw_args_with_help() -> Vec<OsString> {
    let mut raw_args: Vec<OsString> = std::env::args_os().collect();
    if raw_args.len() == 1 {
        raw_args.push(OsString::from("--help"));
    }

    raw_args
}

fn parse_args(raw_args: &[OsString]) -> CliArgs {
    let matches = build_command().get_matches_from(raw_args.iter().cloned());
    CliArgs::from_arg_matches(&matches)
        .expect("clap should only return matches that satisfy CliArgs")
}
