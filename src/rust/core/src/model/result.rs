use serde::{Deserialize, Serialize};

/// A generated `.http` file held in memory.
///
/// Host applications can write this file to disk, display it directly, or
/// transform it before presenting it to users.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpFile {
    /// Suggested file name, including the `.http` extension.
    pub filename: String,
    /// Complete file content.
    pub content: String,
}

impl HttpFile {
    /// Creates a generated file from a filename and content.
    ///
    /// # Example
    ///
    /// ```
    /// use httpgenerator_core::HttpFile;
    ///
    /// let file = HttpFile::new("Requests.http", "GET https://example.com");
    ///
    /// assert_eq!(file.filename, "Requests.http");
    /// assert_eq!(file.content, "GET https://example.com");
    /// ```
    pub fn new(filename: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            content: content.into(),
        }
    }
}

/// The complete output of a generation run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeneratorResult {
    /// Generated files in the order they should be written or displayed.
    pub files: Vec<HttpFile>,
}

impl GeneratorResult {
    /// Creates a generation result from generated files.
    ///
    /// # Example
    ///
    /// ```
    /// use httpgenerator_core::{GeneratorResult, HttpFile};
    ///
    /// let result = GeneratorResult::new(vec![HttpFile::new("Requests.http", "")]);
    ///
    /// assert_eq!(result.files.len(), 1);
    /// ```
    pub fn new(files: Vec<HttpFile>) -> Self {
        Self { files }
    }
}
