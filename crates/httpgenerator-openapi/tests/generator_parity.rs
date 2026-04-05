use std::path::PathBuf;

use httpgenerator_core::{GeneratorSettings, OutputType, generate_http_files};
use httpgenerator_openapi::load_and_normalize_document;

fn petstore_input() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("test")
        .join("OpenAPI")
        .join("v3.0")
        .join("petstore.json")
        .to_string_lossy()
        .into_owned()
}

fn petstore_settings() -> GeneratorSettings {
    GeneratorSettings {
        open_api_path: petstore_input(),
        ..GeneratorSettings::default()
    }
}

fn newline() -> &'static str {
    if cfg!(windows) { "\r\n" } else { "\n" }
}

#[test]
fn petstore_renders_expected_one_request_per_file_outputs() {
    let document = load_and_normalize_document(&petstore_input()).unwrap();

    let result = generate_http_files(&petstore_settings(), &document);

    assert_eq!(result.files.len(), 19);
    assert_eq!(result.files[0].filename, "PutUpdatePet.http");
    assert!(
        result
            .files
            .iter()
            .any(|file| file.filename == "GetFindPetsByStatus.http")
    );
    assert!(
        result
            .files
            .iter()
            .any(|file| file.filename == "PostCreateUsersWithListInput.http")
    );
    assert!(
        result
            .files
            .iter()
            .any(|file| file.filename == "PutUpdateUser.http")
    );
}

#[test]
fn petstore_renders_expected_one_file_content() {
    let document = load_and_normalize_document(&petstore_input()).unwrap();
    let settings = GeneratorSettings {
        output_type: OutputType::OneFile,
        ..petstore_settings()
    };

    let result = generate_http_files(&settings, &document);
    let content = &result.files[0].content;

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].filename, "Requests.http");
    assert!(content.starts_with(&format!(
        "@baseUrl = /api/v3{nl}@contentType = application/json{nl}{nl}",
        nl = newline()
    )));
    assert!(content.contains("### Request: PUT /pet"));
    assert!(content.contains("### Summary: Update an existing pet"));
    assert!(content.contains("@GetFindPetsByStatus_status = str"));
    assert!(
        content.contains("GET {{baseUrl}}/pet/findByStatus?status={{GetFindPetsByStatus_status}}")
    );
    assert!(content.contains("\"category\": {\"property\": \"value\"}"));
    assert!(content.contains(&format!(
        "[{nl}  \"item1\",{nl}  \"item2\"{nl}]",
        nl = newline()
    )));
}

#[test]
fn petstore_renders_expected_one_file_per_tag_outputs() {
    let document = load_and_normalize_document(&petstore_input()).unwrap();
    let settings = GeneratorSettings {
        output_type: OutputType::OneFilePerTag,
        ..petstore_settings()
    };

    let result = generate_http_files(&settings, &document);

    assert_eq!(result.files.len(), 3);
    assert_eq!(result.files[0].filename, "Pet.http");
    assert_eq!(result.files[1].filename, "Store.http");
    assert_eq!(result.files[2].filename, "User.http");
    assert!(result.files[0].content.contains("PUT {{baseUrl}}/pet"));
    assert!(
        result.files[1]
            .content
            .contains("POST {{baseUrl}}/store/order")
    );
    assert!(result.files[2]
        .content
        .contains("GET {{baseUrl}}/user/login?username={{GetLoginUser_username}}&password={{GetLoginUser_password}}"));
}

#[test]
fn petstore_preserves_skip_headers_and_base_url_override_quirks() {
    let document = load_and_normalize_document(&petstore_input()).unwrap();

    let skip_header_result = generate_http_files(
        &GeneratorSettings {
            output_type: OutputType::OneFile,
            skip_headers: true,
            ..petstore_settings()
        },
        &document,
    );
    let override_result = generate_http_files(
        &GeneratorSettings {
            output_type: OutputType::OneFile,
            base_url: Some("https://api.example.com".to_string()),
            ..petstore_settings()
        },
        &document,
    );

    let skipped = &skip_header_result.files[0].content;
    let overridden = &override_result.files[0].content;

    assert!(!skipped.contains("@baseUrl = "));
    assert!(!skipped.contains("@contentType = "));
    assert!(skipped.contains("PUT {{baseUrl}}/pet"));
    assert!(skipped.contains("Content-Type: {{contentType}}"));
    assert!(overridden.starts_with(&format!(
        "@baseUrl = https://api.example.com/api/v3{nl}@contentType = application/json{nl}{nl}",
        nl = newline()
    )));
}
