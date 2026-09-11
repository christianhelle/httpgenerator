//! Helpers for redacting sensitive command-line arguments before logging them.

const FLAG: &str = "--authorization-header";
const REPLACEMENT: &str = "--authorization-header [REDACTED]";

/// Replaces `--authorization-header` argument values with a redacted marker.
///
/// This is primarily useful when echoing CLI commands or telemetry-friendly diagnostics without
/// leaking secrets into logs. Values may be quoted, attached with `=`, or written as a scheme
/// followed by a credential; arguments that follow the value are left intact.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::redact_authorization_headers;
///
/// let input =
///     "--authorization-header Bearer secret-token --base-url https://api.example.com";
///
/// assert_eq!(
///     redact_authorization_headers(input),
///     "--authorization-header [REDACTED] --base-url https://api.example.com"
/// );
/// ```
pub fn redact_authorization_headers(input: &str) -> String {
    let mut redacted = String::with_capacity(input.len());
    let mut rest = input;

    while let Some(offset) = find_flag(rest) {
        redacted.push_str(&rest[..offset]);
        let after_flag = &rest[offset + FLAG.len()..];

        match value_end(after_flag) {
            Some(end) => {
                redacted.push_str(REPLACEMENT);
                rest = &after_flag[end..];
            }
            None => {
                redacted.push_str(FLAG);
                rest = after_flag;
            }
        }
    }

    redacted.push_str(rest);
    redacted
}

/// Finds the next `--authorization-header` option, ignoring options that merely start with it.
fn find_flag(haystack: &str) -> Option<usize> {
    let lowercased = haystack.to_ascii_lowercase();
    let mut searched = 0;

    while let Some(relative) = lowercased[searched..].find(FLAG) {
        let start = searched + relative;
        let starts_an_argument = start == 0
            || haystack[..start]
                .chars()
                .next_back()
                .is_some_and(char::is_whitespace);
        let carries_a_value = haystack[start + FLAG.len()..]
            .chars()
            .next()
            .is_some_and(|character| character == '=' || character.is_whitespace());

        if starts_an_argument && carries_a_value {
            return Some(start);
        }

        searched = start + FLAG.len();
    }

    None
}

/// Returns how much of `after_flag` holds the option separator and its value.
fn value_end(after_flag: &str) -> Option<usize> {
    let separator = match after_flag.chars().next()? {
        '=' => 1,
        character if character.is_whitespace() => {
            after_flag.find(|character: char| !character.is_whitespace())?
        }
        _ => return None,
    };

    let value = &after_flag[separator..];
    let first = value.chars().next()?;

    if first.is_whitespace() {
        return None;
    }

    if matches!(first, '"' | '\'') {
        let closing = value[first.len_utf8()..]
            .find(first)
            .map(|offset| first.len_utf8() + offset + first.len_utf8());

        return Some(separator + closing.unwrap_or(value.len()));
    }

    let mut consumed = value.find(char::is_whitespace).unwrap_or(value.len());
    let remainder = &value[consumed..];

    // An authorization header is often written as a scheme and a credential, so a second word is
    // redacted too unless it starts the next option.
    if let Some(gap) = remainder.find(|character: char| !character.is_whitespace()) {
        let next_argument = &remainder[gap..];

        if !next_argument.starts_with('-') {
            consumed += gap
                + next_argument
                    .find(char::is_whitespace)
                    .unwrap_or(next_argument.len());
        }
    }

    Some(separator + consumed)
}

#[cfg(test)]
mod tests {
    use super::redact_authorization_headers;

    #[test]
    fn redacts_authorization_header_variants() {
        let inputs = [
            "--authorization-header XxxxXxxxXxxx",
            "--authorization-header \"XxxxXxxxXxxx\"",
            "--authorization-header 'XxxxXxxxXxxx'",
            "--authorization-header Bearer XxxxXxxxXxxx",
            "--authorization-header Basic XxxxXxxxXxxx",
            "--authorization-header Token XxxxXxxxXxxx",
            "--authorization-header bearer XxxxXxxxXxxx",
            "--authorization-header basic XxxxXxxxXxxx",
            "--authorization-header token XxxxXxxxXxxx",
            "--authorization-header 'Bearer XxxxXxxxXxxx'",
            "--authorization-header 'Basic XxxxXxxxXxxx'",
            "--authorization-header 'Token XxxxXxxxXxxx'",
            "--authorization-header 'bearer XxxxXxxxXxxx'",
            "--authorization-header 'basic XxxxXxxxXxxx'",
            "--authorization-header 'token XxxxXxxxXxxx'",
            "--authorization-header \"Bearer XxxxXxxxXxxx\"",
            "--authorization-header \"Basic XxxxXxxxXxxx\"",
            "--authorization-header \"Token XxxxXxxxXxxx\"",
            "--authorization-header \"bearer XxxxXxxxXxxx\"",
            "--authorization-header \"basic XxxxXxxxXxxx\"",
            "--authorization-header \"token XxxxXxxxXxxx\"",
        ];

        for input in inputs {
            assert_eq!(
                redact_authorization_headers(input),
                "--authorization-header [REDACTED]"
            );
        }
    }

    #[test]
    fn preserves_non_authorization_text() {
        let inputs = [
            "--base-url https://api.example.com",
            "--output ./output",
            "some random text",
        ];

        for input in inputs {
            assert_eq!(redact_authorization_headers(input), input);
        }
    }

    #[test]
    fn preserves_arguments_that_follow_the_redacted_value() {
        assert_eq!(
            redact_authorization_headers(
                "--authorization-header secret --base-url https://api.example.com"
            ),
            "--authorization-header [REDACTED] --base-url https://api.example.com"
        );
        assert_eq!(
            redact_authorization_headers(
                "petstore.json --authorization-header \"Bearer secret\" --output ./out"
            ),
            "petstore.json --authorization-header [REDACTED] --output ./out"
        );
    }

    #[test]
    fn redacts_quoted_values_that_contain_spaces() {
        assert_eq!(
            redact_authorization_headers("--authorization-header 'Bearer a b' --output ./out"),
            "--authorization-header [REDACTED] --output ./out"
        );
    }

    #[test]
    fn redacts_values_attached_with_an_equals_sign() {
        assert_eq!(
            redact_authorization_headers("--authorization-header=Bearer secret --output ./out"),
            "--authorization-header [REDACTED] --output ./out"
        );
    }

    #[test]
    fn leaves_similarly_named_options_untouched() {
        let input = "--authorization-header-variable-name AUTH_TOKEN";

        assert_eq!(redact_authorization_headers(input), input);
    }

    #[test]
    fn redacts_multiple_authorization_headers() {
        let input = "--authorization-header Bearer token1 --authorization-header Basic token2";
        let result = redact_authorization_headers(input);

        assert!(result.contains("[REDACTED]"));
        assert!(!result.contains("token1"));
        assert!(!result.contains("token2"));
    }
}
