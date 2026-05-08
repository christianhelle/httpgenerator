use httpgenerator_core::support_key;
use std::{env, time::Duration};
use unicode_width::UnicodeWidthStr;

const DEFAULT_RULE_WIDTH: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PresentationMode {
    Rich,
    Plain,
}

pub(super) fn mode_from_terminal(is_terminal: bool) -> PresentationMode {
    if is_terminal {
        PresentationMode::Rich
    } else {
        PresentationMode::Plain
    }
}

pub(super) fn render_rule(label: &str, border_codes: &[&str]) -> String {
    let label_width = text_width(label);
    let total_width = rule_width().max(label_width + 4);
    let fill_width = total_width.saturating_sub(label_width + 2);
    let left_fill = fill_width / 2;
    let right_fill = fill_width.saturating_sub(left_fill);

    format!(
        "{} {} {}",
        style(&"─".repeat(left_fill), border_codes),
        style(label, border_codes),
        style(&"─".repeat(right_fill), border_codes)
    )
}

pub(super) fn render_panel(
    content_plain: &str,
    content_styled: &str,
    border_codes: &[&str],
) -> String {
    let inner_width = text_width(content_plain);
    let top = style(&format!("╭{}╮", "─".repeat(inner_width + 2)), border_codes);
    let middle = format!(
        "{} {}{} {}",
        style("│", border_codes),
        content_styled,
        " ".repeat(inner_width.saturating_sub(text_width(content_plain))),
        style("│", border_codes)
    );
    let bottom = style(&format!("╰{}╯", "─".repeat(inner_width + 2)), border_codes);

    format!("{top}\n{middle}\n{bottom}")
}

pub(super) fn table_row(
    left_plain: &str,
    left_styled: &str,
    right_plain: &str,
    right_styled: &str,
    left_width: usize,
    right_width: usize,
) -> String {
    format!(
        "{} {}{} {} {}{} {}\n",
        style("│", &["32"]),
        left_styled,
        " ".repeat(left_width.saturating_sub(text_width(left_plain))),
        style("│", &["32"]),
        right_styled,
        " ".repeat(right_width.saturating_sub(text_width(right_plain))),
        style("│", &["32"])
    )
}

pub(super) fn support_key_line(no_logging: bool) -> String {
    if no_logging {
        "Unavailable when logging is disabled".to_string()
    } else {
        support_key()
    }
}

pub(super) fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let milliseconds = duration.subsec_millis();

    format!("{minutes:02}:{seconds:02}.{milliseconds:03}")
}

pub(super) fn style(text: &str, codes: &[&str]) -> String {
    format!("\u{1b}[{}m{text}\u{1b}[0m", codes.join(";"))
}

pub(super) fn text_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

fn rule_width() -> usize {
    env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|width| *width >= 20)
        .unwrap_or(DEFAULT_RULE_WIDTH)
}

#[cfg(test)]
mod tests {
    use super::{format_duration, support_key_line};
    use std::time::Duration;

    #[test]
    fn support_key_line_uses_runtime_support_key_when_logging_is_enabled() {
        let runtime_support_key = support_key_line(false);

        assert_eq!(runtime_support_key.len(), 7);
        assert_ne!(runtime_support_key, "Unavailable when logging is disabled");
    }

    #[test]
    fn support_key_line_hides_support_key_when_logging_is_disabled() {
        assert_eq!(
            support_key_line(true),
            "Unavailable when logging is disabled"
        );
    }

    #[test]
    fn format_duration_matches_runtime_display_shape() {
        assert_eq!(format_duration(Duration::from_millis(8_123)), "00:08.123");
        assert_eq!(format_duration(Duration::from_millis(83_456)), "01:23.456");
    }
}
