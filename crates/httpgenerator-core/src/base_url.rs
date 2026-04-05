use url::Url;

pub fn resolve_base_url(
    open_api_path: &str,
    server_url: Option<&str>,
    configured_base_url: Option<&str>,
) -> String {
    let server_url = server_url.unwrap_or_default();
    let mut base_url = configured_base_url.unwrap_or_default().to_string();

    if base_url.trim().is_empty() {
        base_url = server_url.to_string();
    } else if !is_absolute_uri(server_url) {
        base_url.push_str(server_url);
    }

    if let Some(configured_base_url) = configured_base_url {
        if configured_base_url.starts_with("{{") && configured_base_url.ends_with("}}") {
            return base_url;
        }
    }

    if !is_absolute_uri(&base_url)
        && open_api_path.to_ascii_lowercase().starts_with("http")
        && let Some(authority) = authority(open_api_path)
    {
        base_url = format!("{authority}{base_url}");
    }

    base_url
}

fn is_absolute_uri(value: &str) -> bool {
    if is_dotnet_style_windows_file_uri(value) {
        return false;
    }

    Url::parse(value).is_ok()
}

fn authority(value: &str) -> Option<String> {
    let url = Url::parse(value).ok()?;
    let host = url.host_str()?;
    let mut authority = format!("{}://{}", url.scheme(), host);

    if let Some(port) = url.port() {
        authority.push(':');
        authority.push_str(&port.to_string());
    }

    Some(authority)
}

fn is_dotnet_style_windows_file_uri(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("file://") else {
        return false;
    };

    let bytes = rest.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && bytes[2] == b'/'
}

#[cfg(test)]
mod tests {
    use super::resolve_base_url;

    #[test]
    fn uses_openapi_authority_for_relative_server_urls() {
        let base_url = resolve_base_url(
            "https://petstore.swagger.io/v2/swagger.json",
            Some("/api/v3"),
            None,
        );

        assert_eq!(base_url, "https://petstore.swagger.io/api/v3");
    }

    #[test]
    fn preserves_configured_absolute_base_url() {
        let base_url = resolve_base_url(
            "https://petstore.swagger.io/v2/swagger.json",
            Some("https://petstore.swagger.io/api/v3"),
            Some("https://api.example.com"),
        );

        assert_eq!(base_url, "https://api.example.com");
    }

    #[test]
    fn appends_relative_server_path_to_environment_base_url() {
        let base_url = resolve_base_url(
            "https://petstore.swagger.io/v2/swagger.json",
            Some("/api/v3"),
            Some("{{MY_BASE_URL}}"),
        );

        assert_eq!(base_url, "{{MY_BASE_URL}}/api/v3");
    }

    #[test]
    fn falls_back_to_openapi_authority_when_no_server_url_exists() {
        let base_url = resolve_base_url("https://petstore.swagger.io/v2/swagger.json", None, None);

        assert_eq!(base_url, "https://petstore.swagger.io");
    }

    #[test]
    fn appends_dotnet_style_windows_file_uri_to_configured_base_url() {
        let base_url = resolve_base_url(
            "C:\\specs\\petstore.json",
            Some("file://C:/specs"),
            Some("https://api.example.io/"),
        );

        assert_eq!(base_url, "https://api.example.io/file://C:/specs");
    }

    #[test]
    fn uses_dotnet_style_windows_file_uri_when_no_base_url_is_configured() {
        let base_url = resolve_base_url("C:\\specs\\petstore.json", Some("file://C:/specs"), None);

        assert_eq!(base_url, "file://C:/specs");
    }

    #[test]
    fn preserves_configured_absolute_base_url_for_local_specs_without_a_server_url() {
        let base_url = resolve_base_url(
            "C:\\specs\\petstore.json",
            None,
            Some("https://api.example.io/"),
        );

        assert_eq!(base_url, "https://api.example.io/");
    }
}
