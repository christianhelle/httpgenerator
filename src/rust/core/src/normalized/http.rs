use serde::{Deserialize, Serialize};

/// Normalized HTTP method used by the generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedHttpMethod {
    /// `GET`
    Get,
    /// `PUT`
    Put,
    /// `POST`
    Post,
    /// `DELETE`
    Delete,
    /// `OPTIONS`
    Options,
    /// `HEAD`
    Head,
    /// `PATCH`
    Patch,
    /// `TRACE`
    Trace,
}

impl NormalizedHttpMethod {
    /// Returns the lowercase method name used in normalized OpenAPI-style lookups and rendering.
    ///
    /// # Examples
    ///
    /// ```
    /// use httpgenerator_core::NormalizedHttpMethod;
    ///
    /// assert_eq!(NormalizedHttpMethod::Patch.as_str(), "patch");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Put => "put",
            Self::Post => "post",
            Self::Delete => "delete",
            Self::Options => "options",
            Self::Head => "head",
            Self::Patch => "patch",
            Self::Trace => "trace",
        }
    }
}

/// Location of a parameter in the rendered request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedParameterLocation {
    /// URL path placeholder, such as `{petId}`.
    Path,
    /// Query string parameter.
    Query,
    /// HTTP header parameter.
    Header,
    /// Cookie parameter.
    Cookie,
}
