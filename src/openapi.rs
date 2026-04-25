use std::fs;

use anyhow::{anyhow, Context};
use serde_json::Value;
use url::Url;

#[derive(Debug, Clone)]
pub struct Document {
    pub value: Value,
    pub source: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OpenApiStatistics {
    pub path_items: usize,
    pub operations: usize,
    pub parameters: usize,
    pub request_bodies: usize,
    pub responses: usize,
    pub links: usize,
    pub callbacks: usize,
    pub schemas: usize,
}

pub fn load_document(path_or_url: &str) -> anyhow::Result<Document> {
    let content = if is_http_url(path_or_url) {
        reqwest::blocking::get(path_or_url)
            .with_context(|| format!("Failed to GET {path_or_url}"))?
            .error_for_status()
            .with_context(|| format!("Failed to GET {path_or_url}"))?
            .text()
            .with_context(|| format!("Failed to read response body from {path_or_url}"))?
    } else {
        fs::read_to_string(path_or_url)
            .with_context(|| format!("Failed to read OpenAPI file {path_or_url}"))?
    };

    parse_document(&content).map(|value| Document {
        value,
        source: path_or_url.to_string(),
    })
}

pub fn parse_document(content: &str) -> anyhow::Result<Value> {
    serde_json::from_str(content)
        .or_else(|json_error| {
            serde_yaml::from_str(content)
                .map_err(|yaml_error| anyhow!("Input is not valid JSON or YAML. JSON error: {json_error}; YAML error: {yaml_error}"))
        })
}

pub fn is_http_url(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}

pub fn statistics(document: &Document) -> OpenApiStatistics {
    let mut stats = OpenApiStatistics::default();
    let Some(paths) = document.value.get("paths").and_then(Value::as_object) else {
        return stats;
    };

    stats.path_items = paths.len();
    for path_item in paths.values() {
        stats.parameters += path_item
            .get("parameters")
            .and_then(Value::as_array)
            .map_or(0, Vec::len);

        for operation in operations(path_item).map(|(_, operation)| operation) {
            stats.operations += 1;
            stats.parameters += operation
                .get("parameters")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            if operation.get("requestBody").is_some()
                || operation
                    .get("parameters")
                    .and_then(Value::as_array)
                    .is_some_and(|parameters| {
                        parameters
                            .iter()
                            .any(|p| p.get("in").and_then(Value::as_str) == Some("body"))
                    })
            {
                stats.request_bodies += 1;
            }
            stats.responses += operation
                .get("responses")
                .and_then(Value::as_object)
                .map_or(0, |responses| responses.len());
            stats.callbacks += operation
                .get("callbacks")
                .and_then(Value::as_object)
                .map_or(0, |callbacks| callbacks.len());
        }
    }

    stats.schemas = document
        .value
        .pointer("/components/schemas")
        .or_else(|| document.value.pointer("/definitions"))
        .and_then(Value::as_object)
        .map_or(0, |schemas| schemas.len());

    stats.links = count_key_recursive(&document.value, "links");
    stats
}

pub fn operations(path_item: &Value) -> impl Iterator<Item = (&str, &Value)> {
    path_item
        .as_object()
        .into_iter()
        .flat_map(|object| object.iter())
        .filter_map(|(key, value)| is_http_method(key).then_some((key.as_str(), value)))
}

pub fn server_url(document: &Document) -> String {
    if let Some(url) = document
        .value
        .get("servers")
        .and_then(Value::as_array)
        .and_then(|servers| servers.first())
        .and_then(|server| server.get("url"))
        .and_then(Value::as_str)
    {
        return url.to_string();
    }

    let host = document
        .value
        .get("host")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if host.is_empty() {
        return String::new();
    }

    let scheme = document
        .value
        .get("schemes")
        .and_then(Value::as_array)
        .and_then(|schemes| schemes.first())
        .and_then(Value::as_str)
        .unwrap_or("https");
    let base_path = document
        .value
        .get("basePath")
        .and_then(Value::as_str)
        .unwrap_or_default();
    format!("{scheme}://{host}{base_path}")
}

pub fn resolve_ref<'a>(document: &'a Value, value: &'a Value) -> &'a Value {
    let Some(reference) = value.get("$ref").and_then(Value::as_str) else {
        return value;
    };
    let Some(pointer) = reference.strip_prefix('#') else {
        return value;
    };
    document.pointer(pointer).unwrap_or(value)
}

pub fn is_absolute_url(value: &str) -> bool {
    Url::parse(value).is_ok_and(|url| url.has_host())
}

fn is_http_method(value: &str) -> bool {
    matches!(
        value,
        "get" | "put" | "post" | "delete" | "options" | "head" | "patch" | "trace"
    )
}

fn count_key_recursive(value: &Value, key: &str) -> usize {
    match value {
        Value::Object(object) => {
            object.contains_key(key) as usize
                + object
                    .values()
                    .map(|child| count_key_recursive(child, key))
                    .sum::<usize>()
        }
        Value::Array(array) => array
            .iter()
            .map(|child| count_key_recursive(child, key))
            .sum(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_yaml() {
        let value = parse_document("openapi: 3.0.0\npaths: {}\n").unwrap();
        assert_eq!(value["openapi"], "3.0.0");
    }

    #[test]
    fn computes_v2_server_url() {
        let value = parse_document(
            r#"{"swagger":"2.0","host":"example.com","basePath":"/v1","schemes":["https"]}"#,
        )
        .unwrap();
        let document = Document {
            value,
            source: "openapi.json".to_string(),
        };
        assert_eq!(server_url(&document), "https://example.com/v1");
    }

    #[test]
    fn computes_statistics_for_v3_document() {
        let value = parse_document(
            r#"{"openapi":"3.0.0","paths":{"/pets/{id}":{"parameters":[{"name":"id","in":"path"}],"get":{"parameters":[{"name":"q","in":"query"}],"requestBody":{"content":{"application/json":{"schema":{"type":"object"}}}},"responses":{"200":{"description":"ok"}},"callbacks":{"cb":{}}}}},"components":{"schemas":{"Pet":{"type":"object"}}}}"#,
        )
        .unwrap();
        let document = Document {
            value,
            source: "openapi.json".to_string(),
        };
        let stats = statistics(&document);
        assert_eq!(stats.path_items, 1);
        assert_eq!(stats.operations, 1);
        assert_eq!(stats.parameters, 2);
        assert_eq!(stats.request_bodies, 1);
        assert_eq!(stats.responses, 1);
        assert_eq!(stats.callbacks, 1);
        assert_eq!(stats.schemas, 1);
    }

    #[test]
    fn resolves_local_refs() {
        let value = parse_document(r##"{"components":{"schemas":{"Pet":{"type":"object"}}},"schema":{"$ref":"#/components/schemas/Pet"}}"##).unwrap();
        let resolved = resolve_ref(&value, &value["schema"]);
        assert_eq!(resolved["type"], "object");
    }
}
