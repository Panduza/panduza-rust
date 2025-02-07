use serde_json::Value as JsonValue;

#[derive(Debug)]
/// Metadata for an attribute
///
pub struct AttributeMetadata {
    ///
    ///
    r#type: String,
    ///
    ///
    info: Option<String>,
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
    pub fn from_json_value(value: &JsonValue) -> Result<Self, String> {
        //
        //
        let t = value
            .get("type")
            .map(|v| v.clone())
            .ok_or("field 'type' not found")?
            .to_string();

        Ok(Self {
            r#type: t,
            info: None,
        })
    }

    // try_into
}
