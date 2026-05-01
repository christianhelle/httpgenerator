use std::path::{Path, PathBuf};

use httpgenerator_core::OutputType;

mod runner;

pub use runner::{
    CommandOutput, CommandSpec, DifferentialRunError, DifferentialRunPlan, DifferentialRunResult,
    DirectoryComparison, DirectorySnapshot, DotnetOracleLaunch, DotnetOracleRunner,
    OutputDifference, OutputFileSnapshot, RustCliLaunch, RustCliRunner, ScenarioOutputLayout,
    execute_differential_plan, scenario_directory_name,
};

const SMOKE_FIXTURE_NAMES: &[&str] = &[
    "petstore",
    "petstore-expanded",
    "petstore-minimal",
    "petstore-simple",
    "petstore-with-external-docs",
    "api-with-examples",
    "callback-example",
    "link-example",
    "uber",
    "uspto",
    "hubspot-events",
    "hubspot-webhooks",
    "non-oauth-scopes",
    "webhook-example",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityScenario {
    pub name: String,
    pub input_path: PathBuf,
    pub output_type: OutputType,
    pub no_logging: bool,
    pub skip_validation: bool,
    pub authorization_header: Option<String>,
    pub authorization_header_from_environment_variable: bool,
    pub authorization_header_variable_name: String,
    pub content_type: String,
    pub base_url: Option<String>,
    pub generate_intellij_tests: bool,
    pub custom_headers: Vec<String>,
    pub skip_headers: bool,
}

impl CompatibilityScenario {
    pub fn cli_arguments(&self, output_dir: &Path) -> Vec<String> {
        let mut arguments = vec![
            self.input_path.to_string_lossy().into_owned(),
            "--output".to_string(),
            output_dir.to_string_lossy().into_owned(),
        ];

        if self.output_type != OutputType::OneRequestPerFile {
            arguments.push("--output-type".to_string());
            arguments.push(output_type_name(self.output_type).to_string());
        }

        if self.no_logging {
            arguments.push("--no-logging".to_string());
        }

        if self.skip_validation {
            arguments.push("--skip-validation".to_string());
        }

        if let Some(authorization_header) = &self.authorization_header {
            arguments.push("--authorization-header".to_string());
            arguments.push(authorization_header.clone());
        }

        if self.authorization_header_from_environment_variable {
            arguments.push("--load-authorization-header-from-environment".to_string());
            arguments.push("--authorization-header-variable-name".to_string());
            arguments.push(self.authorization_header_variable_name.clone());
        }

        if self.content_type != "application/json" {
            arguments.push("--content-type".to_string());
            arguments.push(self.content_type.clone());
        }

        if let Some(base_url) = &self.base_url {
            arguments.push("--base-url".to_string());
            arguments.push(base_url.clone());
        }

        if self.generate_intellij_tests {
            arguments.push("--generate-intellij-tests".to_string());
        }

        for custom_header in &self.custom_headers {
            arguments.push("--custom-header".to_string());
            arguments.push(custom_header.clone());
        }

        if self.skip_headers {
            arguments.push("--skip-headers".to_string());
        }

        arguments
    }

    pub fn dotnet_arguments(&self, output_dir: &Path) -> Vec<String> {
        self.cli_arguments(output_dir)
    }

    pub fn differential_plan(&self, artifacts_root: impl AsRef<Path>) -> DifferentialRunPlan {
        DifferentialRunPlan::for_scenario(artifacts_root, self.clone())
    }
}

pub fn local_smoke_scenarios(repo_root: &Path) -> Vec<CompatibilityScenario> {
    let mut scenarios = Vec::new();

    for version in ["v2.0", "v3.0", "v3.1"] {
        for format in ["json", "yaml"] {
            for fixture_name in SMOKE_FIXTURE_NAMES {
                let input_path = repo_root
                    .join("test")
                    .join("OpenAPI")
                    .join(version)
                    .join(format!("{fixture_name}.{format}"));

                if !input_path.is_file() {
                    continue;
                }

                let skip_validation = version == "v3.1";
                let mut base = CompatibilityScenario {
                    name: format!(
                        "{fixture_name}-{version}-{format}-{}",
                        output_type_slug(OutputType::OneRequestPerFile)
                    ),
                    input_path: input_path.clone(),
                    output_type: OutputType::OneRequestPerFile,
                    no_logging: true,
                    skip_validation,
                    authorization_header: None,
                    authorization_header_from_environment_variable: false,
                    authorization_header_variable_name: "authorization".to_string(),
                    content_type: "application/json".to_string(),
                    base_url: Some("https://api.example.io/".to_string()),
                    generate_intellij_tests: true,
                    custom_headers: vec!["X-Custom-Header: 1234".to_string()],
                    skip_headers: false,
                };

                scenarios.push(base.clone());

                base.output_type = OutputType::OneFile;
                base.name = format!(
                    "{fixture_name}-{version}-{format}-{}",
                    output_type_slug(base.output_type)
                );
                scenarios.push(base.clone());

                base.output_type = OutputType::OneFilePerTag;
                base.name = format!(
                    "{fixture_name}-{version}-{format}-{}",
                    output_type_slug(base.output_type)
                );
                scenarios.push(base.clone());

                if fixture_name == &"petstore" && !skip_validation {
                    scenarios.push(CompatibilityScenario {
                        name: format!("{fixture_name}-{version}-{format}-auth-header"),
                        input_path: input_path.clone(),
                        output_type: OutputType::OneFile,
                        no_logging: true,
                        skip_validation: false,
                        authorization_header: Some("Bearer test-token-123".to_string()),
                        authorization_header_from_environment_variable: false,
                        authorization_header_variable_name: "authorization".to_string(),
                        content_type: "application/json".to_string(),
                        base_url: None,
                        generate_intellij_tests: false,
                        custom_headers: Vec::new(),
                        skip_headers: false,
                    });

                    scenarios.push(CompatibilityScenario {
                        name: format!("{fixture_name}-{version}-{format}-auth-env"),
                        input_path: input_path.clone(),
                        output_type: OutputType::OneFile,
                        no_logging: true,
                        skip_validation: false,
                        authorization_header: None,
                        authorization_header_from_environment_variable: true,
                        authorization_header_variable_name: "my_token".to_string(),
                        content_type: "application/json".to_string(),
                        base_url: None,
                        generate_intellij_tests: false,
                        custom_headers: Vec::new(),
                        skip_headers: false,
                    });

                    scenarios.push(CompatibilityScenario {
                        name: format!("{fixture_name}-{version}-{format}-skip-headers"),
                        input_path: input_path.clone(),
                        output_type: OutputType::OneFile,
                        no_logging: true,
                        skip_validation: false,
                        authorization_header: None,
                        authorization_header_from_environment_variable: false,
                        authorization_header_variable_name: "authorization".to_string(),
                        content_type: "application/json".to_string(),
                        base_url: None,
                        generate_intellij_tests: false,
                        custom_headers: Vec::new(),
                        skip_headers: true,
                    });

                    scenarios.push(CompatibilityScenario {
                        name: format!("{fixture_name}-{version}-{format}-xml"),
                        input_path: input_path.clone(),
                        output_type: OutputType::OneFile,
                        no_logging: true,
                        skip_validation: false,
                        authorization_header: None,
                        authorization_header_from_environment_variable: false,
                        authorization_header_variable_name: "authorization".to_string(),
                        content_type: "application/xml".to_string(),
                        base_url: None,
                        generate_intellij_tests: false,
                        custom_headers: Vec::new(),
                        skip_headers: false,
                    });

                    scenarios.push(CompatibilityScenario {
                        name: format!("{fixture_name}-{version}-{format}-env-baseurl"),
                        input_path: input_path.clone(),
                        output_type: OutputType::OneFile,
                        no_logging: true,
                        skip_validation: false,
                        authorization_header: None,
                        authorization_header_from_environment_variable: false,
                        authorization_header_variable_name: "authorization".to_string(),
                        content_type: "application/json".to_string(),
                        base_url: Some("{{MY_BASE_URL}}".to_string()),
                        generate_intellij_tests: false,
                        custom_headers: Vec::new(),
                        skip_headers: false,
                    });
                }
            }
        }
    }

    scenarios
}

fn output_type_name(output_type: OutputType) -> &'static str {
    match output_type {
        OutputType::OneRequestPerFile => "OneRequestPerFile",
        OutputType::OneFile => "OneFile",
        OutputType::OneFilePerTag => "OneFilePerTag",
    }
}

fn output_type_slug(output_type: OutputType) -> &'static str {
    match output_type {
        OutputType::OneRequestPerFile => "one-request-per-file",
        OutputType::OneFile => "one-file",
        OutputType::OneFilePerTag => "one-file-per-tag",
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use httpgenerator_core::OutputType;

    use super::local_smoke_scenarios;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path| path.parent())
            .expect("workspace root should exist")
            .to_path_buf()
    }

    #[test]
    fn includes_v31_scenarios_with_skip_validation() {
        let scenarios = local_smoke_scenarios(&repo_root());

        let scenario = scenarios
            .iter()
            .find(|scenario| scenario.name == "webhook-example-v3.1-json-one-request-per-file")
            .expect("expected webhook v3.1 json scenario");

        assert!(scenario.skip_validation);
        assert_eq!(scenario.output_type, OutputType::OneRequestPerFile);
        assert_eq!(scenario.custom_headers, vec!["X-Custom-Header: 1234"]);
        assert_eq!(
            scenario.base_url.as_deref(),
            Some("https://api.example.io/")
        );
        assert!(
            scenarios
                .iter()
                .any(|scenario| scenario.name == "non-oauth-scopes-v3.1-json-one-file")
        );
    }

    #[test]
    fn includes_petstore_variations_for_non_v31_specs() {
        let scenarios = local_smoke_scenarios(&repo_root());

        assert!(
            scenarios
                .iter()
                .any(|scenario| scenario.name == "petstore-v3.0-json-auth-header")
        );
        assert!(
            scenarios
                .iter()
                .any(|scenario| scenario.name == "petstore-v3.0-json-auth-env")
        );
        assert!(
            scenarios
                .iter()
                .any(|scenario| scenario.name == "petstore-v3.0-json-skip-headers")
        );
        assert!(
            scenarios
                .iter()
                .any(|scenario| scenario.name == "petstore-v3.0-json-xml")
        );
        assert!(
            scenarios
                .iter()
                .any(|scenario| scenario.name == "petstore-v3.0-json-env-baseurl")
        );
    }

    #[test]
    fn default_output_type_omits_explicit_output_flag() {
        let scenarios = local_smoke_scenarios(&repo_root());
        let scenario = scenarios
            .iter()
            .find(|scenario| scenario.name == "petstore-v2.0-json-one-request-per-file")
            .expect("expected petstore v2.0 json scenario");

        let arguments = scenario.dotnet_arguments(PathBuf::from("Generated").as_path());

        assert!(
            arguments
                .windows(2)
                .all(|window| window != ["--output-type", "OneRequestPerFile"])
        );
        assert!(arguments.contains(&"--no-logging".to_string()));
        assert!(arguments.contains(&"--generate-intellij-tests".to_string()));
    }
}
