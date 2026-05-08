use serde::{Deserialize, Serialize};

use super::OutputType;

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
