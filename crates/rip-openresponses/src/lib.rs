use jsonschema::JSONSchema;
use once_cell::sync::Lazy;
use serde_json::Value;

const OPENAPI_URI: &str = "urn:openresponses:openapi";

static OPENAPI: Lazy<Value> = Lazy::new(|| {
    let raw = include_str!("../../../schemas/openresponses/openapi.json");
    serde_json::from_str(raw).expect("openapi.json valid")
});

static STREAM_EVENT_TYPES: Lazy<Vec<String>> = Lazy::new(|| {
    let raw = include_str!("../../../schemas/openresponses/streaming_event_types.json");
    serde_json::from_str(raw).expect("streaming_event_types.json valid")
});

static STREAM_SCHEMA: Lazy<Value> =
    Lazy::new(|| extract_streaming_schema().expect("streaming event schema not found"));

static RESPONSE_SCHEMA: Lazy<Value> = Lazy::new(|| {
    extract_component_schema("ResponseResource").expect("ResponseResource schema not found")
});

static CREATE_RESPONSE_SCHEMA: Lazy<Value> = Lazy::new(|| {
    extract_component_schema("CreateResponseBody").expect("CreateResponseBody schema not found")
});

static TOOL_PARAM_SCHEMA: Lazy<Value> = Lazy::new(|| {
    extract_component_schema("ResponsesToolParam").expect("ResponsesToolParam schema not found")
});

static TOOL_CHOICE_SCHEMA: Lazy<Value> = Lazy::new(|| {
    extract_component_schema("ToolChoiceParam").expect("ToolChoiceParam schema not found")
});

static ITEM_PARAM_SCHEMA: Lazy<Value> = Lazy::new(|| {
    let mut schema = extract_component_schema("ItemParam").expect("ItemParam schema not found");
    if let Some(obj) = schema.as_object_mut() {
        obj.remove("discriminator");
    }
    schema
});

static STREAM_VALIDATOR: Lazy<JSONSchema> = Lazy::new(|| {
    JSONSchema::options()
        .with_document(OPENAPI_URI.to_string(), OPENAPI.clone())
        .compile(&STREAM_SCHEMA)
        .expect("compile streaming schema")
});

static RESPONSE_VALIDATOR: Lazy<JSONSchema> = Lazy::new(|| {
    JSONSchema::options()
        .with_document(OPENAPI_URI.to_string(), OPENAPI.clone())
        .compile(&RESPONSE_SCHEMA)
        .expect("compile response schema")
});

static CREATE_RESPONSE_VALIDATOR: Lazy<JSONSchema> = Lazy::new(|| {
    JSONSchema::options()
        .with_document(OPENAPI_URI.to_string(), OPENAPI.clone())
        .compile(&CREATE_RESPONSE_SCHEMA)
        .expect("compile create response schema")
});

// jsonschema rejects ItemParam oneOf with multiple message roles; validate each variant directly.
static ITEM_PARAM_VARIANTS: Lazy<Vec<(String, JSONSchema)>> =
    Lazy::new(|| build_component_variant_validators("ItemParam"));

const TOOL_CHOICE_VALUES: [&str; 3] = ["auto", "required", "none"];
const COMPUTER_ENVIRONMENTS: [&str; 4] = ["windows", "mac", "linux", "browser"];

pub fn openapi() -> &'static Value {
    &OPENAPI
}

pub fn allowed_stream_event_types() -> &'static [String] {
    &STREAM_EVENT_TYPES
}

pub fn streaming_event_schema() -> &'static Value {
    &STREAM_SCHEMA
}

pub fn response_resource_schema() -> &'static Value {
    &RESPONSE_SCHEMA
}

pub fn create_response_body_schema() -> &'static Value {
    &CREATE_RESPONSE_SCHEMA
}

pub fn tool_param_schema() -> &'static Value {
    &TOOL_PARAM_SCHEMA
}

pub fn tool_choice_param_schema() -> &'static Value {
    &TOOL_CHOICE_SCHEMA
}

pub fn item_param_schema() -> &'static Value {
    &ITEM_PARAM_SCHEMA
}

pub fn validate_stream_event(value: &Value) -> Result<(), Vec<String>> {
    match STREAM_VALIDATOR.validate(value) {
        Ok(_) => Ok(()),
        Err(errors) => Err(errors.map(|e| e.to_string()).collect()),
    }
}

pub fn validate_response_resource(value: &Value) -> Result<(), Vec<String>> {
    match RESPONSE_VALIDATOR.validate(value) {
        Ok(_) => Ok(()),
        Err(errors) => Err(errors.map(|e| e.to_string()).collect()),
    }
}

pub fn validate_create_response_body(value: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let mut stripped = value.clone();
    if let Value::Object(map) = &mut stripped {
        // Validate tool fields separately; jsonschema rejects the oneOf tool variants.
        if let Some(tools) = map.remove("tools") {
            match tools {
                Value::Null => {}
                Value::Array(items) => {
                    for (idx, item) in items.iter().enumerate() {
                        if let Err(errs) = validate_responses_tool_param(item) {
                            errors
                                .extend(errs.into_iter().map(|err| format!("tools[{idx}]: {err}")));
                        }
                    }
                }
                _ => errors.push("tools must be an array or null".to_string()),
            }
        }
        if let Some(choice) = map.remove("tool_choice") {
            match choice {
                Value::Null => {}
                _ => {
                    if let Err(errs) = validate_tool_choice_param(&choice) {
                        errors.extend(errs.into_iter().map(|err| format!("tool_choice: {err}")));
                    }
                }
            }
        }
    }

    if let Err(errs) = CREATE_RESPONSE_VALIDATOR.validate(&stripped) {
        errors.extend(errs.map(|e| e.to_string()));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_responses_tool_param(value: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let map = match value.as_object() {
        Some(map) => map,
        None => return Err(vec!["ResponsesToolParam must be an object".to_string()]),
    };

    let tool_type = match map.get("type").and_then(|value| value.as_str()) {
        Some(tool_type) => tool_type,
        None => {
            errors.push("ResponsesToolParam.type must be a string".to_string());
            return Err(errors);
        }
    };

    match tool_type {
        "function" => {
            require_string_field(map, "name", "ResponsesToolParam(function)", &mut errors);
        }
        "custom" => {
            require_string_field(map, "name", "ResponsesToolParam(custom)", &mut errors);
        }
        "mcp" => {
            require_string_field(map, "server_label", "ResponsesToolParam(mcp)", &mut errors);
        }
        "file_search" => {
            let context = "ResponsesToolParam(file_search)";
            match require_field(map, "vector_store_ids", context, &mut errors) {
                Some(Value::Array(items)) => {
                    if items.is_empty() {
                        errors.push(format!("{context}.vector_store_ids must not be empty"));
                    }
                    for (idx, item) in items.iter().enumerate() {
                        if !item.is_string() {
                            errors.push(format!(
                                "{context}.vector_store_ids[{idx}] must be a string"
                            ));
                        }
                    }
                }
                Some(_) => errors.push(format!("{context}.vector_store_ids must be an array")),
                None => {}
            }
        }
        "code_interpreter" => {
            let context = "ResponsesToolParam(code_interpreter)";
            match require_field(map, "container", context, &mut errors) {
                Some(Value::String(_)) => {}
                Some(Value::Object(container)) => {
                    match container.get("type").and_then(|value| value.as_str()) {
                        Some("auto") => {}
                        Some(_) => {
                            errors.push(format!("{context}.container.type must be \"auto\""))
                        }
                        None => errors.push(format!("{context}.container.type must be a string")),
                    }
                }
                Some(_) => errors.push(format!("{context}.container must be a string or object")),
                None => {}
            }
        }
        "computer-preview" | "computer_use_preview" => {
            let context = format!("ResponsesToolParam({tool_type})");
            require_positive_integer_field(map, "display_width", &context, &mut errors);
            require_positive_integer_field(map, "display_height", &context, &mut errors);
            if let Some(value) = require_field(map, "environment", &context, &mut errors) {
                match value.as_str() {
                    Some(env) => {
                        if !COMPUTER_ENVIRONMENTS.contains(&env) {
                            errors.push(format!(
                                "{context}.environment must be one of {}",
                                COMPUTER_ENVIRONMENTS.join(", ")
                            ));
                        }
                    }
                    None => errors.push(format!("{context}.environment must be a string")),
                }
            }
        }
        "web_search"
        | "web_search_2025_08_26"
        | "web_search_ga"
        | "web_search_preview"
        | "web_search_preview_2025_03_11"
        | "image_generation"
        | "local_shell"
        | "shell"
        | "apply_patch" => {}
        other => errors.push(format!(
            "ResponsesToolParam.type has unsupported value \"{other}\""
        )),
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_tool_choice_param(value: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    match value {
        Value::String(choice) => {
            if !TOOL_CHOICE_VALUES.contains(&choice.as_str()) {
                errors.push(format!(
                    "ToolChoiceParam must be one of {}",
                    TOOL_CHOICE_VALUES.join(", ")
                ));
            }
        }
        Value::Object(map) => {
            let choice_type = match map.get("type").and_then(|value| value.as_str()) {
                Some(choice_type) => choice_type,
                None => {
                    errors.push("ToolChoiceParam.type must be a string".to_string());
                    return Err(errors);
                }
            };
            if choice_type == "allowed_tools" {
                let context = "ToolChoiceParam(allowed_tools)";
                match require_field(map, "tools", context, &mut errors) {
                    Some(Value::Array(items)) => {
                        if items.is_empty() {
                            errors.push(format!("{context}.tools must not be empty"));
                        }
                        for (idx, item) in items.iter().enumerate() {
                            match validate_specific_tool_choice(item) {
                                Ok(_) => {}
                                Err(errs) => errors.extend(
                                    errs.into_iter()
                                        .map(|err| format!("{context}.tools[{idx}]: {err}")),
                                ),
                            }
                        }
                    }
                    Some(_) => errors.push(format!("{context}.tools must be an array")),
                    None => {}
                }
                if let Some(mode) = map.get("mode") {
                    match mode.as_str() {
                        Some(value) => {
                            if !TOOL_CHOICE_VALUES.contains(&value) {
                                errors.push(format!(
                                    "{context}.mode must be one of {}",
                                    TOOL_CHOICE_VALUES.join(", ")
                                ));
                            }
                        }
                        None => errors.push(format!("{context}.mode must be a string")),
                    }
                }
            } else if let Err(errs) = validate_specific_tool_choice(value) {
                errors.extend(errs);
            }
        }
        _ => errors.push("ToolChoiceParam must be a string or object".to_string()),
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_specific_tool_choice_param(value: &Value) -> Result<(), Vec<String>> {
    validate_specific_tool_choice(value)
}

pub fn validate_item_param(value: &Value) -> Result<(), Vec<String>> {
    validate_variants("ItemParam", value, &ITEM_PARAM_VARIANTS)
}

fn validate_variants(
    label: &str,
    value: &Value,
    variants: &[(String, JSONSchema)],
) -> Result<(), Vec<String>> {
    let mut matches = 0;
    let mut errors = Vec::new();
    for (name, validator) in variants.iter() {
        match validator.validate(value) {
            Ok(_) => matches += 1,
            Err(errs) => errors.extend(errs.map(|e| format!("{name}: {e}"))),
        }
    }
    if matches == 1 {
        Ok(())
    } else if matches == 0 {
        Err(errors)
    } else {
        Err(vec![format!(
            "{label} matches multiple schemas ({matches})"
        )])
    }
}

fn require_field<'a>(
    map: &'a serde_json::Map<String, Value>,
    field: &str,
    context: &str,
    errors: &mut Vec<String>,
) -> Option<&'a Value> {
    match map.get(field) {
        Some(value) => Some(value),
        None => {
            errors.push(format!("{context} missing required field `{field}`"));
            None
        }
    }
}

fn require_string_field(
    map: &serde_json::Map<String, Value>,
    field: &str,
    context: &str,
    errors: &mut Vec<String>,
) {
    match require_field(map, field, context, errors) {
        Some(Value::String(_)) => {}
        Some(_) => errors.push(format!("{context}.{field} must be a string")),
        None => {}
    }
}

fn require_positive_integer_field(
    map: &serde_json::Map<String, Value>,
    field: &str,
    context: &str,
    errors: &mut Vec<String>,
) {
    match require_field(map, field, context, errors) {
        Some(Value::Number(num)) => {
            let ok = match (num.as_i64(), num.as_u64()) {
                (Some(value), _) => value > 0,
                (None, Some(value)) => value > 0,
                (None, None) => false,
            };
            if !ok {
                errors.push(format!("{context}.{field} must be a positive integer"));
            }
        }
        Some(_) => errors.push(format!("{context}.{field} must be an integer")),
        None => {}
    }
}

fn validate_specific_tool_choice(value: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let map = match value.as_object() {
        Some(map) => map,
        None => return Err(vec!["SpecificToolChoiceParam must be an object".to_string()]),
    };
    let tool_type = match map.get("type").and_then(|value| value.as_str()) {
        Some(tool_type) => tool_type,
        None => {
            errors.push("SpecificToolChoiceParam.type must be a string".to_string());
            return Err(errors);
        }
    };

    match tool_type {
        "function" => require_string_field(
            map,
            "name",
            "SpecificToolChoiceParam(function)",
            &mut errors,
        ),
        "custom" => {
            require_string_field(map, "name", "SpecificToolChoiceParam(custom)", &mut errors)
        }
        "mcp" => require_string_field(
            map,
            "server_label",
            "SpecificToolChoiceParam(mcp)",
            &mut errors,
        ),
        "file_search"
        | "web_search"
        | "web_search_preview"
        | "image_generation"
        | "computer-preview"
        | "computer_use_preview"
        | "code_interpreter"
        | "local_shell"
        | "shell"
        | "apply_patch" => {}
        other => errors.push(format!(
            "SpecificToolChoiceParam.type has unsupported value \"{other}\""
        )),
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn component_oneof_refs(component_name: &str) -> Vec<String> {
    let schema = extract_component_schema(component_name)
        .unwrap_or_else(|| panic!("schema missing: {component_name}"));
    schema
        .get("oneOf")
        .and_then(|value| value.as_array())
        .expect("oneOf variants")
        .iter()
        .filter_map(|item| item.get("$ref").and_then(|value| value.as_str()))
        .filter_map(|ref_name| ref_name.rsplit('/').next().map(|name| name.to_string()))
        .collect()
}

fn compile_component_validators(names: Vec<String>) -> Vec<(String, JSONSchema)> {
    names
        .into_iter()
        .map(|name| {
            let schema = serde_json::json!({
                "$ref": format!("{OPENAPI_URI}#/components/schemas/{name}")
            });
            let validator = JSONSchema::options()
                .with_document(OPENAPI_URI.to_string(), OPENAPI.clone())
                .compile(&schema)
                .expect("compile schema variant");
            (name, validator)
        })
        .collect()
}

fn build_component_variant_validators(component_name: &str) -> Vec<(String, JSONSchema)> {
    let variants = component_oneof_refs(component_name);
    compile_component_validators(variants)
}

fn extract_streaming_schema() -> Option<Value> {
    let pointer = "/paths/~1responses/post/responses/200/content/text~1event-stream/schema";
    OPENAPI.pointer(pointer).cloned()
}

fn extract_component_schema(name: &str) -> Option<Value> {
    let pointer = format!("/components/schemas/{name}");
    OPENAPI.pointer(&pointer).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_list_is_non_empty() {
        assert!(!allowed_stream_event_types().is_empty());
    }

    #[test]
    fn openapi_loads() {
        assert!(openapi().get("openapi").is_some());
    }

    #[test]
    fn streaming_schema_is_present() {
        assert!(streaming_event_schema().get("oneOf").is_some());
    }

    #[test]
    fn response_schema_is_present() {
        assert!(response_resource_schema().get("properties").is_some());
    }

    #[test]
    fn create_response_schema_is_present() {
        assert!(create_response_body_schema().get("properties").is_some());
    }

    #[test]
    fn tool_param_schema_is_present() {
        assert!(tool_param_schema().get("oneOf").is_some());
    }

    #[test]
    fn tool_choice_param_schema_is_present() {
        assert!(tool_choice_param_schema().get("oneOf").is_some());
    }

    #[test]
    fn item_param_schema_is_present() {
        assert!(item_param_schema().get("oneOf").is_some());
    }

    #[test]
    fn validate_stream_event_rejects_empty() {
        let value = serde_json::json!({});
        assert!(validate_stream_event(&value).is_err());
    }

    #[test]
    fn validate_response_resource_rejects_empty() {
        let value = serde_json::json!({});
        assert!(validate_response_resource(&value).is_err());
    }

    #[test]
    fn validate_create_response_body_accepts_minimal() {
        let value = serde_json::json!({
            "model": "gpt-4.1",
            "input": "hi"
        });
        assert!(validate_create_response_body(&value).is_ok());
    }

    #[test]
    fn validate_create_response_body_rejects_invalid_type() {
        let value = serde_json::json!("nope");
        assert!(validate_create_response_body(&value).is_err());
    }

    #[test]
    fn validate_create_response_body_accepts_tools_and_choice() {
        let value = serde_json::json!({
            "model": "gpt-4.1",
            "input": "hi",
            "tools": [
                { "type": "function", "name": "echo" }
            ],
            "tool_choice": "auto"
        });
        let errors = validate_create_response_body(&value)
            .err()
            .unwrap_or_default();
        assert!(errors.is_empty(), "errors: {errors:?}");
    }

    #[test]
    fn validate_create_response_body_reports_invalid_tool_fields() {
        let value = serde_json::json!({
            "model": "gpt-4.1",
            "input": "hi",
            "tools": "nope",
            "tool_choice": { "type": "unknown" }
        });
        let errors = validate_create_response_body(&value)
            .err()
            .unwrap_or_default();
        assert!(
            errors.iter().any(|err| err.contains("tools must be")),
            "errors: {errors:?}"
        );
        assert!(
            errors.iter().any(|err| err.contains("tool_choice")),
            "errors: {errors:?}"
        );
    }

    #[test]
    fn validate_tool_param_accepts_function_tool() {
        let value = serde_json::json!({
            "type": "function",
            "name": "echo"
        });
        let errors = validate_responses_tool_param(&value)
            .err()
            .unwrap_or_default();
        assert!(errors.is_empty(), "errors: {errors:?}");
    }

    #[test]
    fn validate_tool_param_accepts_all_variants() {
        let variants = vec![
            serde_json::json!({ "type": "code_interpreter", "container": "cntr_123" }),
            serde_json::json!({ "type": "custom", "name": "custom_tool" }),
            serde_json::json!({ "type": "web_search" }),
            serde_json::json!({ "type": "web_search_2025_08_26" }),
            serde_json::json!({ "type": "web_search_ga" }),
            serde_json::json!({ "type": "web_search_preview" }),
            serde_json::json!({ "type": "web_search_preview_2025_03_11" }),
            serde_json::json!({ "type": "image_generation" }),
            serde_json::json!({ "type": "mcp", "server_label": "srv" }),
            serde_json::json!({ "type": "file_search", "vector_store_ids": ["vs_1"] }),
            serde_json::json!({
                "type": "computer-preview",
                "display_width": 1024,
                "display_height": 768,
                "environment": "linux"
            }),
            serde_json::json!({
                "type": "computer_use_preview",
                "display_width": 800,
                "display_height": 600,
                "environment": "browser"
            }),
            serde_json::json!({ "type": "local_shell" }),
            serde_json::json!({ "type": "shell" }),
            serde_json::json!({ "type": "apply_patch" }),
        ];

        for value in variants {
            let errors = validate_responses_tool_param(&value)
                .err()
                .unwrap_or_default();
            assert!(errors.is_empty(), "errors: {errors:?} for {value}");
        }
    }

    #[test]
    fn validate_tool_param_rejects_invalid() {
        let value = serde_json::json!(42);
        assert!(validate_responses_tool_param(&value).is_err());
    }

    #[test]
    fn validate_tool_choice_param_accepts_auto() {
        let value = serde_json::json!("auto");
        let errors = validate_tool_choice_param(&value).err().unwrap_or_default();
        assert!(errors.is_empty(), "errors: {errors:?}");
    }

    #[test]
    fn validate_tool_choice_param_accepts_specific_function() {
        let value = serde_json::json!({
            "type": "function",
            "name": "echo"
        });
        let errors = validate_tool_choice_param(&value).err().unwrap_or_default();
        assert!(errors.is_empty(), "errors: {errors:?}");
    }

    #[test]
    fn validate_tool_choice_param_accepts_specific_variants() {
        let variants = vec![
            serde_json::json!({ "type": "file_search" }),
            serde_json::json!({ "type": "web_search" }),
            serde_json::json!({ "type": "web_search_preview" }),
            serde_json::json!({ "type": "image_generation" }),
            serde_json::json!({ "type": "computer-preview" }),
            serde_json::json!({ "type": "computer_use_preview" }),
            serde_json::json!({ "type": "code_interpreter" }),
            serde_json::json!({ "type": "local_shell" }),
            serde_json::json!({ "type": "shell" }),
            serde_json::json!({ "type": "apply_patch" }),
            serde_json::json!({ "type": "custom", "name": "custom_tool" }),
            serde_json::json!({ "type": "mcp", "server_label": "srv" }),
        ];

        for value in variants {
            let errors = validate_tool_choice_param(&value).err().unwrap_or_default();
            assert!(errors.is_empty(), "errors: {errors:?} for {value}");
        }
    }

    #[test]
    fn validate_tool_choice_param_accepts_allowed_tools() {
        let value = serde_json::json!({
            "type": "allowed_tools",
            "tools": [
                { "type": "function", "name": "echo" }
            ]
        });
        let errors = validate_tool_choice_param(&value).err().unwrap_or_default();
        assert!(errors.is_empty(), "errors: {errors:?}");
    }

    #[test]
    fn validate_tool_choice_param_rejects_invalid() {
        let value = serde_json::json!(false);
        assert!(validate_tool_choice_param(&value).is_err());
    }

    #[test]
    fn validate_item_param_accepts_user_message() {
        let value = serde_json::json!({
            "type": "message",
            "role": "user",
            "content": "hi"
        });
        let errors = validate_item_param(&value).err().unwrap_or_default();
        assert!(errors.is_empty(), "errors: {errors:?}");
    }

    #[test]
    fn validate_item_param_rejects_invalid_type() {
        let value = serde_json::json!("nope");
        assert!(validate_item_param(&value).is_err());
    }

    #[test]
    fn validate_user_message_item_schema_accepts_simple() {
        let schema =
            extract_component_schema("UserMessageItemParam").expect("UserMessageItemParam schema");
        let validator = JSONSchema::options()
            .with_document(OPENAPI_URI.to_string(), OPENAPI.clone())
            .compile(&schema)
            .expect("compile user message schema");
        let value = serde_json::json!({
            "type": "message",
            "role": "user",
            "content": "hi"
        });
        let errors = match validator.validate(&value) {
            Ok(_) => Vec::new(),
            Err(errs) => errs.map(|e| e.to_string()).collect(),
        };
        assert!(errors.is_empty(), "errors: {errors:?}");
    }
}
