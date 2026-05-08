use serde_json::{Map, Value};

use crate::{
    NormalizedHttpMethod, NormalizedInlineRequestBody, NormalizedMediaType, NormalizedRequestBody,
};

use super::super::OpenApiNormalizationError;
use super::schema::normalize_schema;

pub(super) fn normalize_request_body(
    root: &Value,
    path: &str,
    method: NormalizedHttpMethod,
    operation: &Map<String, Value>,
) -> Result<Option<NormalizedRequestBody>, OpenApiNormalizationError> {
    if let Some(request_body) = operation.get("requestBody") {
        return normalize_openapi3_request_body(root, path, method, request_body);
    }

    normalize_swagger2_request_body(root, path, method, operation)
}

fn normalize_openapi3_request_body(
    root: &Value,
    path: &str,
    method: NormalizedHttpMethod,
    request_body: &Value,
) -> Result<Option<NormalizedRequestBody>, OpenApiNormalizationError> {
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
            content: normalize_request_body_content(root, path, method, request_body)?,
        },
    )))
}

fn normalize_swagger2_request_body(
    root: &Value,
    path: &str,
    method: NormalizedHttpMethod,
    operation: &Map<String, Value>,
) -> Result<Option<NormalizedRequestBody>, OpenApiNormalizationError> {
    let Some(parameters) = operation.get("parameters").and_then(Value::as_array) else {
        return Ok(None);
    };

    let Some(body_parameter) = parameters.iter().find(|parameter| {
        parameter
            .get("in")
            .and_then(Value::as_str)
            .is_some_and(|location| location == "body")
    }) else {
        return Ok(None);
    };

    if let Some(reference) = body_parameter.get("$ref").and_then(Value::as_str) {
        return Err(OpenApiNormalizationError::UnsupportedRequestBodyReference {
            path: path.to_string(),
            method,
            reference: reference.to_string(),
        });
    }

    let Some(body_parameter) = body_parameter.as_object() else {
        return Err(OpenApiNormalizationError::InvalidStructure {
            path: format!("{path}.{}.parameters.body", method.as_str()),
            context: "expected a body parameter object".to_string(),
        });
    };

    Ok(Some(NormalizedRequestBody::Inline(
        NormalizedInlineRequestBody {
            description: body_parameter
                .get("description")
                .and_then(Value::as_str)
                .map(str::to_string),
            required: body_parameter
                .get("required")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            content: normalize_swagger2_request_body_content(root, body_parameter, operation),
        },
    )))
}

fn normalize_request_body_content(
    root: &Value,
    path: &str,
    method: NormalizedHttpMethod,
    request_body: &Map<String, Value>,
) -> Result<Vec<NormalizedMediaType>, OpenApiNormalizationError> {
    match request_body.get("content") {
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
                        schema: media_type
                            .get("schema")
                            .map(|schema| normalize_schema(root, schema)),
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        }
        None => Ok(Vec::new()),
    }
}

fn normalize_swagger2_request_body_content(
    root: &Value,
    body_parameter: &Map<String, Value>,
    operation: &Map<String, Value>,
) -> Vec<NormalizedMediaType> {
    let content_types = operation
        .get("consumes")
        .or_else(|| root.get("consumes"))
        .and_then(Value::as_array)
        .map(|content_types| {
            content_types
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .filter(|content_types| !content_types.is_empty())
        .unwrap_or_else(|| vec!["application/json".to_string()]);

    content_types
        .into_iter()
        .map(|content_type| NormalizedMediaType {
            content_type,
            schema: body_parameter
                .get("schema")
                .map(|schema| normalize_schema(root, schema)),
        })
        .collect()
}
