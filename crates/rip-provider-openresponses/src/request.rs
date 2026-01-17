use serde_json::{Map, Value};

use rip_openresponses::{
    validate_create_response_body, validate_item_param, validate_responses_tool_param,
    validate_specific_tool_choice_param, validate_tool_choice_param,
};

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
pub struct ToolParam {
    value: Value,
    errors: Vec<String>,
}

impl ToolParam {
    pub fn new(value: Value) -> Self {
        let errors = match validate_responses_tool_param(&value) {
            Ok(_) => Vec::new(),
            Err(errs) => errs,
        };
        Self { value, errors }
    }

    pub fn function(name: impl Into<String>) -> Self {
        let mut obj = Map::new();
        obj.insert("type".to_string(), Value::String("function".to_string()));
        obj.insert("name".to_string(), Value::String(name.into()));
        let value = Value::Object(obj);
        Self::new(value)
    }

    pub fn code_interpreter(container: impl Into<Value>) -> Self {
        let mut obj = Map::new();
        obj.insert(
            "type".to_string(),
            Value::String("code_interpreter".to_string()),
        );
        obj.insert("container".to_string(), container.into());
        Self::new(Value::Object(obj))
    }

    pub fn custom(name: impl Into<String>) -> Self {
        let mut obj = Map::new();
        obj.insert("type".to_string(), Value::String("custom".to_string()));
        obj.insert("name".to_string(), Value::String(name.into()));
        Self::new(Value::Object(obj))
    }

    pub fn web_search() -> Self {
        Self::new(Value::Object(tool_type_only("web_search")))
    }

    pub fn web_search_2025_08_26() -> Self {
        Self::new(Value::Object(tool_type_only("web_search_2025_08_26")))
    }

    pub fn web_search_ga() -> Self {
        Self::new(Value::Object(tool_type_only("web_search_ga")))
    }

    pub fn web_search_preview() -> Self {
        Self::new(Value::Object(tool_type_only("web_search_preview")))
    }

    pub fn web_search_preview_2025_03_11() -> Self {
        Self::new(Value::Object(tool_type_only(
            "web_search_preview_2025_03_11",
        )))
    }

    pub fn image_generation() -> Self {
        Self::new(Value::Object(tool_type_only("image_generation")))
    }

    pub fn mcp(server_label: impl Into<String>) -> Self {
        let mut obj = Map::new();
        obj.insert("type".to_string(), Value::String("mcp".to_string()));
        obj.insert(
            "server_label".to_string(),
            Value::String(server_label.into()),
        );
        Self::new(Value::Object(obj))
    }

    pub fn file_search(vector_store_ids: Vec<String>) -> Self {
        let mut obj = Map::new();
        obj.insert("type".to_string(), Value::String("file_search".to_string()));
        obj.insert(
            "vector_store_ids".to_string(),
            Value::Array(vector_store_ids.into_iter().map(Value::String).collect()),
        );
        Self::new(Value::Object(obj))
    }

    pub fn computer_preview(
        display_width: u64,
        display_height: u64,
        environment: impl Into<String>,
    ) -> Self {
        Self::new(Value::Object(computer_tool_value(
            "computer-preview",
            display_width,
            display_height,
            environment,
        )))
    }

    pub fn computer_use_preview(
        display_width: u64,
        display_height: u64,
        environment: impl Into<String>,
    ) -> Self {
        Self::new(Value::Object(computer_tool_value(
            "computer_use_preview",
            display_width,
            display_height,
            environment,
        )))
    }

    pub fn local_shell() -> Self {
        Self::new(Value::Object(tool_type_only("local_shell")))
    }

    pub fn shell() -> Self {
        Self::new(Value::Object(tool_type_only("shell")))
    }

    pub fn apply_patch() -> Self {
        Self::new(Value::Object(tool_type_only("apply_patch")))
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
pub struct ToolChoiceParam {
    value: Value,
    errors: Vec<String>,
}

impl ToolChoiceParam {
    pub fn new(value: Value) -> Self {
        let errors = match validate_tool_choice_param(&value) {
            Ok(_) => Vec::new(),
            Err(errs) => errs,
        };
        Self { value, errors }
    }

    pub fn auto() -> Self {
        Self::new(Value::String("auto".to_string()))
    }

    pub fn none() -> Self {
        Self::new(Value::String("none".to_string()))
    }

    pub fn required() -> Self {
        Self::new(Value::String("required".to_string()))
    }

    pub fn specific_function(name: impl Into<String>) -> Self {
        Self::specific(SpecificToolChoiceParam::function(name))
    }

    pub fn specific(tool: SpecificToolChoiceParam) -> Self {
        Self::new(tool.into_value())
    }

    pub fn specific_file_search() -> Self {
        Self::specific(SpecificToolChoiceParam::file_search())
    }

    pub fn specific_web_search() -> Self {
        Self::specific(SpecificToolChoiceParam::web_search())
    }

    pub fn specific_web_search_preview() -> Self {
        Self::specific(SpecificToolChoiceParam::web_search_preview())
    }

    pub fn specific_image_generation() -> Self {
        Self::specific(SpecificToolChoiceParam::image_generation())
    }

    pub fn specific_computer_preview() -> Self {
        Self::specific(SpecificToolChoiceParam::computer_preview())
    }

    pub fn specific_computer_use_preview() -> Self {
        Self::specific(SpecificToolChoiceParam::computer_use_preview())
    }

    pub fn specific_code_interpreter() -> Self {
        Self::specific(SpecificToolChoiceParam::code_interpreter())
    }

    pub fn specific_local_shell() -> Self {
        Self::specific(SpecificToolChoiceParam::local_shell())
    }

    pub fn specific_shell() -> Self {
        Self::specific(SpecificToolChoiceParam::shell())
    }

    pub fn specific_apply_patch() -> Self {
        Self::specific(SpecificToolChoiceParam::apply_patch())
    }

    pub fn specific_custom(name: impl Into<String>) -> Self {
        Self::specific(SpecificToolChoiceParam::custom(name))
    }

    pub fn specific_mcp(server_label: impl Into<String>) -> Self {
        Self::specific(SpecificToolChoiceParam::mcp(server_label))
    }

    pub fn allowed_tools(tools: Vec<SpecificToolChoiceParam>) -> Self {
        Self::allowed_tools_with_mode(tools, None)
    }

    pub fn allowed_tools_with_mode(
        tools: Vec<SpecificToolChoiceParam>,
        mode: Option<ToolChoiceValue>,
    ) -> Self {
        let array = tools
            .into_iter()
            .map(SpecificToolChoiceParam::into_value)
            .collect::<Vec<_>>();
        let mut obj = Map::new();
        obj.insert(
            "type".to_string(),
            Value::String("allowed_tools".to_string()),
        );
        obj.insert("tools".to_string(), Value::Array(array));
        if let Some(mode) = mode {
            obj.insert("mode".to_string(), Value::String(mode.as_str().to_string()));
        }
        Self::new(Value::Object(obj))
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

#[derive(Debug, Clone, Copy)]
pub enum ToolChoiceValue {
    Auto,
    Required,
    None,
}

impl ToolChoiceValue {
    fn as_str(&self) -> &'static str {
        match self {
            ToolChoiceValue::Auto => "auto",
            ToolChoiceValue::Required => "required",
            ToolChoiceValue::None => "none",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpecificToolChoiceParam {
    value: Value,
    errors: Vec<String>,
}

impl SpecificToolChoiceParam {
    pub fn new(value: Value) -> Self {
        let errors = match validate_specific_tool_choice_param(&value) {
            Ok(_) => Vec::new(),
            Err(errs) => errs,
        };
        Self { value, errors }
    }

    pub fn function(name: impl Into<String>) -> Self {
        let mut obj = Map::new();
        obj.insert("type".to_string(), Value::String("function".to_string()));
        obj.insert("name".to_string(), Value::String(name.into()));
        Self::new(Value::Object(obj))
    }

    pub fn custom(name: impl Into<String>) -> Self {
        let mut obj = Map::new();
        obj.insert("type".to_string(), Value::String("custom".to_string()));
        obj.insert("name".to_string(), Value::String(name.into()));
        Self::new(Value::Object(obj))
    }

    pub fn mcp(server_label: impl Into<String>) -> Self {
        let mut obj = Map::new();
        obj.insert("type".to_string(), Value::String("mcp".to_string()));
        obj.insert(
            "server_label".to_string(),
            Value::String(server_label.into()),
        );
        Self::new(Value::Object(obj))
    }

    pub fn file_search() -> Self {
        Self::new(Value::Object(tool_type_only("file_search")))
    }

    pub fn web_search() -> Self {
        Self::new(Value::Object(tool_type_only("web_search")))
    }

    pub fn web_search_preview() -> Self {
        Self::new(Value::Object(tool_type_only("web_search_preview")))
    }

    pub fn image_generation() -> Self {
        Self::new(Value::Object(tool_type_only("image_generation")))
    }

    pub fn computer_preview() -> Self {
        Self::new(Value::Object(tool_type_only("computer-preview")))
    }

    pub fn computer_use_preview() -> Self {
        Self::new(Value::Object(tool_type_only("computer_use_preview")))
    }

    pub fn code_interpreter() -> Self {
        Self::new(Value::Object(tool_type_only("code_interpreter")))
    }

    pub fn local_shell() -> Self {
        Self::new(Value::Object(tool_type_only("local_shell")))
    }

    pub fn shell() -> Self {
        Self::new(Value::Object(tool_type_only("shell")))
    }

    pub fn apply_patch() -> Self {
        Self::new(Value::Object(tool_type_only("apply_patch")))
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

fn tool_type_only(tool_type: &str) -> Map<String, Value> {
    let mut obj = Map::new();
    obj.insert("type".to_string(), Value::String(tool_type.to_string()));
    obj
}

fn computer_tool_value(
    tool_type: &str,
    display_width: u64,
    display_height: u64,
    environment: impl Into<String>,
) -> Map<String, Value> {
    let mut obj = Map::new();
    obj.insert("type".to_string(), Value::String(tool_type.to_string()));
    obj.insert(
        "display_width".to_string(),
        Value::Number(display_width.into()),
    );
    obj.insert(
        "display_height".to_string(),
        Value::Number(display_height.into()),
    );
    obj.insert("environment".to_string(), Value::String(environment.into()));
    obj
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

    pub fn tools(mut self, tools: Vec<ToolParam>) -> Self {
        let array = tools
            .into_iter()
            .map(ToolParam::into_value)
            .collect::<Vec<_>>();
        self.body.insert("tools".to_string(), Value::Array(array));
        self
    }

    pub fn tools_raw(mut self, tools: Vec<Value>) -> Self {
        self.body.insert("tools".to_string(), Value::Array(tools));
        self
    }

    pub fn tool_choice(mut self, choice: ToolChoiceParam) -> Self {
        self.body
            .insert("tool_choice".to_string(), choice.into_value());
        self
    }

    pub fn tool_choice_raw(mut self, choice: Value) -> Self {
        self.body.insert("tool_choice".to_string(), choice);
        self
    }

    pub fn parallel_tool_calls(mut self, enabled: bool) -> Self {
        self.body
            .insert("parallel_tool_calls".to_string(), Value::Bool(enabled));
        self
    }

    pub fn max_tool_calls(mut self, max_calls: u64) -> Self {
        self.body.insert(
            "max_tool_calls".to_string(),
            Value::Number(max_calls.into()),
        );
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
