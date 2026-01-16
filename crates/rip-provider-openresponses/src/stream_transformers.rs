use rip_kernel::{Event, EventKind, ProviderEventStatus};
use serde_json::Value;

fn event_type(event: &Event) -> Option<&str> {
    match &event.kind {
        EventKind::ProviderEvent {
            status,
            event_name,
            data,
            ..
        } => {
            if *status != ProviderEventStatus::Event {
                return None;
            }
            if let Some(Value::Object(obj)) = data {
                if let Some(Value::String(value)) = obj.get("type") {
                    return Some(value.as_str());
                }
            }
            event_name.as_deref()
        }
        _ => None,
    }
}

fn event_delta(event: &Event, expected_type: &str) -> Option<String> {
    if event_type(event)? != expected_type {
        return None;
    }
    match &event.kind {
        EventKind::ProviderEvent {
            data: Some(Value::Object(obj)),
            ..
        } => obj
            .get("delta")
            .and_then(|value| value.as_str())
            .map(|value| value.to_string()),
        _ => None,
    }
}

pub fn extract_text_deltas(events: &[Event]) -> Vec<String> {
    events
        .iter()
        .filter_map(|event| event_delta(event, "response.output_text.delta"))
        .collect()
}

pub fn extract_reasoning_deltas(events: &[Event]) -> Vec<String> {
    events
        .iter()
        .filter_map(|event| event_delta(event, "response.reasoning.delta"))
        .collect()
}

pub fn extract_tool_call_argument_deltas(events: &[Event]) -> Vec<String> {
    events
        .iter()
        .filter_map(|event| event_delta(event, "response.function_call_arguments.delta"))
        .collect()
}
