pub mod github;
pub mod memory;

use crate::tracker::TrackerError;
use serde_json::Value;

pub(crate) fn normalized_labels(values: Option<&Value>) -> Vec<String> {
    let mut result = Vec::new();
    if let Some(items) = values.and_then(Value::as_array) {
        for item in items {
            let name = item
                .as_str()
                .or_else(|| item.get("name").and_then(Value::as_str));
            if let Some(name) = name {
                let name = name.trim().to_lowercase();
                if !name.is_empty() && !result.contains(&name) {
                    result.push(name);
                }
            }
        }
    }
    result
}

pub(crate) fn required<'a>(item: &'a Value, key: &str) -> Result<&'a str, TrackerError> {
    item.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| TrackerError::Payload(format!("missing {key}")))
}
