use serde_json::{Map, Value};

use crate::{
    NormalizedHttpMethod, NormalizedInlineParameter, NormalizedParameter,
    NormalizedParameterLocation,
};

use super::super::OpenApiNormalizationError;
use super::schema::normalize_schema;

pub(super) fn normalize_parameters(
    root: &Value,
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
        let Some(normalized) = normalize_parameter(root, path, method, parameter)? else {
            continue;
        };
        let parameter_key = normalized.inline_key();

        if let Some(parameter_key) = parameter_key
            && let Some(index) = merged.iter().position(|existing: &NormalizedParameter| {
                existing.inline_key() == Some(parameter_key)
            })
        {
            merged[index] = normalized;
            continue;
        }

        merged.push(normalized);
    }

    Ok(merged)
}

pub(super) fn get_parameter_values<'a>(
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
    root: &Value,
    path: &str,
    method: NormalizedHttpMethod,
    value: &Value,
) -> Result<Option<NormalizedParameter>, OpenApiNormalizationError> {
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

    let location_name = parameter.get("in").and_then(Value::as_str).ok_or_else(|| {
        OpenApiNormalizationError::InvalidStructure {
            path: format!("{path}.{}.parameters", method.as_str()),
            context: "parameter is missing a location".to_string(),
        }
    })?;
    let Some(location) = normalize_parameter_location(location_name) else {
        return Ok(None);
    };

    let synthetic_schema = synthesize_swagger2_parameter_schema(parameter);

    Ok(Some(NormalizedParameter::Inline(
        NormalizedInlineParameter {
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
            schema: parameter
                .get("schema")
                .or(synthetic_schema.as_ref())
                .map(|schema| normalize_schema(root, schema)),
        },
    )))
}

fn synthesize_swagger2_parameter_schema(parameter: &Map<String, Value>) -> Option<Value> {
    if parameter.get("schema").is_some() {
        return None;
    }

    let mut schema = Map::new();

    for field_name in ["type", "items", "allOf", "oneOf", "anyOf", "properties"] {
        if let Some(value) = parameter.get(field_name) {
            schema.insert(field_name.to_string(), value.clone());
        }
    }

    if schema.is_empty() {
        None
    } else {
        Some(Value::Object(schema))
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
