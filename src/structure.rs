use crate::attribute_metadata::AttributeMetadata;
use crate::fbs::{PzaBuffer, StructureBuffer};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::Notify;
use yash_fnmatch::{without_escape, Pattern};
use zenoh::handlers::FifoChannelHandler;
use zenoh::query::Reply;

#[derive(Debug)]
struct StructureData {
    ///
    ///
    brut: StructureBuffer,

    /// Structure extracted and prepared for easy access
    ///
    flat: HashMap<String, AttributeMetadata>,

    ///
    ///
    initialized: Arc<Notify>,
}

impl StructureData {
    ///
    ///
    pub fn new() -> StructureData {
        Self {
            brut: StructureBuffer::default(),
            flat: HashMap::new(),
            initialized: Arc::new(Notify::new()),
        }
    }

    ///
    ///
    pub fn update(&mut self, payload: zenoh::bytes::ZBytes) -> Result<(), String> {
        // Convert ZBytes to StructureBuffer
        let brut = StructureBuffer::from_zbytes(payload);

        // trace
        // println!("StructureBuffer size: {}", brut.size());

        // Extract and flatten the structure
        self.flatten_structure_buffer(&brut)?;

        // Store the StructureBuffer
        self.brut = brut;

        self.initialized.notify_waiters();

        Ok(())
    }

    /// Extract and flatten structure from StructureBuffer
    ///
    fn flatten_structure_buffer(
        &mut self,
        structure_buffer: &StructureBuffer,
    ) -> Result<(), String> {
        // Clear the existing flat structure
        self.flat.clear();

        // Get the message from the buffer
        let message = structure_buffer.as_message();

        // Extract the StructureNode from the payload
        if let Some(structure_node) = message.payload_as_structure_node() {
            // Start flattening from the root with "pza" prefix
            self.flatten_structure_node("pza".to_string(), &structure_node)?;
        }

        Ok(())
    }

    /// Recursively flatten a StructureNode
    ///
    fn flatten_structure_node(
        &mut self,
        level: String,
        node: &crate::fbs::panduza_generated::panduza::StructureNode,
    ) -> Result<(), String> {
        // Process attributes in this node
        if let Some(attributes) = node.attributes() {
            for i in 0..attributes.len() {
                let attr = attributes.get(i);
                if let Some(_attr_name) = attr.name() {
                    self.register_flat_entry_from_attribute(level.clone(), &attr)?;
                }
            }
        }

        // Process children nodes recursively
        if let Some(children) = node.children() {
            for i in 0..children.len() {
                let child = children.get(i);
                if let Some(child_name) = child.name() {
                    let child_level = format!("{}/{}", level, child_name);
                    self.flatten_structure_node(child_level, &child)?;
                }
            }
        }

        Ok(())
    }

    /// Register an attribute entry from StructureBuffer AttributeEntry
    ///
    fn register_flat_entry_from_attribute(
        &mut self,
        level: String,
        attr: &crate::fbs::panduza_generated::panduza::AttributeEntry,
    ) -> Result<(), String> {
        // Extract attribute information
        let attr_type = attr.type_().unwrap_or("unknown");
        let attr_mode = attr.mode().unwrap_or("unknown");

        // Create AttributeMetadata from the attribute information
        let metadata = AttributeMetadata::from_structure_buffer_attribute(
            level.clone(),
            attr_type,
            attr_mode,
        )?;

        // Insert into flat structure
        self.flat.insert(level, metadata);

        Ok(())
    }
    /// TODO REWORK THIS SERIOUSLY
    ///
    pub fn find_attribute<A: Into<String>>(&self, pattern: A) -> Option<AttributeMetadata> {
        let pattern_str: String = pattern.into();

        // Si le pattern ne commence pas par "pza", on préfixe par "*/"
        let pattern_str = if !pattern_str.starts_with("pza") {
            format!("*/{}", pattern_str)
        } else {
            pattern_str
        };

        // Utilisation de yash_fnmatch pour le matching wildcard
        if let Ok(pattern) = Pattern::parse(without_escape(&pattern_str)) {
            for (topic, metadata) in self.flat.iter() {
                if pattern.is_match(topic)
                    && topic.chars().rev().take(3).collect::<String>()
                        == pattern_str.chars().rev().take(3).collect::<String>()
                {
                    return Some(metadata.clone());
                }
            }
        } else {
            println!("Invalid pattern: {:?}", pattern_str);
        }
        None
    }

    ///
    ///
    pub fn initialized_notifier(&self) -> Arc<Notify> {
        self.initialized.clone()
    }

    // ------------------------------------------------------------------------

    /// Debug fonction to list all the received topics
    ///
    pub fn list_of_registered_topics(&self) -> Vec<String> {
        self.flat.keys().cloned().collect()
    }

    // ------------------------------------------------------------------------
}

#[derive(Clone, Debug)]
/// Object to manage the structure
///
pub struct Structure {
    /// initial data
    ///
    value: Arc<Mutex<StructureData>>,
}

impl Structure {
    pub async fn new(query: FifoChannelHandler<Reply>) -> Self {
        let structure_data = Arc::new(Mutex::new(StructureData::new()));

        let structure_data_clone = structure_data.clone();

        while let Ok(sample) = query.recv_async().await {
            // println!("LE SAMPLE : {:?}", sample.clone());
            match structure_data_clone.lock() {
                Ok(mut deref_value) => {
                    deref_value
                        .update(
                            sample
                                .result()
                                .expect("Failed to get result from Zenoh sample")
                                .payload()
                                .clone(),
                        )
                        .expect("Failed to update structure data with new payload");
                }
                Err(e) => {
                    println!("Error = {:?}", e);
                }
            }
        }

        Structure {
            value: structure_data,
        }
    }

    /// Try to find the element in the structure
    ///
    pub fn find_attribute<A: Into<String>>(&self, name: A) -> Option<AttributeMetadata> {
        match self.value.lock() {
            Ok(v) => v.find_attribute(name),
            Err(_) => None,
        }
    }

    /// Debug fonction to list all the received topics
    ///
    pub fn list_of_registered_topics(&self) -> Vec<String> {
        self.value
            .lock()
            .expect("Failed to acquire lock on structure data for listing topics")
            .list_of_registered_topics()
    }

    // pub fn update(&mut self, structure_buffer: StructureBuffer) {
    //     self.brut = structure_buffer;
    // }

    // pub async fn run(&mut self) {

    // }

    ///
    ///
    pub fn initialized_notifier(&self) -> Arc<Notify> {
        self.value
            .lock()
            .expect("Failed to acquire lock on structure data for getting notifier")
            .initialized_notifier()
    }
}
