use std::{env, ffi::OsString};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use sha2::{Digest, Sha256};

pub fn anonymous_identity() -> String {
    let user_name = current_user_name();
    let machine_name = current_machine_name();

    anonymous_identity_from_parts(&user_name, machine_name.as_deref())
}

pub fn anonymous_identity_from_parts(user_name: &str, machine_name: Option<&str>) -> String {
    let machine_name = machine_name
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("localhost");
    let value = format!("{user_name}@{machine_name}");
    let hash = Sha256::digest(value.as_bytes());

    STANDARD.encode(hash).to_ascii_lowercase()
}

pub fn support_key() -> String {
    support_key_from_anonymous_identity(&anonymous_identity())
}

pub fn support_key_from_anonymous_identity(anonymous_identity: &str) -> String {
    anonymous_identity.chars().take(7).collect()
}

fn current_user_name() -> String {
    env_value(&["USERNAME", "USER", "LOGNAME"]).unwrap_or_default()
}

fn current_machine_name() -> Option<String> {
    hostname::get()
        .ok()
        .and_then(normalize_os_string)
        .or_else(|| env_value(&["COMPUTERNAME", "HOSTNAME"]))
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
    use super::{anonymous_identity_from_parts, support_key_from_anonymous_identity};

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
