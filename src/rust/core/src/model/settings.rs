use serde::{Deserialize, Serialize};

use super::OutputType;

/// Configures how `.http` files are generated from a normalized API document.
///
/// Start with [`Default`] and override the fields that affect your output shape, headers, and
/// document-loading behavior.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::{GeneratorSettings, OutputType};
///
/// let settings = GeneratorSettings {
///     open_api_path: "test\\OpenAPI\\v3.0\\petstore.json".into(),
///     output_type: OutputType::OneFilePerTag,
///     skip_headers: true,
///     ..Default::default()
/// };
///
/// assert_eq!(settings.output_type, OutputType::OneFilePerTag);
/// assert!(settings.skip_headers);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorSettings {
    /// Path or URL to the source OpenAPI document.
    ///
    /// This value is also used when resolving relative server URLs into the generated `@baseUrl`
    /// variable.
    pub open_api_path: String,
    /// Literal authorization header value to emit into generated files.
    ///
    /// This is ignored for file headers when
    /// [`Self::authorization_header_from_environment_variable`] is `true`.
    pub authorization_header: Option<String>,
    /// Uses a request variable placeholder for the authorization header instead of inlining the
    /// value into file headers.
    pub authorization_header_from_environment_variable: bool,
    /// Variable name used when writing the `Authorization: {{...}}` reference.
    pub authorization_header_variable_name: String,
    /// Default content type written into the shared `@contentType` variable.
    pub content_type: String,
    /// Optional base URL override.
    ///
    /// When this is relative or templated, it is combined with server information by
    /// [`crate::resolve_base_url`].
    pub base_url: Option<String>,
    /// Output layout for generated files.
    pub output_type: OutputType,
    /// Network timeout in seconds for OpenAPI-loading workflows that fetch remote documents.
    pub timeout: u64,
    /// Appends IntelliJ HTTP Client test blocks after each rendered request.
    pub generate_intellij_tests: bool,
    /// Additional raw header lines appended to every rendered request.
    pub custom_headers: Vec<String>,
    /// Skips shared file headers such as `@baseUrl` and `@contentType`.
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
