use super::NormalizedHttpMethod;

#[test]
fn normalized_http_method_strings_match_openapi_keys() {
    assert_eq!(NormalizedHttpMethod::Get.as_str(), "get");
    assert_eq!(NormalizedHttpMethod::Patch.as_str(), "patch");
    assert_eq!(NormalizedHttpMethod::Trace.as_str(), "trace");
}
