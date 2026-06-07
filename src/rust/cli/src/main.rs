mod ui;

use clap::FromArgMatches;
use httpgenerator_cli::{
    ExceptionlessTelemetrySink, NoopTelemetrySink, TelemetryRecorder, TelemetrySinkCollection,
    args::{CliArgs, build_command},
    execute_with_observer,
};
use std::{ffi::OsString, time::Instant};
use ui::CliPresenter;

#[tokio::main]
async fn main() {
    let raw_args = raw_args_with_help();
    let args = parse_args(&raw_args);
    let started_at = Instant::now();
    let mut presenter = CliPresenter::detect();
    presenter.print_header(args.no_logging);

    let sink = create_telemetry_sink(&args);
    let mut telemetry = TelemetryRecorder::from_cli_args(&raw_args, &args, sink);

    let result = execute_with_observer(args.clone(), &mut presenter);

    match result {
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

    let recorder = telemetry.into_sink();
    flush_telemetry(recorder).await;
}

fn create_telemetry_sink(args: &CliArgs) -> TelemetrySinkCollection {
    if args.no_logging {
        NoopTelemetrySink.into()
    } else {
        ExceptionlessTelemetrySink::new().into()
    }
}

async fn flush_telemetry(sink: TelemetrySinkCollection) {
    sink.flush().await;
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
