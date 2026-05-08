use crate::{
    GeneratorSettings, NormalizedInlineParameter, NormalizedOperation, NormalizedParameter,
    NormalizedParameterLocation, NormalizedRequestBody, NormalizedSchema, NormalizedSchemaType,
    OutputType, generate_operation_name, prefix_line_breaks,
};

use super::{
    sample::generate_sample_json,
    text::{push_blank_line, push_line},
};

pub(super) fn render_request(
    settings: &GeneratorSettings,
    operation: &NormalizedOperation,
) -> String {
    let mut content = String::new();
    append_summary(operation, &mut content);
    let parameter_name_map = append_parameters(settings, operation, &mut content);

    let mut url = operation.path.replace('{', "{{").replace('}', "}}");
    let mut query_parameters = Vec::new();

    for (original_name, generated_name) in &parameter_name_map {
        if operation.path.contains(&format!("{{{original_name}}}")) {
            url = url.replace(
                &format!("{{{{{original_name}}}}}"),
                &format!("{{{{{generated_name}}}}}"),
            );
        } else {
            query_parameters.push((original_name, generated_name));
        }
    }

    if !query_parameters.is_empty() {
        url.push('?');
        url.push_str(
            &query_parameters
                .iter()
                .map(|(name, generated)| format!("{name}={{{{{generated}}}}}"))
                .collect::<Vec<_>>()
                .join("&"),
        );
    }

    push_line(
        &mut content,
        &format!(
            "{} {{{{baseUrl}}}}{url}",
            operation.method.as_str().to_ascii_uppercase()
        ),
    );
    push_line(&mut content, "Content-Type: {{contentType}}");

    if settings.authorization_header_from_environment_variable
        || settings
            .authorization_header
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
    {
        push_line(
            &mut content,
            &format!(
                "Authorization: {{{{{}}}}}",
                settings.authorization_header_variable_name
            ),
        );
    }

    for header in &settings.custom_headers {
        push_line(&mut content, header);
    }

    push_blank_line(&mut content);

    let request_body = match &operation.request_body {
        Some(NormalizedRequestBody::Inline(request_body)) => request_body,
        _ => {
            generate_intellij_test(settings, &mut content);
            return content;
        }
    };

    let Some(media_type) = request_body
        .content
        .iter()
        .find(|content| content.content_type.contains(&settings.content_type))
    else {
        generate_intellij_test(settings, &mut content);
        return content;
    };

    let Some(schema) = media_type.schema.as_ref() else {
        generate_intellij_test(settings, &mut content);
        return content;
    };

    content.push_str(&generate_sample_json(schema));
    push_line(&mut content, "");
    generate_intellij_test(settings, &mut content);
    content
}

fn generate_intellij_test(settings: &GeneratorSettings, content: &mut String) {
    if !settings.generate_intellij_tests {
        return;
    }

    push_blank_line(content);
    push_line(content, "> {%");
    push_line(
        content,
        "    client.test(\"Request executed successfully\", function() {",
    );
    push_line(content, "        client.assert(");
    push_line(content, "            response.status === 200, ");
    push_line(content, "            \"Response status is not 200\");");
    push_line(content, "    });");
    push_line(content, "%}");
}

fn append_summary(operation: &NormalizedOperation, content: &mut String) {
    const PADDING: usize = 2;
    const SUMMARY_PREFIX: &str = "### Summary: ";
    const DESCRIPTION_PREFIX: &str = "### Description: ";

    let request = format!(
        "### Request: {} {}",
        operation.method.as_str().to_ascii_uppercase(),
        operation.path
    );
    let summary_length = operation
        .summary
        .as_deref()
        .map(|summary| SUMMARY_PREFIX.chars().count() + summary.chars().count() + PADDING)
        .unwrap_or_default();
    let description_length = operation
        .description
        .as_deref()
        .map(|description| {
            DESCRIPTION_PREFIX.chars().count() + description.chars().count() + PADDING
        })
        .unwrap_or_default();
    let border = "#".repeat(
        request
            .chars()
            .count()
            .saturating_add(PADDING)
            .max(summary_length)
            .max(description_length),
    );

    push_line(content, &border);
    push_line(content, &request);

    if let Some(summary) = operation
        .summary
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        push_line(content, &format!("{SUMMARY_PREFIX}{summary}"));
    }

    if let Some(description) = operation
        .description
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        let normalized = description.replace("\r\n", "\n");
        if normalized.contains('\n') {
            push_line(content, DESCRIPTION_PREFIX);
            for line in normalized.split('\n') {
                push_line(content, &format!("###   {line}"));
            }
        } else {
            push_line(content, &format!("{DESCRIPTION_PREFIX}{description}"));
        }
    }

    push_line(content, &border);
    push_blank_line(content);
}

fn append_parameters(
    settings: &GeneratorSettings,
    operation: &NormalizedOperation,
    content: &mut String,
) -> Vec<(String, String)> {
    let mut parameter_name_map = Vec::new();

    for parameter in operation.parameters.iter().filter_map(parameter_as_inline) {
        if !matches!(
            parameter.location,
            NormalizedParameterLocation::Path | NormalizedParameterLocation::Query
        ) {
            continue;
        }

        let parameter_name = parameter_name(settings, operation, parameter);
        let description = prefix_line_breaks(parameter.description.as_deref(), "###")
            .unwrap_or_else(|| parameter_name.clone());

        push_line(
            content,
            &format!(
                "### {} Parameter: {description}",
                parameter_location_name(parameter.location)
            ),
        );
        push_line(
            content,
            &format!(
                "@{parameter_name} = {}",
                parameter_default_value(parameter.schema.as_ref())
            ),
        );
        push_blank_line(content);

        parameter_name_map.push((parameter.name.clone(), parameter_name));
    }

    push_blank_line(content);
    parameter_name_map
}

fn parameter_as_inline(parameter: &NormalizedParameter) -> Option<&NormalizedInlineParameter> {
    match parameter {
        NormalizedParameter::Inline(parameter) => Some(parameter),
        NormalizedParameter::Reference { .. } => None,
    }
}

fn parameter_name(
    settings: &GeneratorSettings,
    operation: &NormalizedOperation,
    parameter: &NormalizedInlineParameter,
) -> String {
    if settings.output_type == OutputType::OneRequestPerFile {
        return parameter.name.clone();
    }

    format!("{}_{}", operation_name(operation), parameter.name)
}

pub(super) fn operation_name(operation: &NormalizedOperation) -> String {
    generate_operation_name(
        operation.method.as_str(),
        &operation.path,
        operation.operation_id.as_deref(),
    )
}

fn parameter_default_value(schema: Option<&NormalizedSchema>) -> &'static str {
    let Some(schema) = schema else {
        return "str";
    };

    if schema.types.contains(&NormalizedSchemaType::Integer)
        || schema.types.contains(&NormalizedSchemaType::Number)
    {
        return "0";
    }

    if schema.types.contains(&NormalizedSchemaType::Boolean) {
        return "true";
    }

    "str"
}

fn parameter_location_name(location: NormalizedParameterLocation) -> &'static str {
    match location {
        NormalizedParameterLocation::Path => "Path",
        NormalizedParameterLocation::Query => "Query",
        NormalizedParameterLocation::Header => "Header",
        NormalizedParameterLocation::Cookie => "Cookie",
    }
}
