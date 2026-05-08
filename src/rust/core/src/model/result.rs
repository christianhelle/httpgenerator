use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpFile {
    pub filename: String,
    pub content: String,
}

impl HttpFile {
    pub fn new(filename: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeneratorResult {
    pub files: Vec<HttpFile>,
}

impl GeneratorResult {
    pub fn new(files: Vec<HttpFile>) -> Self {
        Self { files }
    }
}
