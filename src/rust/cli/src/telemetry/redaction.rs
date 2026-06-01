use crate::args::{CliArgs, OutputTypeArg};
use httpgenerator_core::redact_authorization_headers;
use serde_json::{Map, Value};
use std::ffi::OsString;

const REDACTED: &str = "[REDACTED]";

pub(super) fn feature_usage_names(args: &CliArgs) -> Vec<String> {
    let mut features = Vec::new();

    if args.skip_validation {
        features.push("skip-validation".to_string());
    }

    if args.authorization_header.is_some() {
        features.push("authorization-header".to_string());
    }

    if args.authorization_header_from_environment_variable {
        features.push("load-authorization-header-from-environment".to_string());
    }

    features.push("authorization-header-variable-name".to_string());
    features.push("content-type".to_string());

    if args.base_url.is_some() {
        features.push("base-url".to_string());
    }

    features.push("output-type".to_string());

    if args.azure_scope.is_some() {
        features.push("azure-scope".to_string());
    }

    if args.azure_tenant_id.is_some() {
        features.push("azure-tenant-id".to_string());
    }

    features.push("timeout".to_string());

    if args.generate_intellij_tests {
        features.push("generate-intellij-tests".to_string());
    }

    if !args.custom_headers.is_empty() {
        features.push("custom-header".to_string());
    }

    if args.skip_headers {
        features.push("skip-headers".to_string());
    }

    features
}

pub(super) fn redacted_command_line(raw_args: &[OsString]) -> String {
    let mut arguments = raw_args
        .iter()
        .map(|value| value.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    if let Some(program_name) = arguments.first_mut() {
        *program_name = "httpgenerator".to_string();
    }

    redact_authorization_headers(&arguments.join(" "))
}

pub(super) fn redacted_settings(args: &CliArgs) -> Map<String, Value> {
    let mut settings = Map::new();

    settings.insert(
        "openApiPath".to_string(),
        option_string_value(args.open_api_path.as_deref()),
    );
    settings.insert(
        "outputFolder".to_string(),
        Value::String(args.output_folder.clone()),
    );
    settings.insert("noLogging".to_string(), Value::Bool(args.no_logging));
    settings.insert(
        "skipValidation".to_string(),
        Value::Bool(args.skip_validation),
    );
    settings.insert(
        "authorizationHeader".to_string(),
        redacted_authorization_value(args.authorization_header.as_deref()),
    );
    settings.insert(
        "authorizationHeaderFromEnvironmentVariable".to_string(),
        Value::Bool(args.authorization_header_from_environment_variable),
    );
    settings.insert(
        "authorizationHeaderVariableName".to_string(),
        Value::String(args.authorization_header_variable_name.clone()),
    );
    settings.insert(
        "contentType".to_string(),
        Value::String(args.content_type.clone()),
    );
    settings.insert(
        "baseUrl".to_string(),
        option_string_value(args.base_url.as_deref()),
    );
    settings.insert(
        "outputType".to_string(),
        Value::from(output_type_ordinal(args.output_type)),
    );
    settings.insert(
        "azureScope".to_string(),
        option_string_value(args.azure_scope.as_deref()),
    );
    settings.insert(
        "azureTenantId".to_string(),
        option_string_value(args.azure_tenant_id.as_deref()),
    );
    settings.insert("timeout".to_string(), Value::from(args.timeout));
    settings.insert(
        "generateIntellijTests".to_string(),
        Value::Bool(args.generate_intellij_tests),
    );
    settings.insert(
        "customHeaders".to_string(),
        Value::Array(
            args.custom_headers
                .iter()
                .map(|value| Value::String(redact_custom_header(value)))
                .collect(),
        ),
    );
    settings.insert("skipHeaders".to_string(), Value::Bool(args.skip_headers));

    settings
}

fn option_string_value(value: Option<&str>) -> Value {
    value
        .map(|value| Value::String(value.to_string()))
        .unwrap_or(Value::Null)
}

fn redacted_authorization_value(value: Option<&str>) -> Value {
    value
        .map(|_| Value::String(REDACTED.to_string()))
        .unwrap_or(Value::Null)
}

fn output_type_ordinal(output_type: OutputTypeArg) -> u8 {
    match output_type {
        OutputTypeArg::OneRequestPerFile => 0,
        OutputTypeArg::OneFile => 1,
        OutputTypeArg::OneFilePerTag => 2,
    }
}

fn redact_custom_header(value: &str) -> String {
    let Some((name, _)) = value.split_once(':') else {
        return value.to_string();
    };

    if name.trim().eq_ignore_ascii_case("authorization") {
        format!("{}: {REDACTED}", name.trim())
    } else {
        value.to_string()
    }
}
