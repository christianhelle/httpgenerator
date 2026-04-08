use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
    process::Command,
};

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
pub struct CommandOutput {
    pub command: CommandSpec,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    pub fn succeeded(&self) -> bool {
        self.exit_code == Some(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputFileSnapshot {
    pub relative_path: PathBuf,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectorySnapshot {
    pub root: PathBuf,
    pub files: Vec<OutputFileSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputDifference {
    MissingFromOracle { relative_path: PathBuf },
    MissingFromRust { relative_path: PathBuf },
    ContentMismatch { relative_path: PathBuf },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryComparison {
    pub oracle: DirectorySnapshot,
    pub rust: DirectorySnapshot,
    pub differences: Vec<OutputDifference>,
}

impl DirectoryComparison {
    pub fn is_match(&self) -> bool {
        self.differences.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DifferentialRunResult {
    pub plan: DifferentialRunPlan,
    pub oracle_run: CommandOutput,
    pub rust_run: CommandOutput,
    pub comparison: DirectoryComparison,
}

impl DifferentialRunResult {
    pub fn is_match(&self) -> bool {
        self.oracle_run.succeeded() && self.rust_run.succeeded() && self.comparison.is_match()
    }

    pub fn mismatch_report(&self) -> String {
        let mut lines = Vec::new();

        append_command_report(&mut lines, "Oracle", &self.oracle_run);
        append_command_report(&mut lines, "Rust", &self.rust_run);

        for difference in &self.comparison.differences {
            match difference {
                OutputDifference::MissingFromOracle { relative_path } => {
                    lines.push(format!("Only Rust generated '{}'", relative_path.display()))
                }
                OutputDifference::MissingFromRust { relative_path } => lines.push(format!(
                    "Only the .NET oracle generated '{}'",
                    relative_path.display()
                )),
                OutputDifference::ContentMismatch { relative_path } => lines.push(format!(
                    "File contents differ for '{}'",
                    relative_path.display()
                )),
            }
        }

        if lines.is_empty() {
            "no differences".to_string()
        } else {
            lines.join("\n")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DifferentialRunError {
    PrepareArtifacts {
        path: PathBuf,
        reason: String,
    },
    RunCommand {
        program: String,
        working_directory: PathBuf,
        reason: String,
    },
    SnapshotDirectory {
        path: PathBuf,
        reason: String,
    },
}

impl fmt::Display for DifferentialRunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PrepareArtifacts { path, reason } => write!(
                formatter,
                "failed to prepare compatibility artifacts at '{}': {reason}",
                path.display()
            ),
            Self::RunCommand {
                program,
                working_directory,
                reason,
            } => write!(
                formatter,
                "failed to launch '{program}' in '{}': {reason}",
                working_directory.display()
            ),
            Self::SnapshotDirectory { path, reason } => write!(
                formatter,
                "failed to snapshot generated output in '{}': {reason}",
                path.display()
            ),
        }
    }
}

impl Error for DifferentialRunError {}

pub fn execute_differential_plan(
    plan: &DifferentialRunPlan,
    oracle_runner: &DotnetOracleRunner,
    rust_runner: &RustCliRunner,
) -> Result<DifferentialRunResult, DifferentialRunError> {
    prepare_artifact_directories(&plan.layout)?;

    let oracle_run = run_command(&plan.oracle_command(oracle_runner))?;
    let rust_run = run_command(&plan.rust_command(rust_runner))?;
    let comparison =
        compare_output_directories(&plan.layout.oracle_output_dir, &plan.layout.rust_output_dir)?;

    Ok(DifferentialRunResult {
        plan: plan.clone(),
        oracle_run,
        rust_run,
        comparison,
    })
}

fn append_command_report(lines: &mut Vec<String>, label: &str, output: &CommandOutput) {
    if output.succeeded() {
        return;
    }

    lines.push(format!(
        "{label} command exited with {:?}: {} {}",
        output.exit_code,
        output.command.program,
        output.command.args.join(" ")
    ));
    if !output.stdout.trim().is_empty() {
        lines.push(format!("{label} stdout:\n{}", output.stdout.trim()));
    }
    if !output.stderr.trim().is_empty() {
        lines.push(format!("{label} stderr:\n{}", output.stderr.trim()));
    }
}

fn prepare_artifact_directories(layout: &ScenarioOutputLayout) -> Result<(), DifferentialRunError> {
    if layout.scenario_root.exists() {
        fs::remove_dir_all(&layout.scenario_root).map_err(|error| {
            DifferentialRunError::PrepareArtifacts {
                path: layout.scenario_root.clone(),
                reason: error.to_string(),
            }
        })?;
    }

    fs::create_dir_all(&layout.scenario_root).map_err(|error| {
        DifferentialRunError::PrepareArtifacts {
            path: layout.scenario_root.clone(),
            reason: error.to_string(),
        }
    })
}

fn run_command(command: &CommandSpec) -> Result<CommandOutput, DifferentialRunError> {
    let output = Command::new(&command.program)
        .args(&command.args)
        .current_dir(&command.working_directory)
        .output()
        .map_err(|error| DifferentialRunError::RunCommand {
            program: command.program.clone(),
            working_directory: command.working_directory.clone(),
            reason: error.to_string(),
        })?;

    Ok(CommandOutput {
        command: command.clone(),
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

fn compare_output_directories(
    oracle_root: &Path,
    rust_root: &Path,
) -> Result<DirectoryComparison, DifferentialRunError> {
    let oracle = snapshot_directory(oracle_root)?;
    let rust = snapshot_directory(rust_root)?;

    let oracle_files = oracle
        .files
        .iter()
        .map(|file| (file.relative_path.clone(), file.bytes.as_slice()))
        .collect::<BTreeMap<_, _>>();
    let rust_files = rust
        .files
        .iter()
        .map(|file| (file.relative_path.clone(), file.bytes.as_slice()))
        .collect::<BTreeMap<_, _>>();
    let mut relative_paths = BTreeSet::new();

    relative_paths.extend(oracle_files.keys().cloned());
    relative_paths.extend(rust_files.keys().cloned());

    let differences = relative_paths
        .into_iter()
        .filter_map(|relative_path| {
            match (
                oracle_files.get(&relative_path),
                rust_files.get(&relative_path),
            ) {
                (Some(_), None) => Some(OutputDifference::MissingFromRust { relative_path }),
                (None, Some(_)) => Some(OutputDifference::MissingFromOracle { relative_path }),
                (Some(oracle_bytes), Some(rust_bytes)) if oracle_bytes != rust_bytes => {
                    Some(OutputDifference::ContentMismatch { relative_path })
                }
                _ => None,
            }
        })
        .collect();

    Ok(DirectoryComparison {
        oracle,
        rust,
        differences,
    })
}

fn snapshot_directory(root: &Path) -> Result<DirectorySnapshot, DifferentialRunError> {
    let mut files = Vec::new();

    if root.exists() {
        collect_directory_files(root, root, &mut files)?;
        files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    }

    Ok(DirectorySnapshot {
        root: root.to_path_buf(),
        files,
    })
}

fn collect_directory_files(
    root: &Path,
    directory: &Path,
    files: &mut Vec<OutputFileSnapshot>,
) -> Result<(), DifferentialRunError> {
    for entry in
        fs::read_dir(directory).map_err(|error| DifferentialRunError::SnapshotDirectory {
            path: directory.to_path_buf(),
            reason: error.to_string(),
        })?
    {
        let entry = entry.map_err(|error| DifferentialRunError::SnapshotDirectory {
            path: directory.to_path_buf(),
            reason: error.to_string(),
        })?;
        let path = entry.path();
        if path.is_dir() {
            collect_directory_files(root, &path, files)?;
        } else if path.is_file() {
            let relative_path = path
                .strip_prefix(root)
                .expect("file path should always be below the snapshot root")
                .to_path_buf();
            let bytes =
                fs::read(&path).map_err(|error| DifferentialRunError::SnapshotDirectory {
                    path: path.clone(),
                    reason: error.to_string(),
                })?;
            files.push(OutputFileSnapshot {
                relative_path,
                bytes,
            });
        }
    }

    Ok(())
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
                .join("legacy")
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
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use httpgenerator_core::OutputType;

    use super::{
        DifferentialRunPlan, DotnetOracleRunner, RustCliRunner, ScenarioOutputLayout,
        compare_output_directories, scenario_directory_name,
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

    fn temp_directory(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "httpgenerator-compat-tests-{name}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
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
                .join("legacy")
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
            .join("legacy")
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

    #[test]
    fn compares_directory_outputs_and_reports_mismatches() {
        let root = temp_directory("compare-output");
        let oracle_dir = root.join("oracle");
        let rust_dir = root.join("rust");
        fs::create_dir_all(&oracle_dir).unwrap();
        fs::create_dir_all(&rust_dir).unwrap();

        fs::write(oracle_dir.join("Requests.http"), "oracle").unwrap();
        fs::write(oracle_dir.join("OnlyOracle.http"), "only-oracle").unwrap();
        fs::write(rust_dir.join("Requests.http"), "rust").unwrap();
        fs::write(rust_dir.join("OnlyRust.http"), "only-rust").unwrap();

        let comparison = compare_output_directories(&oracle_dir, &rust_dir).unwrap();

        assert_eq!(comparison.oracle.files.len(), 2);
        assert_eq!(comparison.rust.files.len(), 2);
        assert!(comparison.differences.iter().any(|difference| matches!(
            difference,
            super::OutputDifference::ContentMismatch { relative_path }
                if relative_path == &PathBuf::from("Requests.http")
        )));
        assert!(comparison.differences.iter().any(|difference| matches!(
            difference,
            super::OutputDifference::MissingFromRust { relative_path }
                if relative_path == &PathBuf::from("OnlyOracle.http")
        )));
        assert!(comparison.differences.iter().any(|difference| matches!(
            difference,
            super::OutputDifference::MissingFromOracle { relative_path }
                if relative_path == &PathBuf::from("OnlyRust.http")
        )));

        let _ = fs::remove_dir_all(root);
    }
}
