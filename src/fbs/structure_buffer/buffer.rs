use super::StructureBufferBuilder;
use crate::fbs::panduza_generated::panduza::Message;
use crate::PzaBuffer;
use bytes::Bytes;
use serde_json;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StructureBuffer {
    pub raw_data: Bytes,
}

//------------------------------------------------------------------------------
// Implementation of the PzaBuffer trait for StructureBuffer
//------------------------------------------------------------------------------
impl PzaBuffer for StructureBuffer {
    //------------------------------------------------------------------------------

    fn from_zbytes(zbytes: ZBytes) -> Self {
        // Convert ZBytes to Bytes and create a StructureBuffer
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        StructureBuffer { raw_data: bytes }
    }

    //------------------------------------------------------------------------------

    fn to_zbytes(self) -> ZBytes {
        // Convert the internal Bytes to ZBytes
        ZBytes::from(self.raw_data)
    }

    //------------------------------------------------------------------------------

    fn size(&self) -> usize {
        // Return the size of the internal buffer
        self.raw_data.len()
    }

    //------------------------------------------------------------------------------

    fn source(&self) -> Option<u16> {
        // Extract the source from the message header
        let msg = self.as_message();
        msg.header().map(|h| h.source())
    }

    //------------------------------------------------------------------------------

    fn sequence(&self) -> Option<u16> {
        // Extract the sequence from the message header
        let msg = self.as_message();
        msg.header().map(|h| h.sequence())
    }

    //------------------------------------------------------------------------------

    fn as_message(&self) -> Message {
        // Deserialize the FlatBuffer message from raw_data
        flatbuffers::root::<Message>(&self.raw_data)
            .expect("STRUCTURE: Failed to deserialize Message from raw_data")
    }

    //------------------------------------------------------------------------------

    fn has_same_message_value<B: PzaBuffer>(&self, _other_buffer: &B) -> bool {
        // Always returns true (placeholder implementation)
        true
    }

    //------------------------------------------------------------------------------
}

//------------------------------------------------------------------------------
// Implementation of StructureBuffer methods
//------------------------------------------------------------------------------
impl StructureBuffer {
    //------------------------------------------------------------------------------

    pub fn builder() -> StructureBufferBuilder {
        // Return a default StructureBufferBuilder
        StructureBufferBuilder::default()
    }

    //------------------------------------------------------------------------------

    pub fn as_json(&self) -> serde_json::Value {
        // Get the message from the buffer
        let msg = self.as_message();

        // If there's a structure payload, convert it to the required JSON format
        if let Some(structure) = msg.payload_as_structure() {
            Self::structure_to_json_hierarchical(&structure)
        } else {
            // Return empty object if no structure payload
            serde_json::Value::Object(serde_json::Map::new())
        }
    }

    //------------------------------------------------------------------------------

    fn structure_to_json_hierarchical(
        structure: &crate::fbs::panduza_generated::panduza::Structure,
    ) -> serde_json::Value {
        let mut result = serde_json::Map::new();

        // If this structure has a name, it becomes a key in the parent object
        if let Some(name) = structure.name() {
            let mut node_obj = serde_json::Map::new();

            // Add the _node field (with underscore as per requirement)
            if let Some(node_name) = structure.node().variant_name() {
                node_obj.insert(
                    "_node".to_string(),
                    serde_json::Value::String(node_name.to_lowercase()),
                );
            }

            // Add type if present
            if let Some(type_str) = structure.type_() {
                node_obj.insert(
                    "type".to_string(),
                    serde_json::Value::String(type_str.to_string()),
                );
            }

            // Add mode if present
            if let Some(mode) = structure.mode() {
                node_obj.insert(
                    "mode".to_string(),
                    serde_json::Value::String(mode.to_string()),
                );
            }

            // Add tags if present
            if let Some(tags) = structure.tags() {
                let tag_array: Vec<serde_json::Value> = tags
                    .iter()
                    .map(|tag| serde_json::Value::String(tag.to_string()))
                    .collect();
                if !tag_array.is_empty() {
                    node_obj.insert("tags".to_string(), serde_json::Value::Array(tag_array));
                }
            }

            // Add children as direct properties (not in a children array)
            if let Some(children) = structure.children() {
                for child in children.iter() {
                    let child_json = Self::structure_to_json_hierarchical(&child);
                    if let serde_json::Value::Object(child_map) = child_json {
                        // Merge child properties into this node
                        for (key, value) in child_map {
                            node_obj.insert(key, value);
                        }
                    }
                }
            }

            result.insert(name.to_string(), serde_json::Value::Object(node_obj));
        } else {
            // If no name, process children and merge them into the result
            if let Some(children) = structure.children() {
                for child in children.iter() {
                    let child_json = Self::structure_to_json_hierarchical(&child);
                    if let serde_json::Value::Object(child_map) = child_json {
                        // Merge child properties into the result
                        for (key, value) in child_map {
                            result.insert(key, value);
                        }
                    }
                }
            }
        }

        serde_json::Value::Object(result)
    }

    //------------------------------------------------------------------------------
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fbs::PzaBufferBuilder;

    #[test]
    fn test_as_json_simple_structure() {
        // Create a simple structure with an instance containing an attribute
        let buffer = StructureBuffer::builder()
            .with_name("device_test".to_string())
            .with_node("Instance".to_string())
            .with_children(vec![StructureBufferBuilder::default()
                .with_name("class_sensor".to_string())
                .with_node("Class".to_string())
                .with_children(vec![StructureBufferBuilder::default()
                    .with_name("attribute_value".to_string())
                    .with_node("Attribute".to_string())
                    .with_type("string".to_string())
                    .with_mode("RW".to_string())])])
            .build()
            .expect("Failed to build test structure");

        let json = buffer.as_json();

        // Verify the JSON structure
        let obj = json.as_object().expect("Should be a JSON object");

        // Check the root instance
        assert!(obj.contains_key("device_test"));
        let device = obj["device_test"]
            .as_object()
            .expect("Device should be an object");
        assert_eq!(device["_node"], "instance");

        // Check the class
        assert!(device.contains_key("class_sensor"));
        let class = device["class_sensor"]
            .as_object()
            .expect("Class should be an object");
        assert_eq!(class["_node"], "class");

        // Check the attribute
        assert!(class.contains_key("attribute_value"));
        let attribute = class["attribute_value"]
            .as_object()
            .expect("Attribute should be an object");
        assert_eq!(attribute["_node"], "attribute");
        assert_eq!(attribute["type"], "string");
        assert_eq!(attribute["mode"], "RW");
    }

    #[test]
    fn test_as_json_with_tags() {
        // Create a structure with tags
        let buffer = StructureBuffer::builder()
            .with_name("device_with_tags".to_string())
            .with_node("Instance".to_string())
            .with_children(vec![StructureBufferBuilder::default()
                .with_name("class_multi".to_string())
                .with_node("Class".to_string())
                .with_tags(vec![
                    "tag1".to_string(),
                    "tag2".to_string(),
                    "sensor".to_string(),
                ])
                .with_children(vec![StructureBufferBuilder::default()
                    .with_name("attribute_temp".to_string())
                    .with_node("Attribute".to_string())
                    .with_type("number".to_string())
                    .with_mode("R".to_string())])])
            .build()
            .expect("Failed to build test structure with tags");

        let json = buffer.as_json();

        // Verify the JSON structure
        let obj = json.as_object().expect("Should be a JSON object");
        let device = obj["device_with_tags"]
            .as_object()
            .expect("Device should be an object");
        let class = device["class_multi"]
            .as_object()
            .expect("Class should be an object");

        // Check tags array
        assert!(class.contains_key("tags"));
        let tags = class["tags"].as_array().expect("Tags should be an array");
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0], "tag1");
        assert_eq!(tags[1], "tag2");
        assert_eq!(tags[2], "sensor");

        // Check the attribute
        let attribute = class["attribute_temp"]
            .as_object()
            .expect("Attribute should be an object");
        assert_eq!(attribute["_node"], "attribute");
        assert_eq!(attribute["type"], "number");
        assert_eq!(attribute["mode"], "R");
    }

    #[test]
    fn test_as_json_multiple_instances() {
        // Create a structure with multiple instances (simulating multiple children at root level)
        let buffer = StructureBuffer::builder()
            .with_children(vec![
                StructureBufferBuilder::default()
                    .with_name("instance_1".to_string())
                    .with_node("Instance".to_string())
                    .with_children(vec![StructureBufferBuilder::default()
                        .with_name("class_foo".to_string())
                        .with_node("Class".to_string())
                        .with_children(vec![StructureBufferBuilder::default()
                            .with_name("attribute_1".to_string())
                            .with_node("Attribute".to_string())
                            .with_type("string".to_string())
                            .with_mode("RW".to_string())])]),
                StructureBufferBuilder::default()
                    .with_name("instance_2".to_string())
                    .with_node("Instance".to_string())
                    .with_children(vec![StructureBufferBuilder::default()
                        .with_name("class_bar".to_string())
                        .with_node("Class".to_string())
                        .with_children(vec![StructureBufferBuilder::default()
                            .with_name("attribute_2".to_string())
                            .with_node("Attribute".to_string())
                            .with_type("number".to_string())])]),
            ])
            .build()
            .expect("Failed to build test structure with multiple instances");

        let json = buffer.as_json();

        // Verify the JSON structure
        let obj = json.as_object().expect("Should be a JSON object");

        // Check both instances exist
        assert!(obj.contains_key("instance_1"));
        assert!(obj.contains_key("instance_2"));

        // Check instance_1
        let instance1 = obj["instance_1"]
            .as_object()
            .expect("Instance1 should be an object");
        assert_eq!(instance1["_node"], "instance");
        let class1 = instance1["class_foo"]
            .as_object()
            .expect("Class should be an object");
        assert_eq!(class1["_node"], "class");
        let attr1 = class1["attribute_1"]
            .as_object()
            .expect("Attribute should be an object");
        assert_eq!(attr1["_node"], "attribute");
        assert_eq!(attr1["type"], "string");
        assert_eq!(attr1["mode"], "RW");

        // Check instance_2
        let instance2 = obj["instance_2"]
            .as_object()
            .expect("Instance2 should be an object");
        assert_eq!(instance2["_node"], "instance");
        let class2 = instance2["class_bar"]
            .as_object()
            .expect("Class should be an object");
        assert_eq!(class2["_node"], "class");
        let attr2 = class2["attribute_2"]
            .as_object()
            .expect("Attribute should be an object");
        assert_eq!(attr2["_node"], "attribute");
        assert_eq!(attr2["type"], "number");
        // Note: mode is optional and not set for attribute_2
        assert!(!attr2.contains_key("mode"));
    }

    #[test]
    fn test_as_json_empty_structure() {
        // Create an empty structure
        let buffer = StructureBuffer::builder()
            .build()
            .expect("Failed to build empty structure");

        let json = buffer.as_json();

        // Should return an empty object
        let obj = json.as_object().expect("Should be a JSON object");
        assert!(obj.is_empty());
    }

    #[test]
    fn test_as_json_attribute_without_optional_fields() {
        // Create a structure with minimal attribute (only required fields)
        let buffer = StructureBuffer::builder()
            .with_name("minimal_device".to_string())
            .with_node("Instance".to_string())
            .with_children(vec![StructureBufferBuilder::default()
                .with_name("minimal_class".to_string())
                .with_node("Class".to_string())
                .with_children(vec![
                    StructureBufferBuilder::default()
                        .with_name("minimal_attr".to_string())
                        .with_node("Attribute".to_string()), // No type or mode specified
                ])])
            .build()
            .expect("Failed to build minimal structure");

        let json = buffer.as_json();

        // Verify the JSON structure
        let obj = json.as_object().expect("Should be a JSON object");
        let device = obj["minimal_device"]
            .as_object()
            .expect("Device should be an object");
        let class = device["minimal_class"]
            .as_object()
            .expect("Class should be an object");
        let attribute = class["minimal_attr"]
            .as_object()
            .expect("Attribute should be an object");

        // Should only have _node field
        assert_eq!(attribute["_node"], "attribute");
        assert!(!attribute.contains_key("type"));
        assert!(!attribute.contains_key("mode"));
        assert!(!attribute.contains_key("tags"));
    }
}
