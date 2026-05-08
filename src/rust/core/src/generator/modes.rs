use std::collections::HashSet;

use crate::{
    GeneratorResult, GeneratorSettings, HttpFile, NormalizedOpenApiDocument, OutputType,
    capitalize_first_character, resolve_base_url, unique_filename,
};

use super::{
    render::{operation_name, render_request},
    text::{push_blank_line, push_line},
};

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

    match settings.output_type {
        OutputType::OneRequestPerFile => generate_multiple_files(settings, document, &base_url),
        OutputType::OneFile => generate_single_file(settings, document, &base_url),
        OutputType::OneFilePerTag => generate_file_per_tag(settings, document, &base_url),
    }
}

fn generate_single_file(
    settings: &GeneratorSettings,
    document: &NormalizedOpenApiDocument,
    base_url: &str,
) -> GeneratorResult {
    let mut content = String::new();
    write_file_headers(settings, &mut content, base_url);

    for operation in &document.operations {
        content.push_str(&render_request(settings, operation));
        push_blank_line(&mut content);
    }

    GeneratorResult::new(vec![HttpFile::new("Requests.http", content)])
}

fn generate_multiple_files(
    settings: &GeneratorSettings,
    document: &NormalizedOpenApiDocument,
    base_url: &str,
) -> GeneratorResult {
    let mut files = Vec::with_capacity(document.operations.len());
    let mut seen_filenames = HashSet::new();

    for operation in &document.operations {
        let operation_name = operation_name(operation);
        let filename = unique_filename(
            &format!("{}.http", capitalize_first_character(&operation_name)),
            &mut seen_filenames,
        );
        let mut content = String::new();
        write_file_headers(settings, &mut content, base_url);
        content.push_str(&render_request(settings, operation));
        push_blank_line(&mut content);
        files.push(HttpFile::new(filename, content));
    }

    GeneratorResult::new(files)
}

fn generate_file_per_tag(
    settings: &GeneratorSettings,
    document: &NormalizedOpenApiDocument,
    base_url: &str,
) -> GeneratorResult {
    let mut contents: Vec<(String, String)> = Vec::new();

    for operation in &document.operations {
        let tag = operation
            .tags
            .first()
            .cloned()
            .unwrap_or_else(|| "Default".to_string());

        let buffer = if let Some((_, content)) = contents.iter_mut().find(|(name, _)| *name == tag)
        {
            content
        } else {
            contents.push((tag.clone(), String::new()));
            let (_, content) = contents.last_mut().expect("tag entry was just added");
            write_file_headers(settings, content, base_url);
            content
        };

        buffer.push_str(&render_request(settings, operation));
        push_blank_line(buffer);
    }

    GeneratorResult::new(
        contents
            .into_iter()
            .map(|(tag, content)| {
                HttpFile::new(
                    format!("{}.http", capitalize_first_character(&tag)),
                    content,
                )
            })
            .collect(),
    )
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
