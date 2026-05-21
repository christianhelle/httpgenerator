use serde::{Deserialize, Serialize};

/// Selects how generated requests are split across output files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OutputType {
    /// Writes each operation into its own `.http` file.
    ///
    /// File names are derived from operation names and deduplicated when needed.
    #[default]
    OneRequestPerFile,
    /// Writes every operation into a single `Requests.http` file.
    OneFile,
    /// Groups operations by their first tag and writes one file per tag.
    ///
    /// Operations without tags fall back to a `Default.http` group.
    OneFilePerTag,
}
