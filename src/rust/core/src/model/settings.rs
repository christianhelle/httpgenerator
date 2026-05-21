use serde::{Deserialize, Serialize};

use super::OutputType;

/// Options that control `.http` rendering.
///
/// `GeneratorSettings` is intentionally serializable so host applications can
/// move settings between CLI, editor, and test surfaces without translating
/// fields. Defaults match the command-line tool's standard behavior.
///
/// # Example
///
/// ```
/// use httpgenerator_core::{GeneratorSettings, OutputType};
///
/// let settings = GeneratorSettings {
///     open_api_path: "openapi.json".to_string(),
///     output_type: OutputType::OneFile,
///     base_url: Some("https://api.example.com".to_string()),
///     ..GeneratorSettings::default()
/// };
///
/// assert_eq!(settings.content_type, "application/json");
/// assert_eq!(settings.timeout, 120);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorSettings {
    /// Original OpenAPI input path or URL.
    ///
    /// This is used for relative server URL resolution and for downstream hosts
    /// that need to report the source being generated.
    pub open_api_path: String,
    /// Optional authorization header value to emit into generated files.
    ///
    /// When [`authorization_header_from_environment_variable`](Self::authorization_header_from_environment_variable)
    /// is `true`, this value is not written as a literal header variable.
    pub authorization_header: Option<String>,
    /// Indicates whether the authorization value should be read by the HTTP
    /// client from an environment variable rather than emitted literally.
    pub authorization_header_from_environment_variable: bool,
    /// Variable name used for authorization placeholders in generated requests.
    pub authorization_header_variable_name: String,
    /// Default content type used for request bodies.
    pub content_type: String,
    /// Optional base URL override.
    ///
    /// If this is `None`, the first normalized server URL is used. If this is a
    /// templated value such as `{{MY_BASE_URL}}`, relative server paths are
    /// appended but the template itself is preserved.
    pub base_url: Option<String>,
    /// Output grouping mode.
    pub output_type: OutputType,
    /// Timeout value, in seconds, carried through for host compatibility.
    pub timeout: u64,
    /// Indicates whether IntelliJ HTTP Client test snippets should be emitted.
    pub generate_intellij_tests: bool,
    /// Additional raw header lines to include in generated requests.
    pub custom_headers: Vec<String>,
    /// Skips generated file-level variables such as `@baseUrl` and
    /// `@contentType`.
    pub skip_headers: bool,
}

impl Default for GeneratorSettings {
    /// Creates settings that match the default CLI behavior.
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
