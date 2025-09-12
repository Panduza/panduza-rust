use serde::Serialize;

use crate::fbs::PzaBuffer;
use crate::fbs::StructureBuffer;
use crate::AttributeMetadata;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

/// HashMap representation of the StructureBuffer for easier attribute lookup
///
/// This structure provides a flattened view of the hierarchical structure buffer,
/// where each key is a complete Panduza topic without the final [cmd|att] suffix.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlatStructure {
    /// Map containing complete Panduza topics as keys and their metadata as values
    pub attributes: HashMap<String, AttributeMetadata>,
}

impl FlatStructure {
    /// Creates a new empty FlatStructure
    ///
    /// # Returns  
    /// A new FlatStructure instance with an empty HashMap
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    // ------------------------------------------------------------------------

    /// Creates a FlatStructure from a StructureBuffer
    ///
    /// # Arguments
    /// * `buffer` - The StructureBuffer to convert to flat representation
    /// * `base_topic` - The base topic path for building complete topics
    ///
    /// # Returns
    /// A new FlatStructure instance populated from the buffer
    pub fn from_buffer(buffer: &StructureBuffer, base_topic: &str) -> Self {
        let mut flat = Self::new();
        flat.update_from_buffer(buffer, base_topic);
        flat
    }

    // ------------------------------------------------------------------------

    /// Updates the flat structure from a StructureBuffer
    ///
    /// This method clears the existing attributes and rebuilds them from the buffer.
    /// Only nodes with both 'type' and 'mode' fields are considered valid attributes.
    ///
    /// # Arguments
    /// * `buffer` - The StructureBuffer to extract attributes from
    /// * `base_topic` - The base topic path for building complete topics
    pub fn update_from_buffer(&mut self, buffer: &StructureBuffer, base_topic: &str) {
        // Clear existing attributes
        self.attributes.clear();

        // Get the message from the buffer
        let message = buffer.as_message();

        // Extract the Structure from the payload
        if let Some(structure) = message.payload_as_structure() {
            println!(
                "Updating FlatStructure from buffer with base_topic: {}",
                base_topic
            );

            // Extract instance path from base_topic (remove /att suffix if present)
            let instance_path = if base_topic.ends_with("/att") {
                &base_topic[..base_topic.len() - 4]
            } else {
                base_topic
            };

            // Start flattening from the root
            self.flatten_structure_node(instance_path.to_string(), &structure);
        }
    }

    // ------------------------------------------------------------------------

    /// Recursively flattens a Structure node into the attributes HashMap
    ///
    /// This method traverses the hierarchical structure and creates flat entries
    /// for leaf nodes that contain both 'type' and 'mode' fields.
    ///
    /// # Arguments
    /// * `current_path` - The current topic path being built
    /// * `node` - The structure node to process
    fn flatten_structure_node(
        &mut self,
        current_path: String,
        node: &crate::fbs::panduza_generated::panduza::Structure,
    ) {
        println!("pok 55");

        // Get node name, if empty skip this node
        let node_name = match node.name() {
            Some(name) if !name.is_empty() => name,
            _ => return,
        };

        println!("Processing node: {} at path: {}", node_name, current_path);

        // Build the current topic path
        let new_path = if current_path.is_empty() {
            node_name.to_string()
        } else {
            format!("{}/{}", current_path, node_name)
        };

        // If this node has both type and mode, it's a leaf attribute
        // Insert entry only if node contains a 'mode' (indicating it's a valid attribute leaf)
        if let (Some(attr_type), Some(attr_mode)) = (node.type_(), node.mode()) {
            // Create AttributeMetadata from the structure information
            if let Ok(metadata) = crate::AttributeMetadata::from_structure_buffer_attribute(
                new_path.clone(),
                attr_type,
                attr_mode,
            ) {
                // Insert with complete Panduza topic (without final cmd/att)
                self.attributes.insert(new_path.clone(), metadata);
            }
        }

        // Process child nodes recursively
        if let Some(children) = node.children() {
            for i in 0..children.len() {
                let child = children.get(i);
                self.flatten_structure_node(new_path.clone(), &child);
            }
        }
    }

    // ------------------------------------------------------------------------

    /// Gets an attribute metadata by its topic path
    ///
    /// # Arguments
    /// * `topic` - The topic path to look up
    ///
    /// # Returns
    /// An Option containing the AttributeMetadata if found
    pub fn get(&self, topic: &str) -> Option<&AttributeMetadata> {
        self.attributes.get(topic)
    }

    // ------------------------------------------------------------------------

    /// Finds attributes matching a pattern
    ///
    /// # Arguments
    /// * `pattern` - The pattern to search for in topic paths
    ///
    /// # Returns
    /// A vector of AttributeMetadata for matching topics
    pub fn find_attributes(&self, pattern: &str) -> Vec<&AttributeMetadata> {
        self.attributes
            .iter()
            .filter(|(topic, _)| topic.contains(pattern))
            .map(|(_, metadata)| metadata)
            .collect()
    }

    // ------------------------------------------------------------------------

    /// Gets all attribute topics
    ///
    /// # Returns
    /// A vector containing all topic paths
    pub fn get_topics(&self) -> Vec<&String> {
        self.attributes.keys().collect()
    }

    // ------------------------------------------------------------------------

    /// Gets the number of attributes in the flat structure
    ///
    /// # Returns
    /// The number of attributes
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    // ------------------------------------------------------------------------

    /// Checks if the flat structure is empty
    ///
    /// # Returns
    /// True if no attributes are present
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    // ------------------------------------------------------------------------

    /// Clears all attributes from the flat structure
    pub fn clear(&mut self) {
        self.attributes.clear();
    }
}

// ------------------------------------------------------------------------

impl From<(&StructureBuffer, &str)> for FlatStructure {
    /// Creates a FlatStructure from a StructureBuffer and base topic
    ///
    /// # Arguments
    /// * `(buffer, base_topic)` - Tuple containing the buffer and base topic
    ///
    /// # Returns
    /// A new FlatStructure instance
    fn from((buffer, base_topic): (&StructureBuffer, &str)) -> Self {
        Self::from_buffer(buffer, base_topic)
    }
}

// ------------------------------------------------------------------------

impl std::ops::Deref for FlatStructure {
    type Target = HashMap<String, AttributeMetadata>;

    fn deref(&self) -> &Self::Target {
        &self.attributes
    }
}

// ------------------------------------------------------------------------

impl std::ops::DerefMut for FlatStructure {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attributes
    }
}
