use crate::{
    GeneratorSettings, NormalizedHttpMethod, NormalizedInlineParameter,
    NormalizedInlineRequestBody, NormalizedMediaType, NormalizedOpenApiDocument,
    NormalizedOperation, NormalizedParameter, NormalizedParameterLocation, NormalizedRequestBody,
    NormalizedSchema, NormalizedSchemaProperty, NormalizedSchemaType, NormalizedServer,
    NormalizedSpecificationVersion, OutputType,
};

use super::generate_http_files;

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
                request_body: Some(NormalizedRequestBody::Inline(NormalizedInlineRequestBody {
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
                })),
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
    assert!(result.files[1].content.contains(
        "GET {{baseUrl}}/user/login?username={{GetLoginUser_username}}&password={{GetLoginUser_password}}"
    ));
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

    assert!(result.files[0].content.contains(&format!(
        "### Description: {nl}###   First paragraph{nl}###   {nl}###   Second paragraph{nl}###   {nl}",
        nl = newline()
    )));
}
