#![allow(dead_code)]
use serde::{Deserialize, Deserializer};

pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(value.and_then(|value| {
        if value.trim().is_empty() {
            None
        } else {
            Some(value)
        }
    }))
}
