use serde_json::Value;

/// The most `$ref`s followed for one value before the chain is treated as unresolvable.
const MAX_REFERENCE_DEPTH: usize = 32;

/// Follows the local `$ref`s of a parameter, request body or path item to the referenced value.
///
/// Returns `Ok(None)` for a reference to another file or URL. Reading merges external references
/// into the document, so one that is still external could not be resolved and was already reported
/// in the read diagnostics. Returns the failing reference when a local reference does not point
/// at a value in the document, or when references form a cycle.
pub(super) fn resolve_reference<'a>(
    root: &'a Value,
    mut value: &'a Value,
) -> Result<Option<&'a Value>, String> {
    for _ in 0..MAX_REFERENCE_DEPTH {
        let Some(reference) = value.get("$ref").and_then(Value::as_str) else {
            return Ok(Some(value));
        };

        let Some(fragment) = reference.strip_prefix('#') else {
            return Ok(None);
        };

        value = root
            .pointer(&percent_decode(fragment))
            .ok_or_else(|| reference.to_string())?;
    }

    Err(value
        .get("$ref")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string())
}

/// Decodes `%XX` escapes in a URI fragment. JSON Pointer `~0` and `~1` escapes are left for
/// [`Value::pointer`] to handle.
fn percent_decode(fragment: &str) -> String {
    if !fragment.contains('%') {
        return fragment.to_string();
    }

    let bytes = fragment.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        let escaped = (bytes[index] == b'%')
            .then(|| fragment.get(index + 1..index + 3))
            .flatten()
            .and_then(|hex| u8::from_str_radix(hex, 16).ok());
        match escaped {
            Some(byte) => {
                decoded.push(byte);
                index += 3;
            }
            None => {
                decoded.push(bytes[index]);
                index += 1;
            }
        }
    }

    String::from_utf8_lossy(&decoded).into_owned()
}
