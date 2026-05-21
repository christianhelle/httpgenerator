//! Data structures that configure generation and describe generated files.
//!
//! This module contains the main library-facing types for rendering:
//!
//! - [`GeneratorSettings`] controls how requests are emitted
//! - [`OutputType`] selects the file layout strategy
//! - [`GeneratorResult`] and [`HttpFile`] describe the generated output

mod output_type;
mod result;
mod settings;

#[cfg(test)]
mod tests;

pub use output_type::OutputType;
pub use result::{GeneratorResult, HttpFile};
pub use settings::GeneratorSettings;
