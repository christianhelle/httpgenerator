pub mod base_url;
pub mod file_naming;
pub mod model;
pub mod operation_name;
pub mod privacy;
pub mod string_extensions;

pub use base_url::resolve_base_url;
pub use file_naming::unique_filename;
pub use model::{GeneratorResult, GeneratorSettings, HttpFile, OutputType};
pub use operation_name::generate_operation_name;
pub use privacy::redact_authorization_headers;
pub use string_extensions::{
    capitalize_first_character, convert_kebab_case_to_pascal_case, convert_route_to_camel_case,
    convert_spaces_to_pascal_case, prefix, prefix_line_breaks,
};
