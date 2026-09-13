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

        let Some(pointer) = reference.strip_prefix('#') else {
            return Ok(None);
        };

        value = root
            .pointer(pointer)
            .ok_or_else(|| reference.to_string())?;
    }

    Err(value
        .get("$ref")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string())
}
