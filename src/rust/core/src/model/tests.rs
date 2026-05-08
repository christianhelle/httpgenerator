use super::{GeneratorSettings, OutputType};

#[test]
fn generator_settings_defaults_match_current_tool() {
    let settings = GeneratorSettings::default();

    assert_eq!(settings.authorization_header, None);
    assert_eq!(settings.authorization_header_variable_name, "authorization");
    assert_eq!(settings.content_type, "application/json");
    assert_eq!(settings.output_type, OutputType::OneRequestPerFile);
    assert_eq!(settings.timeout, 120);
    assert!(settings.custom_headers.is_empty());
    assert!(!settings.skip_headers);
}
