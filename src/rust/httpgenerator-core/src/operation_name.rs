use crate::string_extensions::{
    capitalize_first_character, convert_kebab_case_to_pascal_case, convert_route_to_camel_case,
    convert_spaces_to_pascal_case, prefix,
};

pub fn generate_operation_name(
    http_method: &str,
    path: &str,
    operation_id: Option<&str>,
) -> String {
    let operation_name = operation_id
        .filter(|value| !value.trim().is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{http_method}_{path}"));

    let method_prefix = capitalize_first_character(&http_method.to_ascii_lowercase());
    let operation_name = capitalize_first_character(&operation_name);
    let operation_name = convert_kebab_case_to_pascal_case(&operation_name);
    let operation_name = convert_route_to_camel_case(&operation_name);
    let operation_name = convert_spaces_to_pascal_case(&operation_name);

    prefix(&operation_name, &method_prefix)
}

#[cfg(test)]
mod tests {
    use super::generate_operation_name;

    #[test]
    fn prefers_operation_id_when_present() {
        let name = generate_operation_name("get", "/pet/find-by-status", Some("find-pets"));

        assert_eq!(name, "GetFindPets");
    }

    #[test]
    fn avoids_duplicate_http_method_prefix() {
        let name = generate_operation_name("get", "/pet", Some("GetPet"));

        assert_eq!(name, "GetPet");
    }

    #[test]
    fn falls_back_to_path_when_operation_id_is_missing() {
        let name = generate_operation_name("post", "/pet/store", None);

        assert_eq!(name, "Post_PetStore");
    }
}
