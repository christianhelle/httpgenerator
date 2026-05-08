use serde_json::{Map, Value};

use crate::{NormalizedHttpMethod, NormalizedOperation};

use super::super::OpenApiNormalizationError;
use super::parameters::{get_parameter_values, normalize_parameters};
use super::request_body::normalize_request_body;

pub(super) fn normalize_operations(
    root: &Value,
) -> Result<Vec<NormalizedOperation>, OpenApiNormalizationError> {
    let Some(paths) = root.get("paths") else {
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
                root,
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
    root: &Value,
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
        parameters: normalize_parameters(root, path, method, path_parameters, operation)?,
        request_body: normalize_request_body(root, path, method, operation)?,
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
