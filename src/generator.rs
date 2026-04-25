use std::collections::HashSet;

use indexmap::IndexMap;
use serde_json::Value;

use crate::cli::OutputType;
use crate::naming::{capitalize_first, operation_name};
use crate::openapi::{is_absolute_url, is_http_url, operations, resolve_ref, server_url, Document};

#[derive(Debug, Clone)]
pub struct GeneratorSettings {
    pub openapi_path: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpFile {
    pub filename: String,
    pub content: String,
}

pub fn generate(
    settings: &GeneratorSettings,
    document: &Document,
) -> anyhow::Result<Vec<HttpFile>> {
    let base_url = resolve_base_url(settings, document);
    match settings.output_type {
        OutputType::OneRequestPerFile => generate_multiple_files(settings, document, &base_url),
        OutputType::OneFile => Ok(vec![HttpFile {
            filename: "Requests.http".to_string(),
            content: generate_single_file(settings, document, &base_url)?,
        }]),
        OutputType::OneFilePerTag => generate_file_per_tag(settings, document, &base_url),
    }
}

fn resolve_base_url(settings: &GeneratorSettings, document: &Document) -> String {
    let doc_server_url = server_url(document);
    let mut base_url = settings.base_url.clone().unwrap_or_default();

    if base_url.trim().is_empty() {
        base_url = doc_server_url;
    } else if !is_absolute_url(&doc_server_url) {
        base_url.push_str(&doc_server_url);
    }

    if settings
        .base_url
        .as_ref()
        .is_some_and(|url| url.starts_with("{{") && url.ends_with("}}"))
    {
        return base_url;
    }

    if !is_absolute_url(&base_url) && is_http_url(&settings.openapi_path) {
        if let Ok(url) = url::Url::parse(&settings.openapi_path) {
            base_url = format!(
                "{}://{}{}",
                url.scheme(),
                url.host_str().unwrap_or_default(),
                base_url
            );
        }
    }

    base_url
}

fn generate_single_file(
    settings: &GeneratorSettings,
    document: &Document,
    base_url: &str,
) -> anyhow::Result<String> {
    let mut content = String::new();
    write_file_headers(settings, &mut content, base_url);

    for (path, path_item, verb, operation) in iter_operations(document)? {
        content.push_str(&generate_request(
            settings, document, path, path_item, verb, operation,
        ));
        content.push('\n');
    }

    Ok(content)
}

fn generate_multiple_files(
    settings: &GeneratorSettings,
    document: &Document,
    base_url: &str,
) -> anyhow::Result<Vec<HttpFile>> {
    let mut files = Vec::new();
    let mut seen = HashSet::new();

    for (path, path_item, verb, operation) in iter_operations(document)? {
        let operation_id = operation.get("operationId").and_then(Value::as_str);
        let name = operation_name(path, &capitalize_first(verb), operation_id);
        let filename = unique_filename(&format!("{}.http", capitalize_first(&name)), &mut seen);
        let mut content = String::new();
        write_file_headers(settings, &mut content, base_url);
        content.push_str(&generate_request(
            settings, document, path, path_item, verb, operation,
        ));
        content.push('\n');
        files.push(HttpFile { filename, content });
    }

    Ok(files)
}

fn generate_file_per_tag(
    settings: &GeneratorSettings,
    document: &Document,
    base_url: &str,
) -> anyhow::Result<Vec<HttpFile>> {
    let mut by_tag = IndexMap::<String, String>::new();

    for (path, path_item, verb, operation) in iter_operations(document)? {
        let tag = first_tag(operation).unwrap_or("Default").to_string();
        let entry = by_tag.entry(tag).or_insert_with(|| {
            let mut content = String::new();
            write_file_headers(settings, &mut content, base_url);
            content
        });
        entry.push_str(&generate_request(
            settings, document, path, path_item, verb, operation,
        ));
        entry.push('\n');
    }

    Ok(by_tag
        .into_iter()
        .map(|(tag, content)| HttpFile {
            filename: format!("{}.http", capitalize_first(&tag)),
            content,
        })
        .collect())
}

fn iter_operations(document: &Document) -> anyhow::Result<Vec<(&str, &Value, &str, &Value)>> {
    let Some(paths) = document.value.get("paths").and_then(Value::as_object) else {
        return Ok(Vec::new());
    };

    let mut result = Vec::new();
    for (path, path_item) in paths {
        for (verb, operation) in operations(path_item) {
            result.push((path.as_str(), path_item, verb, operation));
        }
    }
    Ok(result)
}

fn write_file_headers(settings: &GeneratorSettings, content: &mut String, base_url: &str) {
    if settings.skip_headers {
        return;
    }

    content.push_str(&format!("@baseUrl = {base_url}\n"));
    content.push_str(&format!("@contentType = {}\n", settings.content_type));

    if let Some(header) = settings.authorization_header.as_deref() {
        if !settings.authorization_header_from_environment_variable && !header.trim().is_empty() {
            content.push_str(&format!(
                "@{} = {}\n",
                settings.authorization_header_variable_name, header
            ));
        }
    }

    content.push('\n');
}

fn generate_request(
    settings: &GeneratorSettings,
    document: &Document,
    path: &str,
    path_item: &Value,
    verb: &str,
    operation: &Value,
) -> String {
    let mut content = String::new();
    append_summary(&mut content, verb, path, operation);

    let parameter_name_map = append_parameters(
        &mut content,
        settings,
        document,
        path,
        path_item,
        verb,
        operation,
    );

    let mut url = path.replace('{', "{{").replace('}', "}}");
    let mut query_params = Vec::new();
    for (parameter_key, parameter_name) in parameter_name_map {
        if path.contains(&format!("{{{parameter_key}}}")) {
            url = url.replace(
                &format!("{{{{{parameter_key}}}}}"),
                &format!("{{{{{parameter_name}}}}}"),
            );
        } else {
            query_params.push((parameter_key, parameter_name));
        }
    }

    if !query_params.is_empty() {
        let query = query_params
            .into_iter()
            .map(|(key, name)| format!("{key}={{{{{name}}}}}"))
            .collect::<Vec<_>>()
            .join("&");
        url.push('?');
        url.push_str(&query);
    }

    content.push_str(&format!("{} {{{{baseUrl}}}}{url}\n", verb.to_uppercase()));
    content.push_str("Content-Type: {{contentType}}\n");

    if settings
        .authorization_header
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty())
        || settings.authorization_header_from_environment_variable
    {
        content.push_str(&format!(
            "Authorization: {{{{{}}}}}\n",
            settings.authorization_header_variable_name
        ));
    }

    for custom_header in &settings.custom_headers {
        content.push_str(custom_header);
        content.push('\n');
    }

    content.push('\n');
    if let Some(schema) = request_body_schema(settings, document, operation) {
        if let Some(json) = generate_sample_json(document, schema) {
            content.push_str(&json);
            content.push('\n');
        }
    }

    generate_intellij_test(settings, &mut content);
    content
}

fn append_summary(content: &mut String, verb: &str, path: &str, operation: &Value) {
    const PADDING: usize = 2;
    const SUMMARY: &str = "### Summary: ";
    const DESCRIPTION: &str = "### Description: ";

    let request = format!("### Request: {} {path}", verb.to_uppercase());
    let summary = operation.get("summary").and_then(Value::as_str);
    let description = operation.get("description").and_then(Value::as_str);

    let mut length = request.len() + PADDING;
    length = length.max(summary.map_or(0, str::len) + SUMMARY.len() + PADDING);
    length = length.max(description.map_or(0, str::len) + DESCRIPTION.len() + PADDING);

    content.push_str(&"#".repeat(length));
    content.push('\n');
    content.push_str(&request);
    content.push('\n');

    if let Some(summary) = summary.filter(|value| !value.trim().is_empty()) {
        content.push_str(SUMMARY);
        content.push_str(summary);
        content.push('\n');
    }

    if let Some(description) = description.filter(|value| !value.trim().is_empty()) {
        if description.contains('\n') {
            content.push_str(DESCRIPTION);
            content.push('\n');
            content.push_str("###   ");
            content.push_str(&description.replace("\r\n", "\n").replace('\n', "\n###   "));
            content.push('\n');
        } else {
            content.push_str(DESCRIPTION);
            content.push_str(description);
            content.push('\n');
        }
    }

    content.push_str(&"#".repeat(length));
    content.push_str("\n\n");
}

fn append_parameters(
    content: &mut String,
    settings: &GeneratorSettings,
    document: &Document,
    path: &str,
    path_item: &Value,
    verb: &str,
    operation: &Value,
) -> IndexMap<String, String> {
    let mut parameters = IndexMap::<(String, String), &Value>::new();

    for parameter in path_item
        .get("parameters")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .chain(
            operation
                .get("parameters")
                .and_then(Value::as_array)
                .into_iter()
                .flatten(),
        )
    {
        let parameter = resolve_ref(&document.value, parameter);
        let name = parameter
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("param")
            .to_string();
        let location = parameter
            .get("in")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        parameters.insert((name, location), parameter);
    }

    let mut parameter_name_map = IndexMap::new();
    let mut unnamed = 0usize;
    for ((name, location), parameter) in parameters {
        if location != "path" && location != "query" {
            continue;
        }

        let parameter_key = if name.is_empty() {
            let key = format!("_unnamed_{unnamed}");
            unnamed += 1;
            key
        } else {
            name
        };
        let parameter_name = parameter_name(settings, document, path, verb, operation, parameter);
        parameter_name_map.insert(parameter_key, parameter_name.clone());
        let default_value = parameter_default_value(parameter);
        let description = parameter
            .get("description")
            .and_then(Value::as_str)
            .map(|value| value.replace("\r\n", "\n").replace('\n', "\n### "))
            .unwrap_or_else(|| parameter_name.clone());
        content.push_str(&format!(
            "### {} Parameter: {description}\n@{parameter_name} = {default_value}\n\n",
            capitalize_first(&location)
        ));
    }

    content.push('\n');
    parameter_name_map
}

fn parameter_name(
    settings: &GeneratorSettings,
    _document: &Document,
    path: &str,
    verb: &str,
    operation: &Value,
    parameter: &Value,
) -> String {
    let name = parameter
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("param");
    if settings.output_type == OutputType::OneRequestPerFile {
        return name.to_string();
    }

    let operation_id = operation.get("operationId").and_then(Value::as_str);
    format!(
        "{}_{}",
        operation_name(path, &capitalize_first(verb), operation_id),
        name
    )
}

fn parameter_default_value(parameter: &Value) -> &'static str {
    let schema = parameter.get("schema").unwrap_or(parameter);
    match schema_type(schema).as_deref() {
        Some("integer") | Some("number") => "0",
        Some("boolean") => "true",
        _ => "str",
    }
}

fn request_body_schema<'a>(
    settings: &GeneratorSettings,
    document: &'a Document,
    operation: &'a Value,
) -> Option<&'a Value> {
    if let Some(content) = operation
        .get("requestBody")
        .map(|body| resolve_ref(&document.value, body))
        .and_then(|body| body.get("content"))
        .and_then(Value::as_object)
    {
        let media_type = content
            .iter()
            .find(|(content_type, _)| content_type.contains(&settings.content_type))
            .map(|(_, media)| media)?;
        return media_type.get("schema");
    }

    let consumes_matches = operation
        .get("consumes")
        .and_then(Value::as_array)
        .map(|consumes| {
            consumes
                .iter()
                .filter_map(Value::as_str)
                .any(|content_type| content_type.contains(&settings.content_type))
        })
        .unwrap_or(true);

    if !consumes_matches {
        return None;
    }

    operation
        .get("parameters")
        .and_then(Value::as_array)?
        .iter()
        .map(|parameter| resolve_ref(&document.value, parameter))
        .find(|parameter| parameter.get("in").and_then(Value::as_str) == Some("body"))
        .and_then(|parameter| parameter.get("schema"))
}

fn generate_sample_json(document: &Document, schema: &Value) -> Option<String> {
    let schema = resolve_ref(&document.value, schema);

    for composition in ["allOf", "oneOf", "anyOf"] {
        if let Some(first) = schema
            .get(composition)
            .and_then(Value::as_array)
            .and_then(|schemas| schemas.first())
        {
            return generate_sample_json(document, first);
        }
    }

    if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
        if !properties.is_empty() {
            let props = properties
                .iter()
                .take(3)
                .map(|(key, value)| {
                    format!("  \"{key}\": {}", property_sample_value(document, value))
                })
                .collect::<Vec<_>>()
                .join(",\n");
            return Some(format!("{{\n{props}\n}}"));
        }
    }

    let sample = match schema_type(schema).as_deref() {
        Some("object") => "{\n  \"property\": \"value\"\n}".to_string(),
        Some("array") => "[\n  \"item1\",\n  \"item2\"\n]".to_string(),
        Some("string") => "\"example\"".to_string(),
        Some("integer") | Some("number") => "0".to_string(),
        Some("boolean") => "true".to_string(),
        _ => "{}".to_string(),
    };
    Some(sample)
}

fn property_sample_value(document: &Document, schema: &Value) -> String {
    if schema.get("$ref").is_some() {
        return "{\"property\": \"value\"}".to_string();
    }

    for composition in ["allOf", "oneOf", "anyOf"] {
        if let Some(first) = schema
            .get(composition)
            .and_then(Value::as_array)
            .and_then(|schemas| schemas.first())
        {
            return property_sample_value(document, first);
        }
    }

    match schema_type(schema).as_deref() {
        Some("string") => "\"example\"".to_string(),
        Some("integer") => "0".to_string(),
        Some("number") => "0.0".to_string(),
        Some("boolean") => "true".to_string(),
        Some("array") => "[\"item\"]".to_string(),
        Some("object") => "{\"property\": \"value\"}".to_string(),
        _ => {
            let resolved = resolve_ref(&document.value, schema);
            if !std::ptr::eq(resolved, schema) {
                property_sample_value(document, resolved)
            } else {
                "\"value\"".to_string()
            }
        }
    }
}

fn schema_type(schema: &Value) -> Option<String> {
    match schema.get("type") {
        Some(Value::String(value)) => Some(value.clone()),
        Some(Value::Array(values)) => values.iter().find_map(Value::as_str).map(ToOwned::to_owned),
        _ => None,
    }
}

fn generate_intellij_test(settings: &GeneratorSettings, content: &mut String) {
    if !settings.generate_intellij_tests {
        return;
    }

    content.push('\n');
    content.push_str(
        r#"> {%
    client.test("Request executed successfully", function() {
        client.assert(
            response.status === 200, 
            "Response status is not 200");
    });
%}
"#,
    );
}

fn unique_filename(filename: &str, seen: &mut HashSet<String>) -> String {
    if seen.insert(filename.to_lowercase()) {
        return filename.to_string();
    }

    let Some((name, extension)) = filename.rsplit_once('.') else {
        return filename.to_string();
    };

    let mut counter = 2;
    loop {
        let candidate = format!("{name}_{counter}.{extension}");
        if seen.insert(candidate.to_lowercase()) {
            return candidate;
        }
        counter += 1;
    }
}

fn first_tag(operation: &Value) -> Option<&str> {
    let first = operation.get("tags")?.as_array()?.first()?;
    first
        .as_str()
        .or_else(|| first.get("name").and_then(Value::as_str))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openapi::parse_document;

    fn settings(output_type: OutputType) -> GeneratorSettings {
        GeneratorSettings {
            openapi_path: "openapi.json".to_string(),
            authorization_header: None,
            authorization_header_from_environment_variable: false,
            authorization_header_variable_name: "authorization".to_string(),
            content_type: "application/json".to_string(),
            base_url: None,
            output_type,
            timeout: 120,
            generate_intellij_tests: false,
            custom_headers: vec![],
            skip_headers: false,
        }
    }

    fn document(json: &str) -> Document {
        Document {
            value: parse_document(json).unwrap(),
            source: "openapi.json".to_string(),
        }
    }

    #[test]
    fn generates_one_file_with_qualified_parameters() {
        let document = document(
            r#"{
              "openapi":"3.0.0",
              "servers":[{"url":"/api"}],
              "paths":{"/pets/{id}":{"get":{"operationId":"getPet","parameters":[{"name":"id","in":"path","schema":{"type":"integer"}},{"name":"q","in":"query","schema":{"type":"string"}}]}}}
            }"#,
        );
        let files = generate(&settings(OutputType::OneFile), &document).unwrap();
        assert_eq!(files[0].filename, "Requests.http");
        assert!(files[0].content.contains("@GetPet_id = 0"));
        assert!(files[0]
            .content
            .contains("GET {{baseUrl}}/pets/{{GetPet_id}}?q={{GetPet_q}}"));
    }

    #[test]
    fn generates_custom_headers_and_intellij_tests() {
        let mut settings = settings(OutputType::OneRequestPerFile);
        settings.custom_headers = vec!["X-Test: 1".to_string()];
        settings.generate_intellij_tests = true;
        settings.authorization_header = Some("Bearer token".to_string());
        let document =
            document(r#"{"openapi":"3.0.0","paths":{"/pets":{"get":{"operationId":"listPets"}}}}"#);
        let files = generate(&settings, &document).unwrap();
        assert!(files[0].content.contains("@authorization = Bearer token"));
        assert!(files[0].content.contains("X-Test: 1"));
        assert!(files[0].content.contains("client.test"));
    }

    #[test]
    fn generates_request_body_from_ref() {
        let document = document(
            r##"{
              "openapi":"3.0.0",
              "paths":{"/pets":{"post":{"operationId":"addPet","requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Pet"}}}}}}},
              "components":{"schemas":{"Pet":{"type":"object","properties":{"id":{"type":"integer"},"name":{"type":"string"},"tag":{"type":"string"},"ignored":{"type":"boolean"}}}}}
            }"##,
        );
        let files = generate(&settings(OutputType::OneRequestPerFile), &document).unwrap();
        assert!(files[0].content.contains("\"id\": 0"));
        assert!(files[0].content.contains("\"name\": \"example\""));
        assert!(!files[0].content.contains("ignored"));
    }

    #[test]
    fn groups_requests_by_first_tag() {
        let document = document(
            r#"{"openapi":"3.0.0","paths":{"/pets":{"get":{"operationId":"listPets","tags":["pet"]}},"/orders":{"get":{"operationId":"listOrders","tags":["store"]}}}}"#,
        );
        let files = generate(&settings(OutputType::OneFilePerTag), &document).unwrap();
        assert_eq!(
            files
                .iter()
                .map(|file| file.filename.as_str())
                .collect::<Vec<_>>(),
            vec!["Pet.http", "Store.http"]
        );
    }

    #[test]
    fn skip_headers_omits_file_variables() {
        let mut settings = settings(OutputType::OneFile);
        settings.skip_headers = true;
        let document = document(
            r#"{"openapi":"3.0.0","servers":[{"url":"/api"}],"paths":{"/pets":{"get":{"operationId":"listPets"}}}}"#,
        );
        let files = generate(&settings, &document).unwrap();
        assert!(!files[0].content.contains("@baseUrl"));
        assert!(files[0].content.contains("GET {{baseUrl}}/pets"));
    }

    #[test]
    fn environment_authorization_writes_request_header_only() {
        let mut settings = settings(OutputType::OneFile);
        settings.authorization_header_from_environment_variable = true;
        settings.authorization_header_variable_name = "my_token".to_string();
        let document =
            document(r#"{"openapi":"3.0.0","paths":{"/pets":{"get":{"operationId":"listPets"}}}}"#);
        let files = generate(&settings, &document).unwrap();
        assert!(!files[0].content.contains("@my_token"));
        assert!(files[0].content.contains("Authorization: {{my_token}}"));
    }

    #[test]
    fn template_base_url_keeps_server_suffix() {
        let mut settings = settings(OutputType::OneFile);
        settings.base_url = Some("{{MY_BASE_URL}}".to_string());
        let document = document(
            r#"{"openapi":"3.0.0","servers":[{"url":"/api/v3"}],"paths":{"/pets":{"get":{"operationId":"listPets"}}}}"#,
        );
        let files = generate(&settings, &document).unwrap();
        assert!(files[0]
            .content
            .starts_with("@baseUrl = {{MY_BASE_URL}}/api/v3"));
    }

    #[test]
    fn duplicate_operation_names_get_unique_filenames() {
        let document = document(
            r#"{"openapi":"3.0.0","paths":{"/pets":{"get":{"operationId":"same"}},"/orders":{"get":{"operationId":"same"}}}}"#,
        );
        let files = generate(&settings(OutputType::OneRequestPerFile), &document).unwrap();
        assert_eq!(files[0].filename, "GetSame.http");
        assert_eq!(files[1].filename, "GetSame_2.http");
    }

    #[test]
    fn v2_body_parameter_generates_request_body() {
        let document = document(
            r##"{"swagger":"2.0","host":"example.com","basePath":"/v1","schemes":["https"],"paths":{"/pets":{"post":{"operationId":"addPet","consumes":["application/json"],"parameters":[{"name":"body","in":"body","schema":{"$ref":"#/definitions/Pet"}}]}}},"definitions":{"Pet":{"type":"object","properties":{"id":{"type":"integer"},"name":{"type":"string"}}}}}"##,
        );
        let files = generate(&settings(OutputType::OneRequestPerFile), &document).unwrap();
        assert!(files[0]
            .content
            .contains("@baseUrl = https://example.com/v1"));
        assert!(files[0].content.contains("\"id\": 0"));
        assert!(files[0].content.contains("\"name\": \"example\""));
    }

    #[test]
    fn non_matching_content_type_skips_body() {
        let mut settings = settings(OutputType::OneRequestPerFile);
        settings.content_type = "application/xml".to_string();
        let document = document(
            r##"{"openapi":"3.0.0","paths":{"/pets":{"post":{"operationId":"addPet","requestBody":{"content":{"application/json":{"schema":{"type":"object","properties":{"id":{"type":"integer"}}}}}}}}}}"##,
        );
        let files = generate(&settings, &document).unwrap();
        assert!(!files[0].content.contains("\"id\": 0"));
    }

    #[test]
    fn missing_paths_generates_empty_outputs() {
        let document = document(
            r#"{"openapi":"3.1.0","webhooks":{"newPet":{"post":{"operationId":"newPet"}}}}"#,
        );

        let multiple = generate(&settings(OutputType::OneRequestPerFile), &document).unwrap();
        assert!(multiple.is_empty());

        let single = generate(&settings(OutputType::OneFile), &document).unwrap();
        assert_eq!(single[0].filename, "Requests.http");
        assert!(single[0]
            .content
            .contains("@contentType = application/json"));
    }
}
