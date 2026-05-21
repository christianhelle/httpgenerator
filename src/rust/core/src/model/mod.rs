//! Generation settings and in-memory output types.
//!
//! These types are independent of OpenAPI parsing. They describe how normalized
//! operations should be rendered and how generated `.http` files are returned to
//! callers.

mod output_type;
mod result;
mod settings;

#[cfg(test)]
mod tests;

pub use output_type::OutputType;
pub use result::{GeneratorResult, HttpFile};
pub use settings::GeneratorSettings;
