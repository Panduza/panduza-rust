use serde_json::Value as JsonValue;

///
///
struct Structure {
    /// initial data
    ///
    json_value: JsonValue,
}

impl Structure {
    fn update(&mut self, json_value: JsonValue) {
        self.json_value = json_value;
    }
}
