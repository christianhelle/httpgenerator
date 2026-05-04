use regex::RegexBuilder;

const REPLACEMENT: &str = "--authorization-header [REDACTED]";
const PATTERNS: &[&str] = &[
    r#"--authorization-header "[^ ]+ [^ ]+""#,
    r#"--authorization-header '[^ ]+ [^ ]+'"#,
    r#"--authorization-header [^ ]+ [^ ]+"#,
    r#"--authorization-header "[^ ]+""#,
    r#"--authorization-header '[^ ]+'"#,
    r#"--authorization-header [^ ]+"#,
];

pub fn redact_authorization_headers(input: &str) -> String {
    PATTERNS.iter().fold(input.to_string(), |current, pattern| {
        RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()
            .expect("authorization header redaction regex should compile")
            .replace_all(&current, REPLACEMENT)
            .to_string()
    })
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
    fn redacts_multiple_authorization_headers() {
        let input = "--authorization-header Bearer token1 --authorization-header Basic token2";
        let result = redact_authorization_headers(input);

        assert!(result.contains("[REDACTED]"));
        assert!(!result.contains("token1"));
        assert!(!result.contains("token2"));
    }
}
