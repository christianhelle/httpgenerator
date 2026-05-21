use serde::{Deserialize, Serialize};

/// HTTP methods supported by the normalized renderer model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedHttpMethod {
    /// `GET`.
    Get,
    /// `PUT`.
    Put,
    /// `POST`.
    Post,
    /// `DELETE`.
    Delete,
    /// `OPTIONS`.
    Options,
    /// `HEAD`.
    Head,
    /// `PATCH`.
    Patch,
    /// `TRACE`.
    Trace,
}

impl NormalizedHttpMethod {
    /// Returns the lowercase HTTP method token used in generated `.http` files.
    ///
    /// # Example
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

/// Location of an OpenAPI parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedParameterLocation {
    /// Path parameter, usually replacing `{name}` in the path template.
    Path,
    /// Query-string parameter.
    Query,
    /// Header parameter.
    Header,
    /// Cookie parameter.
    Cookie,
}
