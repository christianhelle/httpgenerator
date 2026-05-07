mod error;
mod format;
mod inspect;
mod loader;
mod normalize;
mod raw;
mod source;
mod typed;
mod version;

pub mod base_url;
pub mod file_naming;
pub mod generator;
pub mod model;
pub mod normalized;
pub mod operation_name;
pub mod privacy;
pub mod string_extensions;
pub mod support_information;

pub use base_url::resolve_base_url;
pub use error::{
    ContentFormatDetectionError, OpenApiDocumentLoadError, OpenApiDocumentNormalizationError,
    OpenApiInspectionError, OpenApiNormalizationError, RawOpenApiLoadError,
    SourceClassificationError, SpecificationVersionDetectionError, TypedOpenApiParseError,
};
pub use file_naming::unique_filename;
pub use format::{OpenApiContentFormat, detect_content_format, sniff_content_format};
pub use generator::generate_http_files;
pub use inspect::{OpenApiInspection, OpenApiStats, inspect_document, inspect_raw_document};
pub use loader::{
    LoadedOpenApiDocument, load_document, load_document_from_raw, load_document_from_source,
};
pub use model::{GeneratorResult, GeneratorSettings, HttpFile, OutputType};
pub use normalize::{
    load_and_normalize_document, load_and_normalize_document_with_options,
    normalize_loaded_document,
};
pub use normalized::{
    NormalizedHttpMethod, NormalizedInlineParameter, NormalizedInlineRequestBody,
    NormalizedMediaType, NormalizedOpenApiDocument, NormalizedOperation, NormalizedParameter,
    NormalizedParameterLocation, NormalizedRequestBody, NormalizedSchema, NormalizedSchemaProperty,
    NormalizedSchemaType, NormalizedServer, NormalizedSpecificationVersion,
};
pub use operation_name::generate_operation_name;
pub use privacy::redact_authorization_headers;
pub use raw::{
    RawOpenApiDocument, decode_raw_document, load_raw_document, load_raw_document_from_source,
};
pub use source::{OpenApiSource, classify_source};
pub use string_extensions::{
    capitalize_first_character, convert_kebab_case_to_pascal_case, convert_route_to_camel_case,
    convert_spaces_to_pascal_case, prefix, prefix_line_breaks,
};
pub use support_information::{
    anonymous_identity, anonymous_identity_from_parts, support_key,
    support_key_from_anonymous_identity,
};
pub use typed::{
    TypedOpenApiDocument, parse_openapi30_document, parse_openapi31_document, parse_typed_document,
};
pub use version::{OpenApiSpecificationVersion, detect_specification_version};
