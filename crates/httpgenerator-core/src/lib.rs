pub mod base_url;
pub mod file_naming;
pub mod model;
pub mod normalized;
pub mod operation_name;
pub mod privacy;
pub mod string_extensions;
pub mod support_information;

pub use base_url::resolve_base_url;
pub use file_naming::unique_filename;
pub use model::{GeneratorResult, GeneratorSettings, HttpFile, OutputType};
pub use normalized::{
    NormalizedHttpMethod, NormalizedInlineParameter, NormalizedInlineRequestBody,
    NormalizedMediaType, NormalizedOpenApiDocument, NormalizedOperation, NormalizedParameter,
    NormalizedParameterLocation, NormalizedRequestBody, NormalizedSchema, NormalizedSchemaProperty,
    NormalizedSchemaType, NormalizedServer, NormalizedSpecificationVersion,
};
pub use operation_name::generate_operation_name;
pub use privacy::redact_authorization_headers;
pub use string_extensions::{
    capitalize_first_character, convert_kebab_case_to_pascal_case, convert_route_to_camel_case,
    convert_spaces_to_pascal_case, prefix, prefix_line_breaks,
};
pub use support_information::{
    anonymous_identity, anonymous_identity_from_parts, support_key,
    support_key_from_anonymous_identity,
};
