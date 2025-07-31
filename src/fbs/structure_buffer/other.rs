use crate::fbs::panduza_generated::panduza::Node;

/// Convert a string to a Node enum
///
impl From<String> for Node {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Instance" => Node::Instance,
            "Class" => Node::Class,
            "Attribute" => Node::Attribute,
            _ => Node::Undefined,
        }
    }
}
