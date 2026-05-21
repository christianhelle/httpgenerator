/// Converts a kebab-case string to PascalCase using compatibility rules.
///
/// Empty segments are ignored and dots are replaced with underscores.
///
/// # Example
///
/// ```
/// use httpgenerator_core::convert_kebab_case_to_pascal_case;
///
/// assert_eq!(
///     convert_kebab_case_to_pascal_case("find-pets.by-status"),
///     "FindPets_byStatus"
/// );
/// ```
pub fn convert_kebab_case_to_pascal_case(value: &str) -> String {
    value
        .split('-')
        .filter(|part| !part.is_empty())
        .map(|part| capitalize_first_character(part).replace('.', "_"))
        .collect::<Vec<_>>()
        .join("")
}

/// Converts a route path into a camel-cased identifier fragment.
///
/// Leading and repeated slashes are ignored. The first path segment keeps its
/// original casing, while subsequent segments are capitalized.
///
/// # Example
///
/// ```
/// use httpgenerator_core::convert_route_to_camel_case;
///
/// assert_eq!(convert_route_to_camel_case("/pet/findByStatus"), "petFindByStatus");
/// ```
pub fn convert_route_to_camel_case(value: &str) -> String {
    let mut parts = value
        .split('/')
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();

    for part in parts.iter_mut().skip(1) {
        *part = capitalize_first_character(part);
    }

    parts.join("")
}

/// Uppercases the first Unicode scalar value in a string.
///
/// Returns an empty string for empty input.
///
/// # Example
///
/// ```
/// use httpgenerator_core::capitalize_first_character;
///
/// assert_eq!(capitalize_first_character("pet"), "Pet");
/// assert_eq!(capitalize_first_character(""), "");
/// ```
pub fn capitalize_first_character(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };

    format!("{}{}", first.to_uppercase(), chars.as_str())
}

/// Converts space-separated words to PascalCase.
///
/// # Example
///
/// ```
/// use httpgenerator_core::convert_spaces_to_pascal_case;
///
/// assert_eq!(convert_spaces_to_pascal_case("list pets"), "ListPets");
/// ```
pub fn convert_spaces_to_pascal_case(value: &str) -> String {
    value
        .split(' ')
        .filter(|part| !part.is_empty())
        .map(capitalize_first_character)
        .collect::<Vec<_>>()
        .join("")
}

/// Prefixes a value unless it already starts with that prefix.
///
/// # Example
///
/// ```
/// use httpgenerator_core::prefix;
///
/// assert_eq!(prefix("Pet", "Get"), "GetPet");
/// assert_eq!(prefix("GetPet", "Get"), "GetPet");
/// ```
pub fn prefix(value: &str, prefix_value: &str) -> String {
    if value.starts_with(prefix_value) {
        return value.to_string();
    }

    format!("{prefix_value}{value}")
}

/// Prefixes every line break in a string with a marker and a space.
///
/// This preserves the platform newline style used by the renderer. `None`
/// remains `None`.
///
/// # Example
///
/// ```
/// use httpgenerator_core::prefix_line_breaks;
///
/// let value = prefix_line_breaks(Some("line 1\nline 2"), "###").unwrap();
///
/// assert!(value.contains("### line 2"));
/// ```
pub fn prefix_line_breaks(value: Option<&str>, prefix_value: &str) -> Option<String> {
    value.map(|value| {
        let newline = if cfg!(windows) { "\r\n" } else { "\n" };
        value
            .replace("\r\n", "\n")
            .replace('\n', &format!("{newline}{prefix_value} "))
    })
}

#[cfg(test)]
mod tests {
    use super::{
        capitalize_first_character, convert_kebab_case_to_pascal_case, convert_route_to_camel_case,
        convert_spaces_to_pascal_case, prefix, prefix_line_breaks,
    };

    #[test]
    fn convert_kebab_case_to_pascal_case_should_convert() {
        let cases = [
            ("kebab-case-string", "KebabCaseString"),
            ("another-kebab-case-string", "AnotherKebabCaseString"),
            ("string-with.dot", "StringWith_dot"),
            ("", ""),
            ("single", "Single"),
        ];

        for (input, expected) in cases {
            assert_eq!(convert_kebab_case_to_pascal_case(input), expected);
        }
    }

    #[test]
    fn convert_route_to_camel_case_should_convert() {
        let cases = [
            ("/route/to/resource", "routeToResource"),
            ("/another/route/to/resource", "anotherRouteToResource"),
        ];

        for (input, expected) in cases {
            assert_eq!(convert_route_to_camel_case(input), expected);
        }
    }

    #[test]
    fn capitalize_first_character_should_capitalize() {
        let cases = [
            ("string", "String"),
            ("anotherString", "AnotherString"),
            ("a", "A"),
            ("", ""),
        ];

        for (input, expected) in cases {
            assert_eq!(capitalize_first_character(input), expected);
        }
    }

    #[test]
    fn convert_spaces_to_pascal_case_should_convert() {
        let cases = [
            ("string with spaces", "StringWithSpaces"),
            ("another string with spaces", "AnotherStringWithSpaces"),
        ];

        for (input, expected) in cases {
            assert_eq!(convert_spaces_to_pascal_case(input), expected);
        }
    }

    #[test]
    fn prefix_should_add_prefix() {
        let cases = [
            ("string", "prefix", "prefixstring"),
            ("prefixstring", "prefix", "prefixstring"),
            ("test", "", "test"),
            ("", "prefix", "prefix"),
        ];

        for (input, prefix_value, expected) in cases {
            assert_eq!(prefix(input, prefix_value), expected);
        }
    }

    #[test]
    fn prefix_line_breaks_should_add_prefix() {
        let input = if cfg!(windows) {
            "line1\nline2\nline3"
        } else {
            "line1\r\nline2\r\nline3"
        };
        let newline = if cfg!(windows) { "\r\n" } else { "\n" };
        let expected = format!("line1{newline}### line2{newline}### line3");

        assert_eq!(prefix_line_breaks(Some(input), "###"), Some(expected));
    }

    #[test]
    fn prefix_line_breaks_should_return_none_for_none_input() {
        assert_eq!(prefix_line_breaks(None, "###"), None);
    }
}
