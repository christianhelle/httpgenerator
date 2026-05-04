use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OutputType {
    #[default]
    OneRequestPerFile,
    OneFile,
    OneFilePerTag,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorSettings {
    pub open_api_path: String,
    pub authorization_header: Option<String>,
    pub authorization_header_from_environment_variable: bool,
    pub authorization_header_variable_name: String,
    pub content_type: String,
    pub base_url: Option<String>,
    pub output_type: OutputType,
    pub timeout: u64,
    pub generate_intellij_tests: bool,
    pub custom_headers: Vec<String>,
    pub skip_headers: bool,
}

impl Default for GeneratorSettings {
    fn default() -> Self {
        Self {
            open_api_path: String::new(),
            authorization_header: None,
            authorization_header_from_environment_variable: false,
            authorization_header_variable_name: "authorization".to_string(),
            content_type: "application/json".to_string(),
            base_url: None,
            output_type: OutputType::default(),
            timeout: 120,
            generate_intellij_tests: false,
            custom_headers: Vec::new(),
            skip_headers: false,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::{GeneratorSettings, OutputType};

    #[test]
    fn generator_settings_defaults_match_current_tool() {
        let settings = GeneratorSettings::default();

        assert_eq!(settings.authorization_header, None);
        assert_eq!(settings.authorization_header_variable_name, "authorization");
        assert_eq!(settings.content_type, "application/json");
        assert_eq!(settings.output_type, OutputType::OneRequestPerFile);
        assert_eq!(settings.timeout, 120);
        assert!(settings.custom_headers.is_empty());
        assert!(!settings.skip_headers);
    }
}
