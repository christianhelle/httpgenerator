pub(super) fn push_line(content: &mut String, line: &str) {
    content.push_str(line);
    content.push_str(newline());
}

pub(super) fn push_blank_line(content: &mut String) {
    content.push_str(newline());
}

fn newline() -> &'static str {
    if cfg!(windows) { "\r\n" } else { "\n" }
}
