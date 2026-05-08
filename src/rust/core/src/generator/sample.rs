use crate::{NormalizedSchema, NormalizedSchemaType};

pub(super) fn generate_sample_json(schema: &NormalizedSchema) -> String {
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
