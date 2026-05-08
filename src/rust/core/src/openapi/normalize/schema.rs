use serde_json::Value;

use crate::{NormalizedSchema, NormalizedSchemaProperty, NormalizedSchemaType};

pub(super) fn normalize_schema(root: &Value, value: &Value) -> NormalizedSchema {
    let mut resolution_stack = Vec::new();
    normalize_schema_with_resolution(root, value, &mut resolution_stack)
}

fn normalize_schema_with_resolution(
    root: &Value,
    value: &Value,
    resolution_stack: &mut Vec<String>,
) -> NormalizedSchema {
    match value {
        Value::Object(schema) => {
            let reference = schema
                .get("$ref")
                .and_then(Value::as_str)
                .map(str::to_string);
            let mut normalized = reference
                .as_deref()
                .and_then(|reference| resolve_internal_reference(root, reference, resolution_stack))
                .unwrap_or_default();
            let overlay = NormalizedSchema {
                reference,
                types: normalize_schema_types(schema.get("type")),
                properties: schema
                    .get("properties")
                    .and_then(Value::as_object)
                    .map(|properties| {
                        properties
                            .iter()
                            .map(|(name, property)| NormalizedSchemaProperty {
                                name: name.clone(),
                                schema: normalize_schema_with_resolution(
                                    root,
                                    property,
                                    resolution_stack,
                                ),
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                items: schema.get("items").map(|items| {
                    Box::new(normalize_schema_with_resolution(
                        root,
                        items,
                        resolution_stack,
                    ))
                }),
                all_of: normalize_schema_array(root, schema.get("allOf"), resolution_stack),
                one_of: normalize_schema_array(root, schema.get("oneOf"), resolution_stack),
                any_of: normalize_schema_array(root, schema.get("anyOf"), resolution_stack),
            };

            merge_schema(&mut normalized, overlay);
            normalized
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

fn normalize_schema_array(
    root: &Value,
    value: Option<&Value>,
    resolution_stack: &mut Vec<String>,
) -> Vec<NormalizedSchema> {
    value
        .and_then(Value::as_array)
        .map(|schemas| {
            schemas
                .iter()
                .map(|schema| normalize_schema_with_resolution(root, schema, resolution_stack))
                .collect()
        })
        .unwrap_or_default()
}

fn resolve_internal_reference(
    root: &Value,
    reference: &str,
    resolution_stack: &mut Vec<String>,
) -> Option<NormalizedSchema> {
    if !reference.starts_with("#/") || resolution_stack.iter().any(|value| value == reference) {
        return None;
    }

    let target = root.pointer(&reference[1..])?;
    resolution_stack.push(reference.to_string());
    let resolved = normalize_schema_with_resolution(root, target, resolution_stack);
    resolution_stack.pop();
    Some(resolved)
}

fn merge_schema(base: &mut NormalizedSchema, overlay: NormalizedSchema) {
    if overlay.reference.is_some() {
        base.reference = overlay.reference;
    }
    if !overlay.types.is_empty() {
        base.types = overlay.types;
    }
    if !overlay.properties.is_empty() {
        base.properties = overlay.properties;
    }
    if overlay.items.is_some() {
        base.items = overlay.items;
    }
    if !overlay.all_of.is_empty() {
        base.all_of = overlay.all_of;
    }
    if !overlay.one_of.is_empty() {
        base.one_of = overlay.one_of;
    }
    if !overlay.any_of.is_empty() {
        base.any_of = overlay.any_of;
    }
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
