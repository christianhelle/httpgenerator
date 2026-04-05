use std::collections::HashSet;

use crate::{
    GeneratorResult, GeneratorSettings, HttpFile, NormalizedInlineParameter,
    NormalizedOpenApiDocument, NormalizedOperation, NormalizedParameter,
    NormalizedParameterLocation, NormalizedRequestBody, NormalizedSchema, NormalizedSchemaType,
    OutputType, capitalize_first_character, generate_operation_name, prefix_line_breaks,
    resolve_base_url, unique_filename,
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

fn render_request(settings: &GeneratorSettings, operation: &NormalizedOperation) -> String {
    let mut content = String::new();
    append_summary(operation, &mut content);
    let parameter_name_map = append_parameters(settings, operation, &mut content);

    let mut url = operation.path.replace('{', "{{").replace('}', "}}");
    let mut query_parameters = Vec::new();

    for (original_name, generated_name) in &parameter_name_map {
        if operation.path.contains(&format!("{{{original_name}}}")) {
            url = url.replace(
                &format!("{{{{{original_name}}}}}"),
                &format!("{{{{{generated_name}}}}}"),
            );
        } else {
            query_parameters.push((original_name, generated_name));
        }
    }

    if !query_parameters.is_empty() {
        url.push('?');
        url.push_str(
            &query_parameters
                .iter()
                .map(|(name, generated)| format!("{name}={{{{{generated}}}}}"))
                .collect::<Vec<_>>()
                .join("&"),
        );
    }

    push_line(
        &mut content,
        &format!(
            "{} {{{{baseUrl}}}}{url}",
            operation.method.as_str().to_ascii_uppercase()
        ),
    );
    push_line(&mut content, "Content-Type: {{contentType}}");

    if settings.authorization_header_from_environment_variable
        || settings
            .authorization_header
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
    {
        push_line(
            &mut content,
            &format!(
                "Authorization: {{{{{}}}}}",
                settings.authorization_header_variable_name
            ),
        );
    }

    for header in &settings.custom_headers {
        push_line(&mut content, header);
    }

    push_blank_line(&mut content);

    let request_body = match &operation.request_body {
        Some(NormalizedRequestBody::Inline(request_body)) => request_body,
        _ => {
            generate_intellij_test(settings, &mut content);
            return content;
        }
    };

    let Some(media_type) = request_body
        .content
        .iter()
        .find(|content| content.content_type.contains(&settings.content_type))
    else {
        generate_intellij_test(settings, &mut content);
        return content;
    };

    let Some(schema) = media_type.schema.as_ref() else {
        generate_intellij_test(settings, &mut content);
        return content;
    };

    content.push_str(&generate_sample_json(schema));
    push_line(&mut content, "");
    generate_intellij_test(settings, &mut content);
    content
}

fn generate_intellij_test(settings: &GeneratorSettings, content: &mut String) {
    if !settings.generate_intellij_tests {
        return;
    }

    push_blank_line(content);
    push_line(content, "> {%");
    push_line(
        content,
        "    client.test(\"Request executed successfully\", function() {",
    );
    push_line(content, "        client.assert(");
    push_line(content, "            response.status === 200, ");
    push_line(content, "            \"Response status is not 200\");");
    push_line(content, "    });");
    push_line(content, "%}");
}

fn append_summary(operation: &NormalizedOperation, content: &mut String) {
    const PADDING: usize = 2;
    const SUMMARY_PREFIX: &str = "### Summary: ";
    const DESCRIPTION_PREFIX: &str = "### Description: ";

    let request = format!(
        "### Request: {} {}",
        operation.method.as_str().to_ascii_uppercase(),
        operation.path
    );
    let summary_length = operation
        .summary
        .as_deref()
        .map(|summary| SUMMARY_PREFIX.chars().count() + summary.chars().count() + PADDING)
        .unwrap_or_default();
    let description_length = operation
        .description
        .as_deref()
        .map(|description| {
            DESCRIPTION_PREFIX.chars().count() + description.chars().count() + PADDING
        })
        .unwrap_or_default();
    let border = "#".repeat(
        request
            .chars()
            .count()
            .saturating_add(PADDING)
            .max(summary_length)
            .max(description_length),
    );

    push_line(content, &border);
    push_line(content, &request);

    if let Some(summary) = operation
        .summary
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        push_line(content, &format!("{SUMMARY_PREFIX}{summary}"));
    }

    if let Some(description) = operation
        .description
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        let normalized = description.replace("\r\n", "\n");
        if normalized.contains('\n') {
            push_line(content, DESCRIPTION_PREFIX);
            for line in normalized.split('\n') {
                push_line(content, &format!("###   {line}"));
            }
        } else {
            push_line(content, &format!("{DESCRIPTION_PREFIX}{description}"));
        }
    }

    push_line(content, &border);
    push_blank_line(content);
}

fn append_parameters(
    settings: &GeneratorSettings,
    operation: &NormalizedOperation,
    content: &mut String,
) -> Vec<(String, String)> {
    let mut parameter_name_map = Vec::new();

    for parameter in operation.parameters.iter().filter_map(parameter_as_inline) {
        if !matches!(
            parameter.location,
            NormalizedParameterLocation::Path | NormalizedParameterLocation::Query
        ) {
            continue;
        }

        let parameter_name = parameter_name(settings, operation, parameter);
        let description = prefix_line_breaks(parameter.description.as_deref(), "###")
            .unwrap_or_else(|| parameter_name.clone());

        push_line(
            content,
            &format!(
                "### {} Parameter: {description}",
                parameter_location_name(parameter.location)
            ),
        );
        push_line(
            content,
            &format!(
                "@{parameter_name} = {}",
                parameter_default_value(parameter.schema.as_ref())
            ),
        );
        push_blank_line(content);

        parameter_name_map.push((parameter.name.clone(), parameter_name));
    }

    push_blank_line(content);
    parameter_name_map
}

fn parameter_as_inline(parameter: &NormalizedParameter) -> Option<&NormalizedInlineParameter> {
    match parameter {
        NormalizedParameter::Inline(parameter) => Some(parameter),
        NormalizedParameter::Reference { .. } => None,
    }
}

fn parameter_name(
    settings: &GeneratorSettings,
    operation: &NormalizedOperation,
    parameter: &NormalizedInlineParameter,
) -> String {
    if settings.output_type == OutputType::OneRequestPerFile {
        return parameter.name.clone();
    }

    format!("{}_{}", operation_name(operation), parameter.name)
}

fn operation_name(operation: &NormalizedOperation) -> String {
    generate_operation_name(
        operation.method.as_str(),
        &operation.path,
        operation.operation_id.as_deref(),
    )
}

fn parameter_default_value(schema: Option<&NormalizedSchema>) -> &'static str {
    let Some(schema) = schema else {
        return "str";
    };

    if schema.types.contains(&NormalizedSchemaType::Integer)
        || schema.types.contains(&NormalizedSchemaType::Number)
    {
        return "0";
    }

    if schema.types.contains(&NormalizedSchemaType::Boolean) {
        return "true";
    }

    "str"
}

fn generate_sample_json(schema: &NormalizedSchema) -> String {
    if let Some(all_of) = schema.all_of.first() {
        return generate_sample_json(all_of);
    }

    if let Some(one_of) = schema.one_of.first() {
        return generate_sample_json(one_of);
    }

    if let Some(any_of) = schema.any_of.first() {
        return generate_sample_json(any_of);
    }

    if !schema.properties.is_empty() {
        let properties = schema
            .properties
            .iter()
            .take(3)
            .map(|property| {
                format!(
                    "  \"{}\": {}",
                    property.name,
                    property_sample_value(&property.schema)
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");
        return format!("{{\n{properties}\n}}");
    }

    if schema.types.contains(&NormalizedSchemaType::Object) {
        return "{\n  \"property\": \"value\"\n}".to_string();
    }

    if schema.types.contains(&NormalizedSchemaType::Array) {
        return "[\n  \"item1\",\n  \"item2\"\n]".to_string();
    }

    if schema.types.contains(&NormalizedSchemaType::String) {
        return "\"example\"".to_string();
    }

    if schema.types.contains(&NormalizedSchemaType::Integer)
        || schema.types.contains(&NormalizedSchemaType::Number)
    {
        return "0".to_string();
    }

    if schema.types.contains(&NormalizedSchemaType::Boolean) {
        return "true".to_string();
    }

    "{}".to_string()
}

fn property_sample_value(schema: &NormalizedSchema) -> String {
    if let Some(all_of) = schema.all_of.first() {
        return property_sample_value(all_of);
    }

    if let Some(one_of) = schema.one_of.first() {
        return property_sample_value(one_of);
    }

    if let Some(any_of) = schema.any_of.first() {
        return property_sample_value(any_of);
    }

    if schema.types.contains(&NormalizedSchemaType::String) {
        return "\"example\"".to_string();
    }

    if schema.types.contains(&NormalizedSchemaType::Integer) {
        return "0".to_string();
    }

    if schema.types.contains(&NormalizedSchemaType::Number) {
        return "0.0".to_string();
    }

    if schema.types.contains(&NormalizedSchemaType::Boolean) {
        return "true".to_string();
    }

    if schema.types.contains(&NormalizedSchemaType::Array) {
        return "[\"item\"]".to_string();
    }

    if schema.types.contains(&NormalizedSchemaType::Object) || !schema.properties.is_empty() {
        return "{\"property\": \"value\"}".to_string();
    }

    "\"value\"".to_string()
}

fn parameter_location_name(location: NormalizedParameterLocation) -> &'static str {
    match location {
        NormalizedParameterLocation::Path => "Path",
        NormalizedParameterLocation::Query => "Query",
        NormalizedParameterLocation::Header => "Header",
        NormalizedParameterLocation::Cookie => "Cookie",
    }
}

fn push_line(content: &mut String, line: &str) {
    content.push_str(line);
    content.push_str(newline());
}

fn push_blank_line(content: &mut String) {
    content.push_str(newline());
}

fn newline() -> &'static str {
    if cfg!(windows) { "\r\n" } else { "\n" }
}

#[cfg(test)]
mod tests {
    use crate::{
        GeneratorSettings, NormalizedHttpMethod, NormalizedInlineParameter,
        NormalizedInlineRequestBody, NormalizedMediaType, NormalizedOpenApiDocument,
        NormalizedOperation, NormalizedParameter, NormalizedParameterLocation,
        NormalizedRequestBody, NormalizedSchema, NormalizedSchemaProperty, NormalizedSchemaType,
        NormalizedServer, NormalizedSpecificationVersion, OutputType, generate_http_files,
    };

    fn sample_document() -> NormalizedOpenApiDocument {
        NormalizedOpenApiDocument {
            specification_version: NormalizedSpecificationVersion::OpenApi30,
            servers: vec![NormalizedServer {
                url: "/api/v3".to_string(),
            }],
            operations: vec![
                NormalizedOperation {
                    path: "/pet".to_string(),
                    method: NormalizedHttpMethod::Put,
                    operation_id: Some("updatePet".to_string()),
                    summary: Some("Update an existing pet".to_string()),
                    description: Some("Update an existing pet by Id".to_string()),
                    tags: vec!["Pet".to_string()],
                    parameters: Vec::new(),
                    request_body: Some(NormalizedRequestBody::Inline(
                        NormalizedInlineRequestBody {
                            description: None,
                            required: true,
                            content: vec![NormalizedMediaType {
                                content_type: "application/json".to_string(),
                                schema: Some(NormalizedSchema {
                                    properties: vec![
                                        NormalizedSchemaProperty {
                                            name: "id".to_string(),
                                            schema: NormalizedSchema {
                                                types: vec![NormalizedSchemaType::Integer],
                                                ..NormalizedSchema::default()
                                            },
                                        },
                                        NormalizedSchemaProperty {
                                            name: "name".to_string(),
                                            schema: NormalizedSchema {
                                                types: vec![NormalizedSchemaType::String],
                                                ..NormalizedSchema::default()
                                            },
                                        },
                                        NormalizedSchemaProperty {
                                            name: "category".to_string(),
                                            schema: NormalizedSchema {
                                                types: vec![NormalizedSchemaType::Object],
                                                ..NormalizedSchema::default()
                                            },
                                        },
                                    ],
                                    ..NormalizedSchema::default()
                                }),
                            }],
                        },
                    )),
                },
                NormalizedOperation {
                    path: "/pet/{petId}".to_string(),
                    method: NormalizedHttpMethod::Get,
                    operation_id: Some("getPetById".to_string()),
                    summary: Some("Find pet by ID".to_string()),
                    description: Some("Returns a single pet".to_string()),
                    tags: vec!["Pet".to_string()],
                    parameters: vec![NormalizedParameter::Inline(NormalizedInlineParameter {
                        name: "petId".to_string(),
                        location: NormalizedParameterLocation::Path,
                        description: Some("ID of pet to return".to_string()),
                        required: true,
                        schema: Some(NormalizedSchema {
                            types: vec![NormalizedSchemaType::Integer],
                            ..NormalizedSchema::default()
                        }),
                    })],
                    request_body: None,
                },
                NormalizedOperation {
                    path: "/user/login".to_string(),
                    method: NormalizedHttpMethod::Get,
                    operation_id: Some("loginUser".to_string()),
                    summary: Some("Logs user into the system".to_string()),
                    description: None,
                    tags: vec!["User".to_string()],
                    parameters: vec![
                        NormalizedParameter::Inline(NormalizedInlineParameter {
                            name: "username".to_string(),
                            location: NormalizedParameterLocation::Query,
                            description: Some("The user name for login".to_string()),
                            required: false,
                            schema: Some(NormalizedSchema {
                                types: vec![NormalizedSchemaType::String],
                                ..NormalizedSchema::default()
                            }),
                        }),
                        NormalizedParameter::Inline(NormalizedInlineParameter {
                            name: "password".to_string(),
                            location: NormalizedParameterLocation::Query,
                            description: Some("The password for login in clear text".to_string()),
                            required: false,
                            schema: Some(NormalizedSchema {
                                types: vec![NormalizedSchemaType::String],
                                ..NormalizedSchema::default()
                            }),
                        }),
                    ],
                    request_body: None,
                },
            ],
        }
    }

    fn newline() -> &'static str {
        if cfg!(windows) { "\r\n" } else { "\n" }
    }

    #[test]
    fn generates_one_request_per_file_outputs() {
        let result = generate_http_files(&GeneratorSettings::default(), &sample_document());

        assert_eq!(result.files.len(), 3);
        assert_eq!(result.files[0].filename, "PutUpdatePet.http");
        assert!(result.files[0].content.starts_with(&format!(
            "@baseUrl = /api/v3{nl}@contentType = application/json{nl}{nl}",
            nl = newline()
        )));
        assert!(result.files[0].content.contains("PUT {{baseUrl}}/pet"));
        assert!(result.files[0].content.contains("\"id\": 0"));
        assert!(result.files[0].content.contains("\"name\": \"example\""));
        assert!(
            result.files[0]
                .content
                .contains("\"category\": {\"property\": \"value\"}")
        );
    }

    #[test]
    fn generates_one_file_with_prefixed_parameter_names() {
        let settings = GeneratorSettings {
            output_type: OutputType::OneFile,
            ..GeneratorSettings::default()
        };

        let result = generate_http_files(&settings, &sample_document());

        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].filename, "Requests.http");
        assert!(result.files[0].content.contains("@GetPetById_petId = 0"));
        assert!(
            result.files[0]
                .content
                .contains("GET {{baseUrl}}/pet/{{GetPetById_petId}}")
        );
        assert!(
            result.files[0]
                .content
                .contains("@GetLoginUser_username = str")
        );
        assert!(result.files[0].content.contains(
            "GET {{baseUrl}}/user/login?username={{GetLoginUser_username}}&password={{GetLoginUser_password}}"
        ));
    }

    #[test]
    fn generates_one_file_per_tag() {
        let settings = GeneratorSettings {
            output_type: OutputType::OneFilePerTag,
            ..GeneratorSettings::default()
        };

        let result = generate_http_files(&settings, &sample_document());

        assert_eq!(result.files.len(), 2);
        assert_eq!(result.files[0].filename, "Pet.http");
        assert_eq!(result.files[1].filename, "User.http");
        assert!(result.files[0].content.contains("PUT {{baseUrl}}/pet"));
        assert!(
            result.files[0]
                .content
                .contains("GET {{baseUrl}}/pet/{{GetPetById_petId}}")
        );
        assert!(result.files[1]
            .content
            .contains("GET {{baseUrl}}/user/login?username={{GetLoginUser_username}}&password={{GetLoginUser_password}}"));
    }

    #[test]
    fn preserves_skip_headers_and_intellij_contract() {
        let settings = GeneratorSettings {
            output_type: OutputType::OneFile,
            skip_headers: true,
            generate_intellij_tests: true,
            authorization_header_from_environment_variable: true,
            custom_headers: vec!["X-API-Key: test123".to_string()],
            ..GeneratorSettings::default()
        };

        let result = generate_http_files(&settings, &sample_document());
        let content = &result.files[0].content;

        assert!(!content.contains("@baseUrl = "));
        assert!(!content.contains("@contentType = "));
        assert!(content.contains("Content-Type: {{contentType}}"));
        assert!(content.contains("Authorization: {{authorization}}"));
        assert!(content.contains("X-API-Key: test123"));
        assert!(content.contains("> {%"));
        assert!(content.contains("response.status === 200"));
    }

    #[test]
    fn preserves_trailing_blank_lines_in_multiline_descriptions() {
        let settings = GeneratorSettings {
            output_type: OutputType::OneFile,
            ..GeneratorSettings::default()
        };
        let document = NormalizedOpenApiDocument {
            specification_version: NormalizedSpecificationVersion::OpenApi30,
            servers: vec![NormalizedServer {
                url: "/api/v3".to_string(),
            }],
            operations: vec![NormalizedOperation {
                path: "/pets".to_string(),
                method: NormalizedHttpMethod::Get,
                operation_id: Some("getPets".to_string()),
                summary: None,
                description: Some("First paragraph\n\nSecond paragraph\n".to_string()),
                tags: Vec::new(),
                parameters: Vec::new(),
                request_body: None,
            }],
        };

        let result = generate_http_files(&settings, &document);

        assert!(result.files[0].content.contains(
            &format!(
                "### Description: {nl}###   First paragraph{nl}###   {nl}###   Second paragraph{nl}###   {nl}",
                nl = newline()
            )
        ));
    }
}
