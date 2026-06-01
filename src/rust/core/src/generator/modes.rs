use std::collections::{HashMap, HashSet};

use crate::{
    GeneratorResult, GeneratorSettings, HttpFile, NormalizedOpenApiDocument, OutputType,
    capitalize_first_character, resolve_base_url, unique_filename,
};

use super::{
    render::{operation_name, render_request},
    text::{push_blank_line, push_line},
};

/// Generates one or more `.http` files from a normalized API document.
///
/// Use this when you already have a [`NormalizedOpenApiDocument`] and want the library's standard
/// `.http` rendering behavior without going through the CLI.
///
/// The returned files are shaped by [`OutputType`]:
///
/// - [`OutputType::OneRequestPerFile`] creates one file per operation
/// - [`OutputType::OneFile`] appends every operation into `Requests.http`
/// - [`OutputType::OneFilePerTag`] groups operations by their first tag
///
/// The generator also writes shared file headers such as `@baseUrl` and `@contentType` unless
/// [`GeneratorSettings::skip_headers`] is enabled. It expects a normalized document as input, so
/// document loading, source parsing, and OpenAPI validation happen before this call.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::{
///     generate_http_files, GeneratorSettings, NormalizedHttpMethod, NormalizedOpenApiDocument,
///     NormalizedOperation, NormalizedServer, NormalizedSpecificationVersion, OutputType,
/// };
///
/// let settings = GeneratorSettings {
///     open_api_path: "https://api.example.com/openapi.json".into(),
///     output_type: OutputType::OneFile,
///     ..Default::default()
/// };
///
/// let document = NormalizedOpenApiDocument {
///     specification_version: NormalizedSpecificationVersion::OpenApi30,
///     servers: vec![NormalizedServer {
///         url: "https://api.example.com".into(),
///     }],
///     operations: vec![NormalizedOperation {
///         path: "/pets".into(),
///         method: NormalizedHttpMethod::Get,
///         operation_id: Some("listPets".into()),
///         summary: Some("List pets".into()),
///         description: None,
///         tags: vec!["pets".into()],
///         parameters: vec![],
///         request_body: None,
///     }],
/// };
///
/// let result = generate_http_files(&settings, &document);
///
/// assert_eq!(result.files.len(), 1);
/// assert_eq!(result.files[0].filename, "Requests.http");
/// assert!(result.files[0].content.contains("GET {{baseUrl}}/pets"));
/// ```
pub fn generate_http_files(
    settings: &GeneratorSettings,
    document: &NormalizedOpenApiDocument,
) -> GeneratorResult {
    let server_url = document
        .servers
        .first()
        .map(|server| server.url.as_str())
        .unwrap_or_default();
    let base_url = resolve_base_url(
        &settings.open_api_path,
        Some(server_url),
        settings.base_url.as_deref(),
    );

    let mut buffers: Vec<HttpFileBuffer> = Vec::new();
    let mut key_to_index: HashMap<String, usize> = HashMap::new();
    let mut seen_filenames = HashSet::new();

    for operation in &document.operations {
        let target = render_target(settings.output_type, operation, &mut seen_filenames);
        let key = target.key.clone();
        let index = key_to_index
            .entry(key)
            .or_insert_with(|| {
                let idx = buffers.len();
                buffers.push(HttpFileBuffer {
                    filename: target.filename,
                    content: String::new(),
                });
                write_file_headers(settings, &mut buffers[idx].content, &base_url);
                idx
            });
        buffers[*index].content.push_str(&render_request(settings, operation));
        push_blank_line(&mut buffers[*index].content);
    }

    GeneratorResult::new(
        buffers
            .into_iter()
            .map(|buffer| HttpFile::new(buffer.filename, buffer.content))
            .collect(),
    )
}

struct HttpFileBuffer {
    filename: String,
    content: String,
}

struct RenderTarget {
    key: String,
    filename: String,
}

fn render_target(
    output_type: OutputType,
    operation: &crate::NormalizedOperation,
    seen_filenames: &mut HashSet<String>,
) -> RenderTarget {
    match output_type {
        OutputType::OneFile => RenderTarget {
            key: "Requests.http".to_string(),
            filename: "Requests.http".to_string(),
        },
        OutputType::OneRequestPerFile => {
            let operation_name = operation_name(operation);
            let filename = unique_filename(
                &format!("{}.http", capitalize_first_character(&operation_name)),
                seen_filenames,
            );
            RenderTarget {
                key: filename.clone(),
                filename,
            }
        }
        OutputType::OneFilePerTag => {
            let tag = operation
                .tags
                .first()
                .cloned()
                .unwrap_or_else(|| "Default".to_string());
            RenderTarget {
                filename: format!("{}.http", capitalize_first_character(&tag)),
                key: tag,
            }
        }
    }
}

fn write_file_headers(settings: &GeneratorSettings, content: &mut String, base_url: &str) {
    if settings.skip_headers {
        return;
    }

    push_line(content, &format!("@baseUrl = {base_url}"));
    push_line(
        content,
        &format!("@contentType = {}", settings.content_type),
    );

    if let Some(authorization_header) = settings
        .authorization_header
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .filter(|_| !settings.authorization_header_from_environment_variable)
    {
        push_line(
            content,
            &format!(
                "@{} = {authorization_header}",
                settings.authorization_header_variable_name
            ),
        );
    }

    push_blank_line(content);
}
