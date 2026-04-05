mod error;
mod format;
mod loader;
mod raw;
mod source;
mod typed;
mod version;

pub use error::{
    ContentFormatDetectionError, OpenApiDocumentLoadError, RawOpenApiLoadError,
    SourceClassificationError, SpecificationVersionDetectionError, TypedOpenApiParseError,
};
pub use format::{OpenApiContentFormat, detect_content_format, sniff_content_format};
pub use loader::{
    LoadedOpenApiDocument, load_document, load_document_from_raw, load_document_from_source,
};
pub use raw::{
    RawOpenApiDocument, decode_raw_document, load_raw_document, load_raw_document_from_source,
};
pub use source::{OpenApiSource, classify_source};
pub use typed::{
    TypedOpenApiDocument, parse_openapi30_document, parse_openapi31_document, parse_typed_document,
};
pub use version::{OpenApiSpecificationVersion, detect_specification_version};
