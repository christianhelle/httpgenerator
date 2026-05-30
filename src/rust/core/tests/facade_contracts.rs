use std::path::PathBuf;

use httpgenerator_core::{generator, model, normalized, openapi};

fn petstore_input() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("test")
        .join("OpenAPI")
        .join("v3.0")
        .join("petstore.json")
        .to_string_lossy()
        .into_owned()
}

#[test]
fn facade_modules_expose_expected_public_types_and_signatures() {
    let _: fn(
        &model::GeneratorSettings,
        &normalized::NormalizedOpenApiDocument,
    ) -> model::GeneratorResult = generator::generate_http_files;
    let _: fn(
        &str,
        openapi::LoadOptions,
    ) -> Result<
        normalized::NormalizedOpenApiDocument,
        openapi::OpenApiDocumentNormalizationError,
    > = openapi::load_and_normalize_document;

    let settings = model::GeneratorSettings {
        open_api_path: "petstore.json".to_string(),
        output_type: model::OutputType::OneRequestPerFile,
        ..Default::default()
    };

    let document = normalized::NormalizedOpenApiDocument {
        specification_version: normalized::NormalizedSpecificationVersion::OpenApi30,
        servers: vec![normalized::NormalizedServer {
            url: "https://example.com".to_string(),
        }],
        operations: vec![normalized::NormalizedOperation {
            path: "/pets".to_string(),
            method: normalized::NormalizedHttpMethod::Get,
            operation_id: Some("listPets".to_string()),
            summary: Some("List pets".to_string()),
            description: None,
            tags: vec!["Pets".to_string()],
            parameters: vec![normalized::NormalizedParameter::Inline(
                normalized::NormalizedInlineParameter {
                    name: "limit".to_string(),
                    location: normalized::NormalizedParameterLocation::Query,
                    description: None,
                    required: false,
                    schema: Some(normalized::NormalizedSchema {
                        types: vec![normalized::NormalizedSchemaType::Integer],
                        ..Default::default()
                    }),
                },
            )],
            request_body: None,
        }],
    };

    let result = generator::generate_http_files(&settings, &document);

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].filename, "GetListPets.http");
    assert!(
        result.files[0]
            .content
            .contains("GET {{baseUrl}}/pets?limit={{limit}}")
    );
}

#[test]
fn openapi_and_generator_facades_stay_compatible_for_petstore() {
    let input = petstore_input();
    let document =
        openapi::load_and_normalize_document(&input, openapi::LoadOptions::default()).unwrap();
    let settings = model::GeneratorSettings {
        open_api_path: input,
        ..Default::default()
    };

    let result = generator::generate_http_files(&settings, &document);

    assert_eq!(
        document.specification_version,
        normalized::NormalizedSpecificationVersion::OpenApi30
    );
    assert_eq!(result.files.len(), 19);
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
}
