use std::{fmt, path::Path};

use crate::{ContentFormatDetectionError, OpenApiSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenApiContentFormat {
    Json,
    Yaml,
}

impl OpenApiContentFormat {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Json => "JSON",
            Self::Yaml => "YAML",
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let extension = path.as_ref().extension()?.to_str()?;

        Self::from_extension(extension)
    }

    fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_ascii_lowercase().as_str() {
            "json" => Some(Self::Json),
            "yaml" | "yml" => Some(Self::Yaml),
            _ => None,
        }
    }
}

impl fmt::Display for OpenApiContentFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub fn detect_content_format(
    source: Option<&OpenApiSource>,
    content: &str,
) -> Result<OpenApiContentFormat, ContentFormatDetectionError> {
    if normalized_content(content).is_empty() {
        return Err(ContentFormatDetectionError::EmptyContent);
    }

    if let Some(format) = source.and_then(OpenApiSource::format_hint) {
        return Ok(format);
    }

    sniff_content_format(content)
}

pub fn sniff_content_format(
    content: &str,
) -> Result<OpenApiContentFormat, ContentFormatDetectionError> {
    let normalized = normalized_content(content);

    if normalized.is_empty() {
        return Err(ContentFormatDetectionError::EmptyContent);
    }

    match normalized.chars().next() {
        Some('{') | Some('[') => Ok(OpenApiContentFormat::Json),
        Some(_) if looks_like_yaml(normalized) => Ok(OpenApiContentFormat::Yaml),
        _ => Err(ContentFormatDetectionError::UnknownFormat),
    }
}

fn normalized_content(content: &str) -> &str {
    content
        .strip_prefix('\u{feff}')
        .unwrap_or(content)
        .trim_start()
}

fn looks_like_yaml(content: &str) -> bool {
    content
        .lines()
        .map(|line| line.split('#').next().unwrap_or_default().trim())
        .find(|line| !line.is_empty())
        .is_some_and(|line| {
            let looks_like_mapping = line.find(':').is_some_and(|index| {
                let key = line[..index].trim();
                let value = line[index + 1..].chars().next();

                !key.is_empty() && value.map(char::is_whitespace).unwrap_or(true)
            });

            line == "---"
                || line.starts_with("%YAML")
                || line.starts_with("- ")
                || looks_like_mapping
        })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::classify_source;

    use super::{
        ContentFormatDetectionError, OpenApiContentFormat, detect_content_format,
        sniff_content_format,
    };

    #[test]
    fn detects_json_from_path_extension() {
        let format = OpenApiContentFormat::from_path(Path::new("petstore.json"));

        assert_eq!(format, Some(OpenApiContentFormat::Json));
    }

    #[test]
    fn detects_yaml_from_path_extension_case_insensitively() {
        let format = OpenApiContentFormat::from_path(Path::new("petstore.YML"));

        assert_eq!(format, Some(OpenApiContentFormat::Yaml));
    }

    #[test]
    fn prefers_source_hint_when_available() {
        let source = classify_source("https://example.com/openapi.yaml?download=1").unwrap();

        let format = detect_content_format(Some(&source), "{\"openapi\":\"3.1.0\"}").unwrap();

        assert_eq!(format, OpenApiContentFormat::Yaml);
    }

    #[test]
    fn falls_back_to_content_sniffing_when_source_has_no_known_extension() {
        let source = classify_source("test\\OpenAPI\\petstore").unwrap();

        let format = detect_content_format(Some(&source), "{\"openapi\":\"3.1.0\"}").unwrap();

        assert_eq!(format, OpenApiContentFormat::Json);
    }

    #[test]
    fn sniffs_json_after_utf8_bom() {
        let format = sniff_content_format("\u{feff}\n  {\"openapi\":\"3.0.0\"}").unwrap();

        assert_eq!(format, OpenApiContentFormat::Json);
    }

    #[test]
    fn sniffs_yaml_from_mapping_content() {
        let format = sniff_content_format("openapi: 3.0.0\ninfo:\n  title: Example").unwrap();

        assert_eq!(format, OpenApiContentFormat::Yaml);
    }

    #[test]
    fn returns_empty_content_error_for_blank_input() {
        let error = sniff_content_format("  \n\t").unwrap_err();

        assert_eq!(error, ContentFormatDetectionError::EmptyContent);
    }

    #[test]
    fn returns_unknown_format_for_unrecognized_content() {
        let error = sniff_content_format("not a document format").unwrap_err();

        assert_eq!(error, ContentFormatDetectionError::UnknownFormat);
    }

    #[test]
    fn does_not_treat_urls_as_yaml_content() {
        let error = sniff_content_format("https://example.com/openapi.json").unwrap_err();

        assert_eq!(error, ContentFormatDetectionError::UnknownFormat);
    }
}
