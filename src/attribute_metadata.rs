use serde_json::Value as JsonValue;

use crate::AttributeMode;

#[derive(Debug, Clone)]
/// Metadata for an attribute
///
pub struct AttributeMetadata {
    pub topic: String,
    ///
    ///
    pub r#type: String,
    ///
    ///
    pub info: Option<String>,

    pub mode: AttributeMode,
    // options
}

impl AttributeMetadata {
    /// Create a new attribute metadata from a topic only
    ///
    /// This creates a minimal metadata instance with default values
    /// for the type and without any info.
    pub fn from_topic<T: Into<String>>(
        topic: T,
        r#type: Option<String>,
        mode: AttributeMode,
    ) -> Self {
        Self {
            topic: topic.into(),
            r#type: r#type.unwrap_or_else(|| "unknown".to_string()),
            info: None,
            mode,
        }
    }

    ///
    ///
    pub fn from_json_value(topic: String, value: &JsonValue) -> Result<Self, String> {
        let t = value
            .get("type")
            .map(|v| v.clone())
            .ok_or("field 'type' not found")?
            .to_string();

        let mode = value
            .get("mode")
            .and_then(|v| v.as_str())
            .map(|v| match v {
                "RO" => AttributeMode::ReadOnly,
                "WO" => AttributeMode::WriteOnly,
                "RW" => AttributeMode::ReadWrite,
                _ => AttributeMode::ReadOnly,
            })
            .unwrap_or(AttributeMode::ReadOnly);

        Ok(Self {
            topic,
            r#type: t,
            info: None,
            mode,
        })
    }

    /// Create AttributeMetadata from StructureBuffer attribute data
    ///
    pub fn from_structure_buffer_attribute(
        topic: String,
        attr_type: &str,
        attr_mode: &str,
    ) -> Result<Self, String> {
        let mode = match attr_mode {
            "RO" => AttributeMode::ReadOnly,
            "WO" => AttributeMode::WriteOnly,
            "RW" => AttributeMode::ReadWrite,
            _ => AttributeMode::ReadOnly,
        };

        Ok(Self {
            topic,
            r#type: attr_type.to_string(),
            info: None,
            mode,
        })
    }
}
