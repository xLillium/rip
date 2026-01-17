use rip_provider_openresponses::{CreateResponseBuilder, CreateResponsePayload, ItemParam};
use serde_json::json;

#[test]
fn create_response_builder_minimal_is_valid() {
    let payload = CreateResponseBuilder::new()
        .model("gpt-4.1")
        .input_text("hi")
        .build();

    assert!(payload.errors().is_empty());
    assert_eq!(payload.body().get("model").unwrap(), "gpt-4.1");
    assert_eq!(payload.body().get("input").unwrap(), "hi");
}

#[test]
fn create_response_payload_captures_validation_errors() {
    let payload = CreateResponsePayload::new(json!("nope"));
    assert!(!payload.errors().is_empty());
}

#[test]
fn item_param_reports_validation_errors() {
    let item = ItemParam::new(json!("nope"));
    assert!(!item.errors().is_empty());
}
