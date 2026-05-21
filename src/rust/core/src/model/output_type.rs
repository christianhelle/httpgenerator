use serde::{Deserialize, Serialize};

/// Controls how rendered HTTP requests are grouped into output files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OutputType {
    /// Generate one `.http` file for each operation.
    ///
    /// Filenames are derived from operation names and are made unique
    /// case-insensitively.
    #[default]
    OneRequestPerFile,
    /// Generate a single `Requests.http` file containing every operation.
    OneFile,
    /// Generate one `.http` file for each operation tag.
    ///
    /// Operations without tags are grouped into `Default.http`.
    OneFilePerTag,
}
