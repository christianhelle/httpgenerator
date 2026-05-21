//! `.http` renderer for normalized OpenAPI documents.
//!
//! The generator accepts a [`NormalizedOpenApiDocument`](crate::NormalizedOpenApiDocument)
//! and [`GeneratorSettings`](crate::GeneratorSettings), then returns an in-memory
//! [`GeneratorResult`](crate::GeneratorResult). Host applications decide whether
//! to write the files to disk, show them in an editor, or package them for
//! another workflow.

mod modes;
mod render;
mod sample;
mod text;

#[cfg(test)]
mod tests;

pub use modes::generate_http_files;
