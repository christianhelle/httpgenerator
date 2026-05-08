use std::fs;

use serde_json::Value;

use crate::NormalizedServer;

use super::super::{LoadedOpenApiDocument, OpenApiNormalizationError, OpenApiSource};

pub(super) fn normalize_servers(
    document: &LoadedOpenApiDocument,
) -> Result<Vec<NormalizedServer>, OpenApiNormalizationError> {
    let value = document.raw().value();
    let Some(servers) = value.get("servers") else {
        if value.get("swagger").is_some() {
            return normalize_swagger2_servers(value, document.source());
        }

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

fn normalize_swagger2_servers(
    value: &Value,
    source: &OpenApiSource,
) -> Result<Vec<NormalizedServer>, OpenApiNormalizationError> {
    let host = value
        .get("host")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    let base_path = value
        .get("basePath")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let schemes = value.get("schemes");

    if host.is_empty() && base_path.is_empty() {
        return Ok(local_swagger2_file_server(source)
            .into_iter()
            .map(|url| NormalizedServer { url })
            .collect());
    }

    let schemes = match schemes {
        Some(schemes) => {
            let Some(schemes) = schemes.as_array() else {
                return Err(OpenApiNormalizationError::InvalidStructure {
                    path: "schemes".to_string(),
                    context: "expected an array".to_string(),
                });
            };

            schemes
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        }
        None => Vec::new(),
    };

    if host.is_empty() {
        return Ok(vec![NormalizedServer {
            url: base_path.to_string(),
        }]);
    }

    if schemes.is_empty() {
        return Ok(vec![NormalizedServer {
            url: format!("https://{host}{base_path}"),
        }]);
    }

    Ok(schemes
        .into_iter()
        .map(|scheme| NormalizedServer {
            url: format!("{scheme}://{host}{base_path}"),
        })
        .collect())
}

fn local_swagger2_file_server(source: &OpenApiSource) -> Option<String> {
    let OpenApiSource::Path(path) = source else {
        return None;
    };

    let path = fs::canonicalize(path).unwrap_or_else(|_| path.clone());
    let directory = path.parent()?;
    let mut directory = directory.to_string_lossy().into_owned();

    if let Some(stripped) = directory.strip_prefix(r"\\?\") {
        directory = stripped.to_string();
    }

    directory = directory.replace('\\', "/");
    Some(format!("file://{directory}"))
}
