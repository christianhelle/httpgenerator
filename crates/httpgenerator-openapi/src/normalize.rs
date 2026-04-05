use httpgenerator_core::{
    NormalizedHttpMethod, NormalizedInlineParameter, NormalizedInlineRequestBody,
    NormalizedMediaType, NormalizedOpenApiDocument, NormalizedOperation, NormalizedParameter,
    NormalizedParameterLocation, NormalizedRequestBody, NormalizedSchema, NormalizedSchemaProperty,
    NormalizedSchemaType, NormalizedServer, NormalizedSpecificationVersion,
};
use serde_json::{Map, Value};

use crate::{
    LoadedOpenApiDocument, OpenApiDocumentNormalizationError, OpenApiNormalizationError,
    load_document,
};

pub fn load_and_normalize_document(
    input: &str,
) -> Result<NormalizedOpenApiDocument, OpenApiDocumentNormalizationError> {
    let document = load_document(input).map_err(OpenApiDocumentNormalizationError::Load)?;
    normalize_loaded_document(&document).map_err(OpenApiDocumentNormalizationError::Normalize)
}

pub fn normalize_loaded_document(
    document: &LoadedOpenApiDocument,
) -> Result<NormalizedOpenApiDocument, OpenApiNormalizationError> {
    Ok(NormalizedOpenApiDocument {
        specification_version: normalize_specification_version(document),
        servers: normalize_servers(document.raw().value())?,
        operations: normalize_operations(document.raw().value())?,
    })
}

fn normalize_specification_version(
    document: &LoadedOpenApiDocument,
) -> NormalizedSpecificationVersion {
    match document.specification_version() {
        crate::OpenApiSpecificationVersion::Swagger2 => NormalizedSpecificationVersion::Swagger2,
        crate::OpenApiSpecificationVersion::OpenApi30 => NormalizedSpecificationVersion::OpenApi30,
        crate::OpenApiSpecificationVersion::OpenApi31 => NormalizedSpecificationVersion::OpenApi31,
    }
}

fn normalize_servers(value: &Value) -> Result<Vec<NormalizedServer>, OpenApiNormalizationError> {
    let Some(servers) = value.get("servers") else {
        return Ok(Vec::new());
    };

    let Some(servers) = servers.as_array() else {
        return Err(OpenApiNormalizationError::InvalidStructure {
            path: "servers".to_string(),
            context: "expected an array".to_string(),
        });
    };

    let mut normalized = Vec::with_capacity(servers.len());
    for (index, server) in servers.iter().enumerate() {
        let Some(server) = server.as_object() else {
            return Err(OpenApiNormalizationError::InvalidStructure {
                path: format!("servers[{index}]"),
                context: "expected an object".to_string(),
            });
        };

        if let Some(url) = server.get("url").and_then(Value::as_str) {
            normalized.push(NormalizedServer {
                url: url.to_string(),
            });
        }
    }

    Ok(normalized)
}

fn normalize_operations(
    value: &Value,
) -> Result<Vec<NormalizedOperation>, OpenApiNormalizationError> {
    let Some(paths) = value.get("paths") else {
        return Ok(Vec::new());
    };

    let Some(paths) = paths.as_object() else {
        return Err(OpenApiNormalizationError::InvalidStructure {
            path: "paths".to_string(),
            context: "expected an object".to_string(),
        });
    };

    let mut operations = Vec::new();
    for (path, path_item_value) in paths {
        let Some(path_item) = path_item_value.as_object() else {
            return Err(OpenApiNormalizationError::InvalidStructure {
                path: path.clone(),
                context: "expected a path item object".to_string(),
            });
        };

        if let Some(reference) = path_item.get("$ref").and_then(Value::as_str) {
            return Err(OpenApiNormalizationError::UnsupportedPathItemReference {
                path: path.clone(),
                reference: reference.to_string(),
            });
        }

        let path_parameters = get_parameter_values(path_item, path, "parameters")?;

        for method in supported_methods() {
            let Some(operation_value) = path_item.get(method.as_str()) else {
                continue;
            };

            let Some(operation) = operation_value.as_object() else {
                return Err(OpenApiNormalizationError::InvalidStructure {
                    path: format!("paths.{path}.{}", method.as_str()),
                    context: "expected an operation object".to_string(),
                });
            };

            operations.push(normalize_operation(
                path,
                method,
                &path_parameters,
                operation,
            )?);
        }
    }

    Ok(operations)
}

fn normalize_operation(
    path: &str,
    method: NormalizedHttpMethod,
    path_parameters: &[&Value],
    operation: &Map<String, Value>,
) -> Result<NormalizedOperation, OpenApiNormalizationError> {
    Ok(NormalizedOperation {
        path: path.to_string(),
        method,
        operation_id: operation
            .get("operationId")
            .and_then(Value::as_str)
            .map(str::to_string),
        summary: operation
            .get("summary")
            .and_then(Value::as_str)
            .map(str::to_string),
        description: operation
            .get("description")
            .and_then(Value::as_str)
            .map(str::to_string),
        tags: normalize_tags(operation)?,
        parameters: normalize_parameters(path, method, path_parameters, operation)?,
        request_body: normalize_request_body(path, method, operation.get("requestBody"))?,
    })
}

fn normalize_tags(
    operation: &Map<String, Value>,
) -> Result<Vec<String>, OpenApiNormalizationError> {
    let Some(tags) = operation.get("tags") else {
        return Ok(Vec::new());
    };

    let Some(tags) = tags.as_array() else {
        return Err(OpenApiNormalizationError::InvalidStructure {
            path: "operation.tags".to_string(),
            context: "expected an array".to_string(),
        });
    };

    Ok(tags
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect())
}

fn normalize_parameters(
    path: &str,
    method: NormalizedHttpMethod,
    path_parameters: &[&Value],
    operation: &Map<String, Value>,
) -> Result<Vec<NormalizedParameter>, OpenApiNormalizationError> {
    let operation_parameters = get_parameter_values(operation, path, "parameters")?;
    let mut merged = Vec::new();

    for parameter in path_parameters
        .iter()
        .copied()
        .chain(operation_parameters.iter().copied())
    {
        let normalized = normalize_parameter(path, method, parameter)?;
        let parameter_key = normalized.inline_key();

        if let Some(parameter_key) = parameter_key {
            if let Some(index) = merged.iter().position(|existing: &NormalizedParameter| {
                existing.inline_key() == Some(parameter_key)
            }) {
                merged[index] = normalized;
                continue;
            }
        }

        merged.push(normalized);
    }

    Ok(merged)
}

fn get_parameter_values<'a>(
    object: &'a Map<String, Value>,
    path: &str,
    field_name: &str,
) -> Result<Vec<&'a Value>, OpenApiNormalizationError> {
    let Some(parameters) = object.get(field_name) else {
        return Ok(Vec::new());
    };

    let Some(parameters) = parameters.as_array() else {
        return Err(OpenApiNormalizationError::InvalidStructure {
            path: format!("{path}.{field_name}"),
            context: "expected an array".to_string(),
        });
    };

    Ok(parameters.iter().collect())
}

fn normalize_parameter(
    path: &str,
    method: NormalizedHttpMethod,
    value: &Value,
) -> Result<NormalizedParameter, OpenApiNormalizationError> {
    if let Some(reference) = value.get("$ref").and_then(Value::as_str) {
        return Err(OpenApiNormalizationError::UnsupportedParameterReference {
            path: path.to_string(),
            method,
            reference: reference.to_string(),
        });
    }

    let Some(parameter) = value.as_object() else {
        return Err(OpenApiNormalizationError::InvalidStructure {
            path: format!("{path}.{}", method.as_str()),
            context: "expected a parameter object".to_string(),
        });
    };

    let name = parameter
        .get("name")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_default();

    let location = parameter
        .get("in")
        .and_then(Value::as_str)
        .and_then(normalize_parameter_location)
        .ok_or_else(|| OpenApiNormalizationError::InvalidStructure {
            path: format!("{path}.{}.parameters", method.as_str()),
            context: "parameter is missing a supported location".to_string(),
        })?;

    Ok(NormalizedParameter::Inline(NormalizedInlineParameter {
        name,
        location,
        description: parameter
            .get("description")
            .and_then(Value::as_str)
            .map(str::to_string),
        required: parameter
            .get("required")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        schema: parameter.get("schema").map(normalize_schema),
    }))
}

fn normalize_request_body(
    path: &str,
    method: NormalizedHttpMethod,
    value: Option<&Value>,
) -> Result<Option<NormalizedRequestBody>, OpenApiNormalizationError> {
    let Some(request_body) = value else {
        return Ok(None);
    };

    if let Some(reference) = request_body.get("$ref").and_then(Value::as_str) {
        return Err(OpenApiNormalizationError::UnsupportedRequestBodyReference {
            path: path.to_string(),
            method,
            reference: reference.to_string(),
        });
    }

    let Some(request_body) = request_body.as_object() else {
        return Err(OpenApiNormalizationError::InvalidStructure {
            path: format!("{path}.{}.requestBody", method.as_str()),
            context: "expected a requestBody object".to_string(),
        });
    };

    let content = match request_body.get("content") {
        Some(content) => {
            let Some(content) = content.as_object() else {
                return Err(OpenApiNormalizationError::InvalidStructure {
                    path: format!("{path}.{}.requestBody.content", method.as_str()),
                    context: "expected a content object".to_string(),
                });
            };

            content
                .iter()
                .map(|(content_type, media_type)| {
                    let Some(media_type) = media_type.as_object() else {
                        return Err(OpenApiNormalizationError::InvalidStructure {
                            path: format!(
                                "{path}.{}.requestBody.content.{content_type}",
                                method.as_str()
                            ),
                            context: "expected a media type object".to_string(),
                        });
                    };

                    Ok(NormalizedMediaType {
                        content_type: content_type.clone(),
                        schema: media_type.get("schema").map(normalize_schema),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?
        }
        None => Vec::new(),
    };

    Ok(Some(NormalizedRequestBody::Inline(
        NormalizedInlineRequestBody {
            description: request_body
                .get("description")
                .and_then(Value::as_str)
                .map(str::to_string),
            required: request_body
                .get("required")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            content,
        },
    )))
}

fn normalize_schema(value: &Value) -> NormalizedSchema {
    match value {
        Value::Object(schema) => {
            let reference = schema
                .get("$ref")
                .and_then(Value::as_str)
                .map(str::to_string);
            let types = normalize_schema_types(schema.get("type"));
            let properties = schema
                .get("properties")
                .and_then(Value::as_object)
                .map(|properties| {
                    properties
                        .iter()
                        .map(|(name, property)| NormalizedSchemaProperty {
                            name: name.clone(),
                            schema: normalize_schema(property),
                        })
                        .collect()
                })
                .unwrap_or_default();
            let items = schema
                .get("items")
                .map(|items| Box::new(normalize_schema(items)));
            let all_of = normalize_schema_array(schema.get("allOf"));
            let one_of = normalize_schema_array(schema.get("oneOf"));
            let any_of = normalize_schema_array(schema.get("anyOf"));

            NormalizedSchema {
                reference,
                types,
                properties,
                items,
                all_of,
                one_of,
                any_of,
            }
        }
        Value::Bool(value) => NormalizedSchema {
            types: vec![NormalizedSchemaType::Other(format!(
                "boolean-schema:{value}"
            ))],
            ..NormalizedSchema::default()
        },
        _ => NormalizedSchema::default(),
    }
}

fn normalize_schema_array(value: Option<&Value>) -> Vec<NormalizedSchema> {
    value
        .and_then(Value::as_array)
        .map(|schemas| schemas.iter().map(normalize_schema).collect())
        .unwrap_or_default()
}

fn normalize_schema_types(value: Option<&Value>) -> Vec<NormalizedSchemaType> {
    match value {
        Some(Value::String(schema_type)) => vec![normalize_schema_type(schema_type)],
        Some(Value::Array(types)) => types
            .iter()
            .filter_map(Value::as_str)
            .map(normalize_schema_type)
            .collect(),
        _ => Vec::new(),
    }
}

fn normalize_schema_type(value: &str) -> NormalizedSchemaType {
    match value {
        "string" => NormalizedSchemaType::String,
        "integer" => NormalizedSchemaType::Integer,
        "number" => NormalizedSchemaType::Number,
        "boolean" => NormalizedSchemaType::Boolean,
        "object" => NormalizedSchemaType::Object,
        "array" => NormalizedSchemaType::Array,
        "null" => NormalizedSchemaType::Null,
        other => NormalizedSchemaType::Other(other.to_string()),
    }
}

fn normalize_parameter_location(value: &str) -> Option<NormalizedParameterLocation> {
    match value {
        "path" => Some(NormalizedParameterLocation::Path),
        "query" => Some(NormalizedParameterLocation::Query),
        "header" => Some(NormalizedParameterLocation::Header),
        "cookie" => Some(NormalizedParameterLocation::Cookie),
        _ => None,
    }
}

fn supported_methods() -> [NormalizedHttpMethod; 8] {
    [
        NormalizedHttpMethod::Get,
        NormalizedHttpMethod::Put,
        NormalizedHttpMethod::Post,
        NormalizedHttpMethod::Delete,
        NormalizedHttpMethod::Options,
        NormalizedHttpMethod::Head,
        NormalizedHttpMethod::Patch,
        NormalizedHttpMethod::Trace,
    ]
}

trait InlineParameterKey {
    fn inline_key(&self) -> Option<(&str, NormalizedParameterLocation)>;
}

impl InlineParameterKey for NormalizedParameter {
    fn inline_key(&self) -> Option<(&str, NormalizedParameterLocation)> {
        match self {
            NormalizedParameter::Inline(parameter) => Some((&parameter.name, parameter.location)),
            NormalizedParameter::Reference { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use httpgenerator_core::{
        NormalizedHttpMethod, NormalizedParameter, NormalizedParameterLocation,
        NormalizedRequestBody, NormalizedSpecificationVersion,
    };

    use crate::{OpenApiSource, decode_raw_document, load_document_from_raw};

    use super::{load_and_normalize_document, normalize_loaded_document};

    #[test]
    fn normalizes_petstore_v30_fixture_into_generator_facing_operations() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.0/petstore.json")),
            include_str!("../../../test/OpenAPI/v3.0/petstore.json"),
        )
        .unwrap();
        let loaded = load_document_from_raw(raw).unwrap();
        let normalized = normalize_loaded_document(&loaded).unwrap();

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
                assert_eq!(
                    application_json
                        .schema
                        .as_ref()
                        .and_then(|schema| schema.reference.as_deref()),
                    Some("#/components/schemas/Pet")
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
                operation.path == "/pet/findByStatus"
                    && operation.method == NormalizedHttpMethod::Get
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
    fn webhook_only_v31_documents_normalize_without_operations() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.1/webhook-example.json")),
            include_str!("../../../test/OpenAPI/v3.1/webhook-example.json"),
        )
        .unwrap();
        let loaded = load_document_from_raw(raw).unwrap();
        let normalized = normalize_loaded_document(&loaded).unwrap();

        assert_eq!(
            normalized.specification_version,
            NormalizedSpecificationVersion::OpenApi31
        );
        assert!(normalized.servers.is_empty());
        assert!(normalized.operations.is_empty());
    }

    #[test]
    fn operation_level_parameters_override_path_level_parameters() {
        let raw = decode_raw_document(
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
        )
        .unwrap();
        let loaded = load_document_from_raw(raw).unwrap();
        let normalized = normalize_loaded_document(&loaded).unwrap();

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
        let raw = decode_raw_document(
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
        )
        .unwrap();
        let loaded = load_document_from_raw(raw).unwrap();
        let error = normalize_loaded_document(&loaded).unwrap_err();

        assert_eq!(
            error,
            crate::OpenApiNormalizationError::UnsupportedRequestBodyReference {
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
                .join("test")
                .join("OpenAPI")
                .join("v3.0")
                .join("petstore.json")
                .to_str()
                .unwrap(),
        )
        .unwrap();

        assert_eq!(
            normalized.specification_version,
            NormalizedSpecificationVersion::OpenApi30
        );
    }
}
