//! Helpers for producing anonymous support identifiers.

use std::{env, ffi::OsString};

use crate::digest::{base64_encode, sha256};

/// Returns a stable anonymous identity derived from the current user and machine.
///
/// The value is SHA-256 hashed and Base64 encoded so support workflows can correlate reports
/// without storing the raw user or host name.
pub fn anonymous_identity() -> String {
    let user_name = current_user_name();
    let machine_name = current_machine_name();

    anonymous_identity_from_parts(&user_name, machine_name.as_deref())
}

/// Returns the anonymous support identity for explicit user and machine parts.
///
/// Empty or missing machine names fall back to `"localhost"` for parity with the legacy .NET
/// implementation.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::anonymous_identity_from_parts;
///
/// assert_eq!(
///     anonymous_identity_from_parts("alice", Some("build-agent")),
///     "prihjx2hffzjfsy4vly5/8ynzks7bznfs3wk4b+e+xm="
/// );
/// ```
pub fn anonymous_identity_from_parts(user_name: &str, machine_name: Option<&str>) -> String {
    let machine_name = machine_name
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("localhost");
    let value = format!("{user_name}@{machine_name}");
    let hash = sha256(value.as_bytes());

    base64_encode(&hash).to_ascii_lowercase()
}

/// Returns the short support key for the current anonymous identity.
///
/// This is a convenience wrapper around [`support_key_from_anonymous_identity`] that uses
/// [`anonymous_identity`].
pub fn support_key() -> String {
    support_key_from_anonymous_identity(&anonymous_identity())
}

/// Returns the short support key associated with an anonymous identity.
///
/// The support key is simply the first seven characters of the full anonymous identity.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::support_key_from_anonymous_identity;
///
/// assert_eq!(
///     support_key_from_anonymous_identity("prihjx2hffzjfsy4vly5/8ynzks7bznfs3wk4b+e+xm="),
///     "prihjx2"
/// );
/// ```
pub fn support_key_from_anonymous_identity(anonymous_identity: &str) -> String {
    anonymous_identity.chars().take(7).collect()
}

fn current_user_name() -> String {
    env_value(&["USERNAME", "USER", "LOGNAME"]).unwrap_or_default()
}

fn current_machine_name() -> Option<String> {
    os_host_name()
        .and_then(normalize_os_string)
        .or_else(|| env_value(&["COMPUTERNAME", "HOSTNAME"]))
}

/// Queries the live host name from the operating system, as the `hostname` crate did.
#[cfg(windows)]
fn os_host_name() -> Option<OsString> {
    use std::os::windows::ffi::OsStringExt;

    const COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME: i32 = 5;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetComputerNameExW(name_type: i32, buffer: *mut u16, size: *mut u32) -> i32;
    }

    let mut size = 0;
    // SAFETY: a null buffer with a zero size only asks for the required buffer length.
    unsafe {
        GetComputerNameExW(COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME, std::ptr::null_mut(), &mut size)
    };

    let mut buffer = vec![0u16; size as usize];
    // SAFETY: `buffer` holds `size` UTF-16 units, and `size` tells the call how many it may write.
    let succeeded = unsafe {
        GetComputerNameExW(COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME, buffer.as_mut_ptr(), &mut size)
    } != 0;

    succeeded.then(|| OsString::from_wide(&buffer[..size as usize]))
}

/// Queries the live host name from the operating system, as the `hostname` crate did.
#[cfg(unix)]
fn os_host_name() -> Option<OsString> {
    use std::{
        ffi::{c_char, c_int},
        os::unix::ffi::OsStringExt,
    };

    unsafe extern "C" {
        fn gethostname(name: *mut c_char, len: usize) -> c_int;
    }

    // POSIX caps host names at 255 bytes; one more byte leaves room for the terminating nul.
    let mut buffer = vec![0u8; 256];
    // SAFETY: the call writes at most `buffer.len() - 1` bytes into `buffer`.
    let succeeded = unsafe { gethostname(buffer.as_mut_ptr().cast(), buffer.len() - 1) } == 0;

    succeeded.then(|| {
        let end = buffer.iter().position(|&byte| byte == 0).unwrap_or(buffer.len());
        buffer.truncate(end);
        OsString::from_vec(buffer)
    })
}

#[cfg(not(any(windows, unix)))]
fn os_host_name() -> Option<OsString> {
    None
}

fn env_value(keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| env::var_os(key).and_then(normalize_os_string))
}

fn normalize_os_string(value: OsString) -> Option<String> {
    let value = value.to_string_lossy();
    let value = value.trim();

    (!value.is_empty()).then(|| value.to_string())
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::{anonymous_identity_from_parts, normalize_os_string, support_key_from_anonymous_identity};

    #[test]
    fn machine_names_are_trimmed_and_blank_names_are_ignored() {
        assert_eq!(
            normalize_os_string(OsString::from("  build-agent\n")),
            Some("build-agent".to_string())
        );
        assert_eq!(normalize_os_string(OsString::from(" \n")), None);
    }

    #[test]
    fn anonymous_identity_matches_dotnet_sha256_base64_lowercase() {
        let identity = anonymous_identity_from_parts("alice", Some("build-agent"));

        assert_eq!(identity, "prihjx2hffzjfsy4vly5/8ynzks7bznfs3wk4b+e+xm=");
        assert_eq!(identity.len(), 44);
        assert!(identity.ends_with('='));
        assert!(identity.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || matches!(character, '+' | '/' | '=')
        }));
    }

    #[test]
    fn anonymous_identity_falls_back_to_localhost_when_machine_name_is_missing() {
        let expected = "o22kzws2q0n0j9qajmfa/dm8puf5ilfqxfxdv4c49so=";

        assert_eq!(anonymous_identity_from_parts("octocat", None), expected);
        assert_eq!(
            anonymous_identity_from_parts("octocat", Some("   ")),
            expected
        );
        assert_eq!(
            anonymous_identity_from_parts("octocat", Some("localhost")),
            expected
        );
    }

    #[test]
    fn support_key_uses_first_seven_characters_of_anonymous_identity() {
        let anonymous_identity = anonymous_identity_from_parts("alice", Some("build-agent"));
        let support_key = support_key_from_anonymous_identity(&anonymous_identity);

        assert_eq!(support_key, "prihjx2");
        assert_eq!(support_key.len(), 7);
    }
}
