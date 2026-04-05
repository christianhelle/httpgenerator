use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use httpgenerator_compat::{
    DotnetOracleRunner, RustCliRunner, execute_differential_plan, local_smoke_scenarios,
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

#[test]
fn petstore_parity_matrix_matches_dotnet_oracle() {
    let repo_root = repo_root();
    let artifacts_root = temp_artifacts_root();
    let scenarios = local_smoke_scenarios(&repo_root);
    let oracle_runner = DotnetOracleRunner::from_repo_root(&repo_root);
    let rust_runner = RustCliRunner::from_repo_root(&repo_root);

    for scenario_name in [
        "petstore-v3.0-json-one-request-per-file",
        "petstore-v3.0-json-one-file",
        "petstore-v3.0-json-one-file-per-tag",
        "petstore-v3.0-json-auth-header",
        "petstore-v3.0-json-auth-env",
        "petstore-v3.0-json-skip-headers",
        "petstore-v3.0-json-env-baseurl",
    ] {
        let scenario = scenarios
            .iter()
            .find(|scenario| scenario.name == scenario_name)
            .unwrap_or_else(|| panic!("expected smoke scenario '{scenario_name}'"));
        let plan = scenario.differential_plan(&artifacts_root);
        let result = execute_differential_plan(&plan, &oracle_runner, &rust_runner).unwrap();

        assert!(
            result.is_match(),
            "expected Rust output to match the .NET oracle for {scenario_name}\n{}",
            result.mismatch_report()
        );
    }

    let _ = fs::remove_dir_all(artifacts_root);
}
