use httpgenerator_core::GeneratorSettings;

use crate::args::CliArgs;

pub(super) fn build_generator_settings(
    args: &CliArgs,
    open_api_path: String,
    authorization_header: Option<String>,
) -> GeneratorSettings {
    GeneratorSettings {
        open_api_path,
        authorization_header,
        authorization_header_from_environment_variable: args
            .authorization_header_from_environment_variable,
        authorization_header_variable_name: args.authorization_header_variable_name.clone(),
        content_type: args.content_type.clone(),
        base_url: args.base_url.clone(),
        output_type: args.output_type.into(),
        timeout: args.timeout,
        generate_intellij_tests: args.generate_intellij_tests,
        custom_headers: args.custom_headers.clone(),
        skip_headers: args.skip_headers,
    }
}
