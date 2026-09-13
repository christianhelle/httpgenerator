use std::path::PathBuf;

use crate::{
    NormalizedHttpMethod, NormalizedParameter, NormalizedParameterLocation, NormalizedRequestBody,
    NormalizedSchemaType, NormalizedServer, NormalizedSpecificationVersion,
};

use crate::openapi::{
    FetchError, OpenApiNormalizationError, OpenApiReader, OpenApiSource, ReadResult,
    TypedParseOptions,
};

use super::{load_and_normalize_document, normalize_document};

/// Reads `content` as if it was loaded from `source`.
fn read_inline(source: OpenApiSource, content: &'static str) -> ReadResult {
    let served = source.clone();
    OpenApiReader::new()
        .with_loader(move |requested: &OpenApiSource| {
            if requested == &served {
                Ok(content.to_string())
            } else {
                Err(FetchError::FileRead {
                    path: requested.to_string().into(),
                    reason: "not found".to_string(),
                })
            }
        })
        .read_source(source)
        .unwrap()
}

#[test]
fn normalizes_petstore_v30_fixture_into_generator_facing_operations() {
    let document = read_inline(
        OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.0/petstore.json")),
        include_str!("../../../../../../test/OpenAPI/v3.0/petstore.json"),
    );
    let normalized = normalize_document(&document).unwrap();

    assert_eq!(
        normalized.specification_version,
        NormalizedSpecificationVersion::OpenApi30
    );
    assert_eq!(normalized.servers[0].url, "/api/v3");
    assert_eq!(normalized.operations.len(), 19);

    let add_pet = normalized
        .operations
        .iter()
        .find(|operation| {
            operation.path == "/pet" && operation.method == NormalizedHttpMethod::Post
        })
        .unwrap();
    assert_eq!(add_pet.tags.first().map(String::as_str), Some("pet"));

    match add_pet.request_body.as_ref().unwrap() {
        NormalizedRequestBody::Inline(request_body) => {
            let application_json = request_body
                .content
                .iter()
                .find(|content| content.content_type == "application/json")
                .unwrap();
            let schema = application_json.schema.as_ref().unwrap();
            assert_eq!(
                schema.reference.as_deref(),
                Some("#/components/schemas/Pet")
            );
            assert_eq!(
                schema
                    .properties
                    .iter()
                    .take(3)
                    .map(|property| property.name.as_str())
                    .collect::<Vec<_>>(),
                vec!["id", "name", "category"]
            );
            let category = schema
                .properties
                .iter()
                .find(|property| property.name == "category")
                .unwrap();
            assert!(
                category
                    .schema
                    .types
                    .contains(&NormalizedSchemaType::Object)
            );
        }
        NormalizedRequestBody::Reference { .. } => {
            panic!("expected addPet to use an inline request body")
        }
    }

    let find_by_status = normalized
        .operations
        .iter()
        .find(|operation| {
            operation.path == "/pet/findByStatus" && operation.method == NormalizedHttpMethod::Get
        })
        .unwrap();
    assert!(find_by_status.parameters.iter().any(|parameter| {
        matches!(
            parameter,
            NormalizedParameter::Inline(parameter)
                if parameter.name == "status"
                    && parameter.location == NormalizedParameterLocation::Query
        )
    }));
}

#[test]
fn normalizes_petstore_v20_fixture_into_generator_facing_operations() {
    let document = read_inline(
        OpenApiSource::Path(PathBuf::from("test/OpenAPI/v2.0/petstore.json")),
        include_str!("../../../../../../test/OpenAPI/v2.0/petstore.json"),
    );
    let normalized = normalize_document(&document).unwrap();

    assert_eq!(
        normalized.specification_version,
        NormalizedSpecificationVersion::Swagger2
    );
    assert_eq!(normalized.servers[0].url, "https://petstore.swagger.io/v2");
    assert_eq!(normalized.operations.len(), 20);
    assert!(normalized.operations.iter().any(|operation| {
        operation.path == "/user/createWithArray" && operation.method == NormalizedHttpMethod::Post
    }));

    let add_pet = normalized
        .operations
        .iter()
        .find(|operation| {
            operation.path == "/pet" && operation.method == NormalizedHttpMethod::Post
        })
        .unwrap();
    match add_pet.request_body.as_ref().unwrap() {
        NormalizedRequestBody::Inline(request_body) => {
            assert_eq!(
                request_body
                    .content
                    .iter()
                    .map(|content| content.content_type.as_str())
                    .collect::<Vec<_>>(),
                vec!["application/json", "application/xml"]
            );
            let schema = request_body.content[0].schema.as_ref().unwrap();
            assert_eq!(schema.reference.as_deref(), Some("#/definitions/Pet"));
            assert_eq!(
                schema
                    .properties
                    .iter()
                    .take(3)
                    .map(|property| property.name.as_str())
                    .collect::<Vec<_>>(),
                vec!["id", "category", "name"]
            );
        }
        NormalizedRequestBody::Reference { .. } => {
            panic!("expected addPet to use an inline Swagger 2 request body")
        }
    }

    let find_by_status = normalized
        .operations
        .iter()
        .find(|operation| {
            operation.path == "/pet/findByStatus" && operation.method == NormalizedHttpMethod::Get
        })
        .unwrap();
    assert!(find_by_status.parameters.iter().any(|parameter| {
        matches!(
            parameter,
            NormalizedParameter::Inline(parameter)
                if parameter.name == "status"
                    && parameter.location == NormalizedParameterLocation::Query
                    && parameter
                        .schema
                        .as_ref()
                        .is_some_and(|schema| schema.types.contains(&NormalizedSchemaType::Array))
        )
    }));

    let upload_image = normalized
        .operations
        .iter()
        .find(|operation| {
            operation.path == "/pet/{petId}/uploadImage"
                && operation.method == NormalizedHttpMethod::Post
        })
        .unwrap();
    assert_eq!(upload_image.parameters.len(), 1);
    assert!(matches!(
        &upload_image.parameters[0],
        NormalizedParameter::Inline(parameter)
            if parameter.name == "petId"
                && parameter.location == NormalizedParameterLocation::Path
    ));

    let update_pet_with_form = normalized
        .operations
        .iter()
        .find(|operation| {
            operation.path == "/pet/{petId}" && operation.method == NormalizedHttpMethod::Post
        })
        .unwrap();
    assert_eq!(update_pet_with_form.parameters.len(), 1);
    assert!(update_pet_with_form.request_body.is_none());
}

#[test]
fn swagger2_local_documents_without_host_or_base_path_use_parent_directory_server() {
    let input = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("test")
        .join("OpenAPI")
        .join("v2.0")
        .join("api-with-examples.json");
    let normalized =
        load_and_normalize_document(input.to_str().unwrap(), TypedParseOptions::default()).unwrap();
    let mut expected_directory = std::fs::canonicalize(&input)
        .unwrap()
        .parent()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    if let Some(stripped) = expected_directory.strip_prefix(r"\\?\") {
        expected_directory = stripped.to_string();
    }

    expected_directory = expected_directory.replace('\\', "/");

    assert_eq!(
        normalized.servers,
        vec![NormalizedServer {
            url: format!("file://{expected_directory}"),
        }]
    );
}

#[test]
fn openapi30_local_documents_without_servers_do_not_use_parent_directory_server() {
    let input = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("test")
        .join("OpenAPI")
        .join("v3.0")
        .join("api-with-examples.json");
    let normalized =
        load_and_normalize_document(input.to_str().unwrap(), TypedParseOptions::default()).unwrap();

    assert!(normalized.servers.is_empty());
}

#[test]
fn webhook_only_v31_documents_normalize_into_webhook_operations() {
    let document = read_inline(
        OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.1/webhook-example.json")),
        include_str!("../../../../../../test/OpenAPI/v3.1/webhook-example.json"),
    );
    let normalized = normalize_document(&document).unwrap();

    assert_eq!(
        normalized.specification_version,
        NormalizedSpecificationVersion::OpenApi31
    );
    assert!(normalized.servers.is_empty());
    assert_eq!(normalized.operations.len(), 1);
    assert_eq!(normalized.operations[0].path, "/newPet");
    assert_eq!(normalized.operations[0].method, NormalizedHttpMethod::Post);
    assert_eq!(
        normalized.operations[0].operation_id.as_deref(),
        Some("newPet")
    );
    assert_eq!(normalized.operations[0].tags, vec!["Webhooks"]);
    assert!(matches!(
        normalized.operations[0].request_body,
        Some(NormalizedRequestBody::Inline(_))
    ));
}

#[test]
fn openapi31_documents_collect_paths_and_webhooks_together() {
    let document = read_inline(
        OpenApiSource::Path(PathBuf::from("inline.json")),
        r#"{
                "openapi": "3.1.0",
                "info": { "title": "Example", "version": "1.0.0" },
                "paths": {
                    "/pets": {
                        "get": {
                            "responses": {
                                "200": {
                                    "description": "ok"
                                }
                            }
                        }
                    }
                },
                "webhooks": {
                    "newPet": {
                        "post": {
                            "responses": {
                                "200": {
                                    "description": "ok"
                                }
                            }
                        }
                    }
                }
            }"#,
    );
    let normalized = normalize_document(&document).unwrap();

    assert_eq!(normalized.operations.len(), 2);
    assert!(normalized.operations.iter().any(
        |operation| operation.path == "/pets" && operation.method == NormalizedHttpMethod::Get
    ));
    assert!(normalized.operations.iter().any(|operation| {
        operation.path == "/newPet"
            && operation.method == NormalizedHttpMethod::Post
            && operation.tags == ["Webhooks"]
            && operation.operation_id.as_deref() == Some("newPet")
    }));
}

#[test]
fn webhook_path_item_refs_fail_explicitly_during_normalization() {
    let document = read_inline(
        OpenApiSource::Path(PathBuf::from("inline.json")),
        r##"{
                "openapi": "3.1.0",
                "info": { "title": "Example", "version": "1.0.0" },
                "webhooks": {
                    "newPet": {
                        "$ref": "#/components/pathItems/NewPet"
                    }
                }
            }"##,
    );
    let error = normalize_document(&document).unwrap_err();

    assert_eq!(
        error,
        OpenApiNormalizationError::UnsupportedPathItemReference {
            path: "webhooks.newPet".to_string(),
            reference: "#/components/pathItems/NewPet".to_string(),
        }
    );
}

#[test]
fn invalid_v31_documents_normalize_when_tolerated() {
    let normalized = load_and_normalize_document(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("..")
            .join("test")
            .join("OpenAPI")
            .join("v3.1")
            .join("non-oauth-scopes.json")
            .to_str()
            .unwrap(),
        TypedParseOptions {
            tolerate_invalid_openapi31: true,
        },
    )
    .unwrap();

    assert_eq!(
        normalized.specification_version,
        NormalizedSpecificationVersion::OpenApi31
    );
    assert_eq!(normalized.operations.len(), 1);
    assert_eq!(normalized.operations[0].path, "/users");
    assert_eq!(normalized.operations[0].method, NormalizedHttpMethod::Get);
}

#[test]
fn operation_level_parameters_override_path_level_parameters() {
    let document = read_inline(
        OpenApiSource::Path(PathBuf::from("inline.json")),
        r#"{
                "openapi": "3.0.2",
                "info": { "title": "Example", "version": "1.0.0" },
                "paths": {
                    "/pets": {
                        "parameters": [
                            {
                                "name": "status",
                                "in": "query",
                                "description": "path-level",
                                "schema": { "type": "string" }
                            }
                        ],
                        "get": {
                            "parameters": [
                                {
                                    "name": "status",
                                    "in": "query",
                                    "description": "operation-level",
                                    "schema": { "type": "string" }
                                }
                            ],
                            "responses": {
                                "200": {
                                    "description": "ok"
                                }
                            }
                        }
                    }
                }
            }"#,
    );
    let normalized = normalize_document(&document).unwrap();

    assert_eq!(normalized.operations.len(), 1);
    assert_eq!(normalized.operations[0].parameters.len(), 1);
    match &normalized.operations[0].parameters[0] {
        NormalizedParameter::Inline(parameter) => {
            assert_eq!(parameter.description.as_deref(), Some("operation-level"));
        }
        NormalizedParameter::Reference { .. } => panic!("expected an inline parameter"),
    }
}

#[test]
fn top_level_request_body_refs_fail_explicitly_during_normalization() {
    let document = read_inline(
        OpenApiSource::Path(PathBuf::from("inline.json")),
        r##"{
                "openapi": "3.0.2",
                "info": { "title": "Example", "version": "1.0.0" },
                "paths": {
                    "/pets": {
                        "post": {
                            "requestBody": {
                                "$ref": "#/components/requestBodies/PetBody"
                            },
                            "responses": {
                                "200": {
                                    "description": "ok"
                                }
                            }
                        }
                    }
                }
            }"##,
    );
    let error = normalize_document(&document).unwrap_err();

    assert_eq!(
        error,
        OpenApiNormalizationError::UnsupportedRequestBodyReference {
            path: "/pets".to_string(),
            method: NormalizedHttpMethod::Post,
            reference: "#/components/requestBodies/PetBody".to_string(),
        }
    );
}

#[test]
fn convenience_loader_normalizes_local_documents() {
    let normalized = load_and_normalize_document(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("..")
            .join("test")
            .join("OpenAPI")
            .join("v3.0")
            .join("petstore.json")
            .to_str()
            .unwrap(),
        TypedParseOptions::default(),
    )
    .unwrap();

    assert_eq!(
        normalized.specification_version,
        NormalizedSpecificationVersion::OpenApi30
    );
}

/// Reads the first of `files` through a loader that serves every file from memory.
fn read_files(files: &'static [(&'static str, &'static str)]) -> ReadResult {
    OpenApiReader::new()
        .with_loader(|requested: &OpenApiSource| {
            files
                .iter()
                .find(|(path, _)| requested == &OpenApiSource::Path(PathBuf::from(path)))
                .map(|(_, content)| content.to_string())
                .ok_or_else(|| FetchError::FileRead {
                    path: requested.to_string().into(),
                    reason: "not found".to_string(),
                })
        })
        .read_source(OpenApiSource::Path(PathBuf::from(files[0].0)))
        .unwrap()
}

const SPLIT_COMPONENT_REFERENCES: &str = r#"
openapi: 3.1.0
info:
  title: Split
  version: 1.0.0
paths:
  /pets:
    get:
      parameters:
        - $ref: 'components.yaml#/components/parameters/Limit'
      responses:
        '200':
          description: ok
    post:
      requestBody:
        $ref: 'components.yaml#/components/requestBodies/Pet'
      responses:
        '200':
          description: ok
  /owners:
    $ref: 'components.yaml#/components/pathItems/Owners'
"#;

#[test]
fn split_documents_resolve_parameter_request_body_and_path_item_components() {
    let document = read_files(&[
        ("/specs/main.yaml", SPLIT_COMPONENT_REFERENCES),
        (
            "/specs/components.yaml",
            r#"
components:
  parameters:
    Limit:
      name: limit
      in: query
      schema:
        type: integer
  requestBodies:
    Pet:
      required: true
      content:
        application/json:
          schema:
            type: object
  pathItems:
    Owners:
      get:
        operationId: listOwners
        responses:
          '200':
            description: ok
"#,
        ),
    ]);
    assert!(document.diagnostics.is_empty(), "{:?}", document.diagnostics);

    let normalized = normalize_document(&document).unwrap();

    let list_pets = normalized
        .operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == NormalizedHttpMethod::Get)
        .unwrap();
    assert!(matches!(
        list_pets.parameters.as_slice(),
        [NormalizedParameter::Inline(parameter)]
            if parameter.name == "limit" && parameter.location == NormalizedParameterLocation::Query
    ));

    let add_pet = normalized
        .operations
        .iter()
        .find(|operation| operation.path == "/pets" && operation.method == NormalizedHttpMethod::Post)
        .unwrap();
    assert!(matches!(
        &add_pet.request_body,
        Some(NormalizedRequestBody::Inline(body))
            if body.required && body.content[0].content_type == "application/json"
    ));

    assert!(normalized.operations.iter().any(|operation| {
        operation.path == "/owners" && operation.operation_id.as_deref() == Some("listOwners")
    }));
}

#[test]
fn unresolved_external_references_are_skipped_during_normalization() {
    let document = read_files(&[("/specs/main.yaml", SPLIT_COMPONENT_REFERENCES)]);
    assert_eq!(document.diagnostics.len(), 3, "{:?}", document.diagnostics);

    let normalized = normalize_document(&document).unwrap();

    assert_eq!(normalized.operations.len(), 2);
    assert!(normalized.operations.iter().all(|operation| operation.path == "/pets"));
    assert!(normalized.operations.iter().all(|operation| {
        operation.parameters.is_empty() && operation.request_body.is_none()
    }));
}
