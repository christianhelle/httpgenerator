//! Data structures that configure generation and describe generated files.
//!
//! Use this module when you need to configure rendering or carry generated files across your own
//! library or host boundary.
//!
//! These types are plain data containers for the rendering pipeline:
//!
//! - [`GeneratorSettings`] controls how requests are emitted
//! - [`OutputType`] selects the file layout strategy
//! - [`GeneratorResult`] and [`HttpFile`] describe the generated output
//!
//! They do not load, normalize, or render documents by themselves.

mod output_type;
mod result;
mod settings;

#[cfg(test)]
mod tests;

pub use output_type::OutputType;
pub use result::{GeneratorResult, HttpFile};
pub use settings::GeneratorSettings;
