//! Rendering pipeline for turning normalized documents into `.http` files.
//!
//! Most consumers only need [`generate_http_files`], which chooses the output layout based on
//! [`crate::OutputType`] and returns an in-memory [`crate::GeneratorResult`].

mod modes;
mod render;
mod sample;
mod text;

#[cfg(test)]
mod tests;

pub use modes::generate_http_files;
