use serde::{Deserialize, Serialize};

/// A generated `.http` file ready to be written to disk or returned to a host surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpFile {
    /// File name that should be used when persisting the generated content.
    pub filename: String,
    /// Full UTF-8 file contents.
    pub content: String,
}

impl HttpFile {
    /// Creates a generated file value from a file name and contents.
    ///
    /// # Examples
    ///
    /// ```
    /// use httpgenerator_core::HttpFile;
    ///
    /// let file = HttpFile::new("Requests.http", "GET {{baseUrl}}/pets");
    ///
    /// assert_eq!(file.filename, "Requests.http");
    /// assert!(file.content.contains("/pets"));
    /// ```
    pub fn new(filename: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            content: content.into(),
        }
    }
}

/// Collection of files produced by one generation run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeneratorResult {
    /// Generated files in output order.
    pub files: Vec<HttpFile>,
}

impl GeneratorResult {
    /// Wraps generated files into a result value.
    ///
    /// # Examples
    ///
    /// ```
    /// use httpgenerator_core::{GeneratorResult, HttpFile};
    ///
    /// let result = GeneratorResult::new(vec![HttpFile::new("Requests.http", "GET /health")]);
    ///
    /// assert_eq!(result.files.len(), 1);
    /// assert_eq!(result.files[0].filename, "Requests.http");
    /// ```
    pub fn new(files: Vec<HttpFile>) -> Self {
        Self { files }
    }
}
