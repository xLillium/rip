use serde_json::{Map, Value};

use rip_openresponses::{validate_create_response_body, validate_item_param};

#[derive(Debug, Clone)]
pub struct ItemParam {
    value: Value,
    errors: Vec<String>,
}

impl ItemParam {
    pub fn new(value: Value) -> Self {
        let errors = match validate_item_param(&value) {
            Ok(_) => Vec::new(),
            Err(errs) => errs,
        };
        Self { value, errors }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn into_value(self) -> Value {
        self.value
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

#[derive(Debug, Clone)]
pub struct CreateResponsePayload {
    body: Value,
    errors: Vec<String>,
}

impl CreateResponsePayload {
    pub fn new(body: Value) -> Self {
        let errors = match validate_create_response_body(&body) {
            Ok(_) => Vec::new(),
            Err(errs) => errs,
        };
        Self { body, errors }
    }

    pub fn body(&self) -> &Value {
        &self.body
    }

    pub fn into_body(self) -> Value {
        self.body
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

#[derive(Debug, Default)]
pub struct CreateResponseBuilder {
    body: Map<String, Value>,
}

impl CreateResponseBuilder {
    pub fn new() -> Self {
        Self { body: Map::new() }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.body
            .insert("model".to_string(), Value::String(model.into()));
        self
    }

    pub fn input_text(mut self, text: impl Into<String>) -> Self {
        self.body
            .insert("input".to_string(), Value::String(text.into()));
        self
    }

    pub fn input_items(mut self, items: Vec<ItemParam>) -> Self {
        let array = items
            .into_iter()
            .map(ItemParam::into_value)
            .collect::<Vec<_>>();
        self.body.insert("input".to_string(), Value::Array(array));
        self
    }

    pub fn input_items_raw(mut self, items: Vec<Value>) -> Self {
        self.body.insert("input".to_string(), Value::Array(items));
        self
    }

    pub fn insert_raw(mut self, key: impl Into<String>, value: Value) -> Self {
        self.body.insert(key.into(), value);
        self
    }

    pub fn build(self) -> CreateResponsePayload {
        CreateResponsePayload::new(Value::Object(self.body))
    }
}
