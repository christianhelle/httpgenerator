//! Rendering pipeline for turning normalized documents into `.http` files.
//!
//! Use this module when you already have a [`crate::NormalizedOpenApiDocument`] and want to
//! produce `.http` files without going back through OpenAPI loading or normalization.
//!
//! Rendering behavior is driven by [`crate::GeneratorSettings`] and [`crate::OutputType`]. This
//! module does not fetch documents, validate them, or mutate the normalized model; it only turns
//! that model into one or more in-memory [`crate::HttpFile`] values.
//!
//! Most consumers only need [`generate_http_files`].

mod modes;
mod render;
mod sample;
mod text;

#[cfg(test)]
mod tests;

pub use modes::generate_http_files;
