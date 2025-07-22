#[derive(Debug)]
/// Helper to decompose Panduza topics into a structured object
///
/// ## Path Format Documentation
///
/// This document describes the structure and components of a specific path format used within our system to access resources or services in a hierarchical manner.
///
/// ### Topic Format
///
/// The topic format is structured as follows:
///
/// `{namespace}/pza/{instance}/{*class}/{attribute}/[cmd|att]`
///
/// ### Components
///
/// 1. **`{namespace}`**:
///    - **Description**: This segment represents a namespace, which is used to group related items and avoid naming conflicts. It typically denotes the name of an organization, project, or specific domain.
///    - **Example**: `company`, `project_alpha`
///
/// 2. **`pza`**:
///    - **Description**: This is a fixed identifier used to designate a specific type of resource or service within the system.
///    - **Note**: The identifier `pza` is constant and should always be included as shown.
///
/// 3. **`{instance}`**:
///    - **Description**: This segment refers to a specific instance within the namespace. An instance is a particular occurrence of a resource or service.
///    - **Example**: `instance_1`, `service_001`
///
/// 4. **`{*class}`**:
///    - **Description**: This segment represents multiple levels of classes. It indicates that this part of the topic can contain a hierarchy of classes or categories, allowing for a deeper and more detailed structuring of resources.
///    - **Example**: `level1/level2/level3`
///
/// 5. **`{attribute}`**:
///    - **Description**: This segment specifies a particular attribute of the class or instance. An attribute is a property or characteristic of an object.
///    - **Example**: `color`, `size`
///
/// 6. **`[cmd|att]`**:
///    - **Description**: This segment indicates that the topic must end with either a command (`cmd`) or an attribute (`att`). This means the "topic" must provide either a command or an attribute to complete the topic.
///    - **Example**: `cmd=start`, `att=value`
///
/// ### Example topic
///
/// Here is an example of a complete topic using the described format:
///
/// `company/pza/service_001/level1/level2/color/cmd`
///
/// In this example:
/// - `company` is the namespace.
/// - `pza` is the fixed identifier.
/// - `service_001` is the instance.
/// - `level1/level2` are the hierarchical class levels.
/// - `color` is the attribute.
/// - `cmd` is the command that completes the path.
///
pub struct Topic {
    pub _namespace: String,

    /// Name of the instance
    ///
    pub instance: String,

    /// Sub layers
    ///
    pub layers: Vec<String>,

    /// True if it is an attribute path, false for container
    ///
    pub is_attribute: bool,
}

impl Topic {
    /// Instance name getter
    ///
    pub fn instance_name(&self) -> &String {
        &self.instance
    }

    ///
    ///
    pub fn class_stack_name(&self) -> String {
        let mut r = String::new();
        if self.is_attribute {
            //
            // Copy layers and remove the last one (which is the name the attribute)
            let mut n = self.layers.clone();
            n.remove(n.len() - 1);
            // CODE DUPLICATION
            let mut first = true;
            for l in &n {
                if first {
                    r = format!("{}", l);
                    first = false;
                } else {
                    r = format!("{}/{}", r, l);
                }
            }
        } else {
            // CODE DUPLICATION
            let mut first = true;
            for l in &self.layers {
                if first {
                    r = format!("{}", l);
                    first = false;
                } else {
                    r = format!("{}/{}", r, l);
                }
            }
        }
        r
    }

    /// Attribute of Class name getter
    ///
    /// We cannot know if it is a attribute or class just with the topic
    ///
    pub fn leaf_name(&self) -> Option<&String> {
        self.layers.last()
    }

    pub fn from_string<A: Into<String>>(topic: A, is_attribute: bool) -> Self {
        // Split the topic
        let topic_string = topic.into();
        let mut layers: Vec<&str> = topic_string.split('/').collect();

        //
        //
        let mut namespace_parts: Vec<String> = Vec::new();
        while !layers.is_empty() {
            {
                let layer = layers.get(0).unwrap();
                if *layer == "pza" {
                    break;
                }
                namespace_parts.push(layer.to_string());
            }
            layers.remove(0);
        }

        // Remove pza
        layers.remove(0);

        //
        //
        let namespace = namespace_parts.join("/");
        let device = layers.remove(0).to_string();

        Self {
            _namespace: namespace,
            instance: device,
            layers: layers.into_iter().map(|l| l.to_string()).collect(),
            is_attribute: is_attribute,
        }
    }

    pub fn layers_len(&self) -> usize {
        self.layers.len()
    }

    pub fn first_layer(&self) -> String {
        self.layers.first().unwrap().clone()
    }

    pub fn last_layer(&self) -> String {
        self.layers.last().unwrap().clone()
    }

    // ------------------------------------------------------------------------

    /// Returns the full path as a vector of strings
    ///
    pub fn vector_path(&self) -> Vec<String> {
        let mut r = vec![self.instance.clone()];
        r.extend(self.layers.iter().cloned());
        r
    }

    // ------------------------------------------------------------------------
}

#[cfg(test)]
mod tests {
    use super::Topic;

    #[test]
    fn test_stack_name() {
        let topic = Topic::from_string("pza/truc/machin", true);

        assert_eq!(topic.class_stack_name(), "".to_string());
    }
}
