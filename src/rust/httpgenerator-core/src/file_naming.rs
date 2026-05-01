use std::{collections::HashSet, path::Path};

pub fn unique_filename(filename: &str, seen: &mut HashSet<String>) -> String {
    if seen.insert(filename.to_ascii_lowercase()) {
        return filename.to_string();
    }

    let path = Path::new(filename);
    let name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(filename);
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| format!(".{ext}"))
        .unwrap_or_default();

    let mut counter = 2;
    loop {
        let candidate = format!("{name}_{counter}{extension}");
        if seen.insert(candidate.to_ascii_lowercase()) {
            return candidate;
        }

        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::unique_filename;

    #[test]
    fn keeps_first_filename() {
        let mut seen = HashSet::new();

        let filename = unique_filename("DeletePet.http", &mut seen);

        assert_eq!(filename, "DeletePet.http");
    }

    #[test]
    fn appends_numeric_suffix_for_duplicates() {
        let mut seen = HashSet::new();

        assert_eq!(
            unique_filename("DeletePet.http", &mut seen),
            "DeletePet.http"
        );
        assert_eq!(
            unique_filename("DeletePet.http", &mut seen),
            "DeletePet_2.http"
        );
        assert_eq!(
            unique_filename("deletepet.http", &mut seen),
            "deletepet_3.http"
        );
    }
}
