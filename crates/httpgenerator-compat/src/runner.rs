use std::path::{Path, PathBuf};

use crate::CompatibilityScenario;

const DEFAULT_RUST_CLI_PACKAGE: &str = "httpgenerator-cli";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    pub working_directory: PathBuf,
    pub program: String,
    pub args: Vec<String>,
}

impl CommandSpec {
    pub fn new(
        working_directory: impl Into<PathBuf>,
        program: impl Into<String>,
        args: Vec<String>,
    ) -> Self {
        Self {
            working_directory: working_directory.into(),
            program: program.into(),
            args,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioOutputLayout {
    pub scenario_root: PathBuf,
    pub oracle_output_dir: PathBuf,
    pub rust_output_dir: PathBuf,
}

impl ScenarioOutputLayout {
    pub fn for_scenario(artifacts_root: impl AsRef<Path>, scenario_name: &str) -> Self {
        let scenario_root = artifacts_root
            .as_ref()
            .join(scenario_directory_name(scenario_name));

        Self {
            scenario_root: scenario_root.clone(),
            oracle_output_dir: scenario_root.join("oracle"),
            rust_output_dir: scenario_root.join("rust"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DifferentialRunPlan {
    pub scenario: CompatibilityScenario,
    pub layout: ScenarioOutputLayout,
}

impl DifferentialRunPlan {
    pub fn for_scenario(artifacts_root: impl AsRef<Path>, scenario: CompatibilityScenario) -> Self {
        let layout = ScenarioOutputLayout::for_scenario(artifacts_root, &scenario.name);

        Self { scenario, layout }
    }

    pub fn oracle_command(&self, runner: &DotnetOracleRunner) -> CommandSpec {
        runner.command(&self.scenario, &self.layout.oracle_output_dir)
    }

    pub fn rust_command(&self, runner: &RustCliRunner) -> CommandSpec {
        runner.command(&self.scenario, &self.layout.rust_output_dir)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DotnetOracleRunner {
    pub working_directory: PathBuf,
    pub launch: DotnetOracleLaunch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DotnetOracleLaunch {
    Project { project_path: PathBuf },
    Assembly { assembly_path: PathBuf },
}

impl DotnetOracleRunner {
    pub fn from_repo_root(repo_root: impl AsRef<Path>) -> Self {
        let repo_root = repo_root.as_ref();

        Self::project(
            repo_root.to_path_buf(),
            repo_root
                .join("src")
                .join("HttpGenerator")
                .join("HttpGenerator.csproj"),
        )
    }

    pub fn project(
        working_directory: impl Into<PathBuf>,
        project_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            working_directory: working_directory.into(),
            launch: DotnetOracleLaunch::Project {
                project_path: project_path.into(),
            },
        }
    }

    pub fn assembly(
        working_directory: impl Into<PathBuf>,
        assembly_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            working_directory: working_directory.into(),
            launch: DotnetOracleLaunch::Assembly {
                assembly_path: assembly_path.into(),
            },
        }
    }

    pub fn command(&self, scenario: &CompatibilityScenario, output_dir: &Path) -> CommandSpec {
        let scenario_args = scenario.cli_arguments(output_dir);
        let args = match &self.launch {
            DotnetOracleLaunch::Project { project_path } => {
                let mut args = vec![
                    "run".to_string(),
                    "--project".to_string(),
                    project_path.to_string_lossy().into_owned(),
                    "--".to_string(),
                ];
                args.extend(scenario_args);
                args
            }
            DotnetOracleLaunch::Assembly { assembly_path } => {
                let mut args = vec![assembly_path.to_string_lossy().into_owned()];
                args.extend(scenario_args);
                args
            }
        };

        CommandSpec::new(self.working_directory.clone(), "dotnet", args)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustCliRunner {
    pub working_directory: PathBuf,
    pub launch: RustCliLaunch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustCliLaunch {
    CargoPackage { package_name: String },
    Binary { executable_path: PathBuf },
}

impl RustCliRunner {
    pub fn from_repo_root(repo_root: impl AsRef<Path>) -> Self {
        Self::cargo_package(repo_root.as_ref().to_path_buf(), DEFAULT_RUST_CLI_PACKAGE)
    }

    pub fn cargo_package(
        working_directory: impl Into<PathBuf>,
        package_name: impl Into<String>,
    ) -> Self {
        Self {
            working_directory: working_directory.into(),
            launch: RustCliLaunch::CargoPackage {
                package_name: package_name.into(),
            },
        }
    }

    pub fn binary(
        working_directory: impl Into<PathBuf>,
        executable_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            working_directory: working_directory.into(),
            launch: RustCliLaunch::Binary {
                executable_path: executable_path.into(),
            },
        }
    }

    pub fn command(&self, scenario: &CompatibilityScenario, output_dir: &Path) -> CommandSpec {
        let scenario_args = scenario.cli_arguments(output_dir);

        match &self.launch {
            RustCliLaunch::CargoPackage { package_name } => {
                let mut args = vec![
                    "run".to_string(),
                    "-p".to_string(),
                    package_name.clone(),
                    "--".to_string(),
                ];
                args.extend(scenario_args);

                CommandSpec::new(self.working_directory.clone(), "cargo", args)
            }
            RustCliLaunch::Binary { executable_path } => CommandSpec::new(
                self.working_directory.clone(),
                executable_path.to_string_lossy().into_owned(),
                scenario_args,
            ),
        }
    }
}

pub fn scenario_directory_name(name: &str) -> String {
    let mut sanitized = String::with_capacity(name.len());
    let mut previous_was_separator = false;

    for character in name.chars() {
        let character = character.to_ascii_lowercase();
        if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
            sanitized.push(character);
            previous_was_separator = false;
        } else if !previous_was_separator && !sanitized.is_empty() {
            sanitized.push('-');
            previous_was_separator = true;
        }
    }

    if sanitized.is_empty() {
        "scenario".to_string()
    } else {
        sanitized.trim_matches('-').to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use httpgenerator_core::OutputType;

    use super::{
        DifferentialRunPlan, DotnetOracleRunner, RustCliRunner, ScenarioOutputLayout,
        scenario_directory_name,
    };
    use crate::CompatibilityScenario;

    fn example_scenario() -> CompatibilityScenario {
        CompatibilityScenario {
            name: "petstore-v3.0-json-one-request-per-file".to_string(),
            input_path: PathBuf::from("test")
                .join("OpenAPI")
                .join("v3.0")
                .join("petstore.json"),
            output_type: OutputType::OneRequestPerFile,
            no_logging: true,
            skip_validation: false,
            authorization_header: None,
            authorization_header_from_environment_variable: false,
            authorization_header_variable_name: "authorization".to_string(),
            content_type: "application/json".to_string(),
            base_url: Some("https://api.example.io/".to_string()),
            generate_intellij_tests: true,
            custom_headers: vec!["X-Custom-Header: 1234".to_string()],
            skip_headers: false,
        }
    }

    #[test]
    fn sanitizes_scenario_directory_names() {
        assert_eq!(
            scenario_directory_name(" Petstore v3.0/json? "),
            "petstore-v3.0-json"
        );
        assert_eq!(scenario_directory_name("!!!"), "scenario");
    }

    #[test]
    fn plans_output_layout_for_each_scenario() {
        let artifacts_root = PathBuf::from("target").join("compat");
        let layout = ScenarioOutputLayout::for_scenario(
            &artifacts_root,
            "petstore-v3.0-json-one-request-per-file",
        );

        let scenario_root = artifacts_root.join("petstore-v3.0-json-one-request-per-file");
        assert_eq!(layout.scenario_root, scenario_root);
        assert_eq!(
            layout.oracle_output_dir,
            layout.scenario_root.join("oracle")
        );
        assert_eq!(layout.rust_output_dir, layout.scenario_root.join("rust"));
    }

    #[test]
    fn builds_dotnet_project_command_for_oracle_runs() {
        let repo_root = PathBuf::from("C:\\repo");
        let plan =
            DifferentialRunPlan::for_scenario(repo_root.join("artifacts"), example_scenario());
        let runner = DotnetOracleRunner::from_repo_root(&repo_root);

        let command = plan.oracle_command(&runner);

        assert_eq!(command.program, "dotnet");
        assert_eq!(command.working_directory, repo_root);
        assert_eq!(command.args[0], "run");
        assert_eq!(command.args[1], "--project");
        assert_eq!(
            command.args[2],
            PathBuf::from("C:\\repo")
                .join("src")
                .join("HttpGenerator")
                .join("HttpGenerator.csproj")
                .to_string_lossy()
        );
        assert_eq!(command.args[3], "--");
        assert!(
            command
                .args
                .contains(&"--generate-intellij-tests".to_string())
        );
        assert!(
            command
                .args
                .windows(2)
                .all(|window| window != ["--output-type", "OneRequestPerFile"])
        );
        assert!(
            command
                .args
                .contains(&plan.layout.oracle_output_dir.to_string_lossy().into_owned())
        );
    }

    #[test]
    fn builds_dotnet_assembly_command_for_oracle_runs() {
        let repo_root = PathBuf::from("C:\\repo");
        let assembly_path = repo_root
            .join("src")
            .join("HttpGenerator")
            .join("bin")
            .join("Release")
            .join("net8.0")
            .join("httpgenerator.dll");
        let plan =
            DifferentialRunPlan::for_scenario(repo_root.join("artifacts"), example_scenario());
        let runner = DotnetOracleRunner::assembly(&repo_root, &assembly_path);

        let command = plan.oracle_command(&runner);

        assert_eq!(command.program, "dotnet");
        assert_eq!(command.args[0], assembly_path.to_string_lossy());
        assert_eq!(command.args[1], plan.scenario.input_path.to_string_lossy());
        assert!(!command.args.contains(&"--".to_string()));
    }

    #[test]
    fn builds_rust_cargo_command_for_rewrite_runs() {
        let repo_root = PathBuf::from("C:\\repo");
        let plan =
            DifferentialRunPlan::for_scenario(repo_root.join("artifacts"), example_scenario());
        let runner = RustCliRunner::from_repo_root(&repo_root);

        let command = plan.rust_command(&runner);

        assert_eq!(command.program, "cargo");
        assert_eq!(command.working_directory, repo_root);
        assert_eq!(command.args[0], "run");
        assert_eq!(command.args[1], "-p");
        assert_eq!(command.args[2], "httpgenerator-cli");
        assert_eq!(command.args[3], "--");
        assert_eq!(command.args[4], plan.scenario.input_path.to_string_lossy());
        assert!(
            command
                .args
                .contains(&plan.layout.rust_output_dir.to_string_lossy().into_owned())
        );
    }
}
