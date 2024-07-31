use serde_json::Value as JsonValue;

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
    // mode
    // options
}

impl AttributeMetadata {
    // /// Create a new attribute metadata
    // pub fn new<T: Into<String>, I: Into<String>>(r#type: T, info: I) -> Self {
    //     Self {
    //         r#type: r#type.into(),
    //         info: info.into(),
    //     }
    // }

    ///
    ///
    pub fn from_json_value(topic: String, value: &JsonValue) -> Result<Self, String> {
        //
        //
        let t = value
            .get("type")
            .map(|v| v.clone())
            .ok_or("field 'type' not found")?
            .to_string();

        Ok(Self {
            topic,
            r#type: t,
            info: None,
        })
    }
}
