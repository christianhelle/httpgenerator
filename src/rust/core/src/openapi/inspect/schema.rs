use serde_json::{Map, Value};

pub(super) fn count_request_body_schemas(request_body: &Value) -> usize {
    let Some(request_body) = request_body.as_object() else {
        return 0;
    };

    if is_reference_object(request_body) {
        return 0;
    }

    request_body
        .get("content")
        .and_then(Value::as_object)
        .map(count_media_type_schemas)
        .unwrap_or_default()
}

pub(super) fn count_response_value_schemas(response: &Value) -> usize {
    let Some(response) = response.as_object() else {
        return 0;
    };

    if is_reference_object(response) {
        return 0;
    }

    response
        .get("schema")
        .map(count_schema_objects)
        .unwrap_or_default()
        + response
            .get("content")
            .and_then(Value::as_object)
            .map(count_media_type_schemas)
            .unwrap_or_default()
}

pub(super) fn count_response_header_schemas_for_object(response: &Map<String, Value>) -> usize {
    if is_reference_object(response) {
        return 0;
    }

    response
        .get("headers")
        .and_then(Value::as_object)
        .map(count_header_schemas)
        .unwrap_or_default()
}

pub(super) fn count_response_links_for_object(response: &Map<String, Value>) -> usize {
    if is_reference_object(response) {
        return 0;
    }

    response
        .get("links")
        .and_then(Value::as_object)
        .map(|links| {
            links
                .values()
                .filter(|link| {
                    link.as_object()
                        .is_some_and(|link| !is_reference_object(link))
                })
                .count()
        })
        .unwrap_or_default()
}

pub(super) fn count_parameter_schema_for_object(parameter: &Map<String, Value>) -> usize {
    if is_reference_object(parameter) {
        return 0;
    }

    if let Some(schema) = parameter.get("schema") {
        return count_schema_objects(schema);
    }

    synthesized_parameter_schema(parameter)
        .map(|schema| count_schema_objects(&schema))
        .unwrap_or_default()
}

pub(super) fn count_header_schema_for_object(header: &Map<String, Value>) -> usize {
    if is_reference_object(header) {
        return 0;
    }

    header
        .get("schema")
        .map(count_schema_objects)
        .unwrap_or_default()
}

pub(super) fn count_schema_objects(value: &Value) -> usize {
    let Some(schema) = value.as_object() else {
        return 0;
    };

    let mut count = usize::from(is_schema_object(schema));

    if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
        count += properties.values().map(count_schema_objects).sum::<usize>();
    }

    if let Some(items) = schema.get("items") {
        count += count_schema_objects(items);
    }

    if let Some(additional_properties) = schema.get("additionalProperties") {
        count += count_schema_objects(additional_properties);
    }

    for field_name in ["allOf", "oneOf", "anyOf"] {
        if let Some(values) = schema.get(field_name).and_then(Value::as_array) {
            count += values.iter().map(count_schema_objects).sum::<usize>();
        }
    }

    count
}

pub(super) fn is_reference_object(value: &Map<String, Value>) -> bool {
    value.contains_key("$ref") && value.len() == 1
}

fn count_media_type_schemas(content: &Map<String, Value>) -> usize {
    content
        .values()
        .filter_map(|media_type| media_type.get("schema"))
        .map(count_schema_objects)
        .sum::<usize>()
}

fn count_header_schemas(headers: &Map<String, Value>) -> usize {
    headers
        .values()
        .filter_map(Value::as_object)
        .map(count_header_schema_for_object)
        .sum::<usize>()
}

fn synthesized_parameter_schema(parameter: &Map<String, Value>) -> Option<Value> {
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

fn is_schema_object(schema: &Map<String, Value>) -> bool {
    schema.contains_key("type")
        || schema.contains_key("properties")
        || schema.contains_key("items")
        || schema.contains_key("allOf")
        || schema.contains_key("oneOf")
        || schema.contains_key("anyOf")
        || schema.contains_key("additionalProperties")
}
