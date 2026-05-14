use serde_json::{Map, Value};

use crate::{NormalizedHttpMethod, NormalizedOperation};

use super::super::OpenApiNormalizationError;
use super::parameters::{get_parameter_values, normalize_parameters};
use super::request_body::normalize_request_body;

pub(super) fn normalize_operations(
    root: &Value,
) -> Result<Vec<NormalizedOperation>, OpenApiNormalizationError> {
    let mut operations = Vec::new();
    normalize_operation_group(root, root.get("paths"), "paths", None, &mut operations)?;
    normalize_operation_group(
        root,
        root.get("webhooks"),
        "webhooks",
        Some("Webhooks"),
        &mut operations,
    )?;

    Ok(operations)
}

fn normalize_operation_group(
    root: &Value,
    path_items: Option<&Value>,
    collection_name: &str,
    fallback_tag: Option<&str>,
    operations: &mut Vec<NormalizedOperation>,
) -> Result<(), OpenApiNormalizationError> {
    let Some(path_items) = path_items else {
        return Ok(());
    };

    let Some(path_items) = path_items.as_object() else {
        return Err(OpenApiNormalizationError::InvalidStructure {
            path: collection_name.to_string(),
            context: "expected an object".to_string(),
        });
    };

    for (raw_path, path_item_value) in path_items {
        let Some(path_item) = path_item_value.as_object() else {
            return Err(OpenApiNormalizationError::InvalidStructure {
                path: format!("{collection_name}.{raw_path}"),
                context: "expected a path item object".to_string(),
            });
        };

        if let Some(reference) = path_item.get("$ref").and_then(Value::as_str) {
            return Err(OpenApiNormalizationError::UnsupportedPathItemReference {
                path: format!("{collection_name}.{raw_path}"),
                reference: reference.to_string(),
            });
        }

        let normalized_path = normalize_operation_path(collection_name, raw_path);
        let path_parameters = get_parameter_values(path_item, &normalized_path, "parameters")?;

        for method in supported_methods() {
            let Some(operation_value) = path_item.get(method.as_str()) else {
                continue;
            };

            let Some(operation) = operation_value.as_object() else {
                return Err(OpenApiNormalizationError::InvalidStructure {
                    path: format!("{collection_name}.{raw_path}.{}", method.as_str()),
                    context: "expected an operation object".to_string(),
                });
            };

            operations.push(normalize_operation(
                root,
                &normalized_path,
                method,
                &path_parameters,
                operation,
                fallback_tag,
                if collection_name == "webhooks" {
                    Some(raw_path)
                } else {
                    None
                },
            )?);
        }
    }

    Ok(())
}

fn normalize_operation(
    root: &Value,
    path: &str,
    method: NormalizedHttpMethod,
    path_parameters: &[&Value],
    operation: &Map<String, Value>,
    fallback_tag: Option<&str>,
    fallback_operation_id: Option<&str>,
) -> Result<NormalizedOperation, OpenApiNormalizationError> {
    Ok(NormalizedOperation {
        path: path.to_string(),
        method,
        operation_id: operation
            .get("operationId")
            .and_then(Value::as_str)
            .map(str::to_string)
            .or_else(|| fallback_operation_id.map(str::to_string)),
        summary: operation
            .get("summary")
            .and_then(Value::as_str)
            .map(str::to_string),
        description: operation
            .get("description")
            .and_then(Value::as_str)
            .map(str::to_string),
        tags: normalize_tags(operation, fallback_tag)?,
        parameters: normalize_parameters(root, path, method, path_parameters, operation)?,
        request_body: normalize_request_body(root, path, method, operation)?,
    })
}

fn normalize_tags(
    operation: &Map<String, Value>,
    fallback_tag: Option<&str>,
) -> Result<Vec<String>, OpenApiNormalizationError> {
    let Some(tags) = operation.get("tags") else {
        return Ok(fallback_tags(fallback_tag));
    };

    let Some(tags) = tags.as_array() else {
        return Err(OpenApiNormalizationError::InvalidStructure {
            path: "operation.tags".to_string(),
            context: "expected an array".to_string(),
        });
    };

    let normalized_tags = tags
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect::<Vec<_>>();

    if normalized_tags.is_empty() {
        return Ok(fallback_tags(fallback_tag));
    }

    Ok(normalized_tags)
}

fn fallback_tags(fallback_tag: Option<&str>) -> Vec<String> {
    fallback_tag
        .map(|tag| vec![tag.to_string()])
        .unwrap_or_default()
}

fn normalize_operation_path(collection_name: &str, raw_path: &str) -> String {
    if collection_name == "webhooks" && !raw_path.starts_with('/') {
        return format!("/{raw_path}");
    }

    raw_path.to_string()
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
