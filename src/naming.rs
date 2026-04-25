pub fn operation_name(path: &str, http_method: &str, operation_id: Option<&str>) -> String {
    let mut name = operation_id
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("{http_method}_{path}"));

    name = capitalize_first(&name);
    name = kebab_to_pascal(&name);
    name = route_to_camel(&name);
    name = spaces_to_pascal(&name);
    prefix(&name, &capitalize_first(&http_method.to_lowercase()))
}

pub fn capitalize_first(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().collect::<String>() + chars.as_str()
}

fn kebab_to_pascal(value: &str) -> String {
    value
        .split('-')
        .filter(|part| !part.is_empty())
        .map(|part| capitalize_first(part).replace('.', "_"))
        .collect()
}

fn route_to_camel(value: &str) -> String {
    value
        .split('/')
        .filter(|part| !part.is_empty())
        .enumerate()
        .map(|(index, part)| {
            if index == 0 {
                part.to_string()
            } else {
                capitalize_first(part)
            }
        })
        .collect()
}

fn spaces_to_pascal(value: &str) -> String {
    value
        .split(' ')
        .filter(|part| !part.is_empty())
        .map(capitalize_first)
        .collect()
}

fn prefix(value: &str, prefix: &str) -> String {
    if value.starts_with(prefix) {
        value.to_string()
    } else {
        format!("{prefix}{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_operation_id_with_verb() {
        assert_eq!(
            operation_name("/pet/{petId}", "Get", Some("pet-by-id")),
            "GetPetById"
        );
    }

    #[test]
    fn does_not_double_prefix() {
        assert_eq!(operation_name("/pet", "Post", Some("post pet")), "PostPet");
    }
}
