#[derive(Debug)]
/// Helper to decompose topics into Panduza structure
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
