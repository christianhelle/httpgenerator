use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use httpgenerator_core::OutputType;
use httpgenerator_compat::{
    CompatibilityScenario, DotnetOracleRunner, RustCliRunner, execute_differential_plan,
    local_smoke_scenarios,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn temp_artifacts_root() -> PathBuf {
    std::env::temp_dir().join(format!(
        "httpgenerator-differential-tests-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn fixture_name(scenario: &CompatibilityScenario) -> &str {
    scenario
        .input_path
        .file_stem()
        .and_then(|name| name.to_str())
        .expect("scenario input should have a file stem")
}

fn version_name(scenario: &CompatibilityScenario) -> &str {
    scenario
        .input_path
        .parent()
        .and_then(|path| path.file_name())
        .and_then(|name| name.to_str())
        .expect("scenario input should live under a version directory")
}

fn is_default_matrix_scenario(scenario: &CompatibilityScenario) -> bool {
    scenario.output_type == OutputType::OneFile
        && scenario.no_logging
        && scenario.authorization_header.is_none()
        && !scenario.authorization_header_from_environment_variable
        && scenario.authorization_header_variable_name == "authorization"
        && scenario.content_type == "application/json"
        && scenario.base_url.as_deref() == Some("https://api.example.io/")
        && scenario.generate_intellij_tests
        && scenario.custom_headers == ["X-Custom-Header: 1234"]
        && !scenario.skip_headers
}

fn is_representative_output_mode_scenario(scenario: &CompatibilityScenario) -> bool {
    fixture_name(scenario) == "petstore"
        && matches!(version_name(scenario), "v2.0" | "v3.0")
        && matches!(
            scenario.output_type,
            OutputType::OneRequestPerFile | OutputType::OneFilePerTag
        )
        && scenario.no_logging
        && scenario.authorization_header.is_none()
        && !scenario.authorization_header_from_environment_variable
        && scenario.content_type == "application/json"
        && scenario.base_url.as_deref() == Some("https://api.example.io/")
        && scenario.generate_intellij_tests
        && scenario.custom_headers == ["X-Custom-Header: 1234"]
        && !scenario.skip_headers
}

fn is_petstore_option_variation(scenario: &CompatibilityScenario) -> bool {
    fixture_name(scenario) == "petstore"
        && matches!(version_name(scenario), "v2.0" | "v3.0")
        && scenario.output_type == OutputType::OneFile
        && (scenario.authorization_header.is_some()
            || scenario.authorization_header_from_environment_variable
            || scenario.skip_headers
            || scenario.content_type != "application/json"
            || scenario.base_url.as_deref() == Some("{{MY_BASE_URL}}"))
}

#[test]
fn parity_matrix_matches_dotnet_oracle() {
    let repo_root = repo_root();
    let artifacts_root = temp_artifacts_root();
    let scenarios = local_smoke_scenarios(&repo_root);
    let oracle_runner = DotnetOracleRunner::from_repo_root(&repo_root);
    let rust_runner = RustCliRunner::from_repo_root(&repo_root);

    let scenarios_to_run = scenarios
        .iter()
        .filter(|scenario| {
            is_default_matrix_scenario(scenario)
                || is_representative_output_mode_scenario(scenario)
                || is_petstore_option_variation(scenario)
        })
        .collect::<Vec<_>>();

    assert!(
        scenarios_to_run
            .iter()
            .any(|scenario| scenario.name == "non-oauth-scopes-v3.1-json-one-file")
    );
    assert!(
        scenarios_to_run
            .iter()
            .any(|scenario| scenario.name == "callback-example-v3.0-yaml-one-file")
    );
    assert!(
        scenarios_to_run
            .iter()
            .any(|scenario| scenario.name == "petstore-v2.0-yaml-one-file-per-tag")
    );

    for scenario in scenarios_to_run {
        let plan = scenario.differential_plan(&artifacts_root);
        let result = execute_differential_plan(&plan, &oracle_runner, &rust_runner).unwrap();

        assert!(
            result.is_match(),
            "expected Rust output to match the .NET oracle for {}\n{}",
            scenario.name,
            result.mismatch_report()
        );
    }

    let _ = fs::remove_dir_all(artifacts_root);
}
