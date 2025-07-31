use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use zenoh::Session;

use super::std_obj::StdObjAttribute;
use super::CallbackId;
use crate::fbs::PzaBuffer;
use crate::fbs::StructureBuffer;
use crate::AttributeMetadata;

/// High-level wrapper for managing structure attributes with tree-like data representation
///
#[derive(Clone, Debug)]
pub struct StructureAttribute {
    /// Flat version of the structure to ease find algorithms
    pub flat: Arc<Mutex<HashMap<String, AttributeMetadata>>>,

    /// Internal generic implementation based on an already design manager
    pub inner: StdObjAttribute<StructureBuffer>,
}

impl StructureAttribute {
    // ------------------------------------------------------------------------

    /// Return the last structure value received
    pub async fn get(&self) -> Option<StructureBuffer> {
        self.inner.get().await
    }

    // ------------------------------------------------------------------------

    /// Return the last structure value received as json string
    pub async fn get_as_json_string(&self) -> Option<String> {
        if let Some(buffer) = self.inner.get().await {
            let json_value = buffer.as_json();
            Some(json_value.to_string())
        } else {
            None
        }
    }

    // ------------------------------------------------------------------------
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        let inner = StdObjAttribute::<StructureBuffer>::new(session, metadata).await;
        let flat = Arc::new(Mutex::new(HashMap::new()));

        let instance = Self {
            flat: flat.clone(),
            inner,
        };

        // Add callback to update flat whenever a new StructureBuffer is received
        instance
            .inner
            .add_callback(
                {
                    let flat_ref = flat.clone();
                    let base_topic = instance.metadata().topic.clone();
                    move |buffer: StructureBuffer| {
                        let flat_clone = flat_ref.clone();
                        let topic_clone = base_topic.clone();
                        Box::pin(async move {
                            let mut flat = flat_clone.lock().await;
                            Self::update_flat_from_buffer(&mut flat, &buffer, &topic_clone);
                        })
                    }
                },
                None::<fn(&StructureBuffer) -> bool>,
            )
            .await;

        // Initialize flat from current buffer if available
        if let Some(buffer) = instance.inner.get().await {
            let mut flat_guard = instance.flat.lock().await;
            Self::update_flat_from_buffer(&mut flat_guard, &buffer, &instance.metadata().topic);
        }

        instance
    }

    // ------------------------------------------------------------------------

    /// Updates the flat HashMap from a StructureBuffer
    ///
    fn update_flat_from_buffer(
        flat: &mut HashMap<String, AttributeMetadata>,
        buffer: &StructureBuffer,
        base_topic: &str,
    ) {
        // Clear the existing flat structure
        flat.clear();

        // Get the message from the buffer
        let message = buffer.as_message();

        // Extract the Structure from the payload
        if let Some(structure) = message.payload_as_structure() {
            // Extract instance path from base_topic (remove /att suffix)
            let instance_path = if base_topic.ends_with("/att") {
                &base_topic[..base_topic.len() - 4]
            } else {
                base_topic
            };

            // Start flattening from the root
            Self::flatten_structure_node(flat, instance_path.to_string(), &structure);
        }
    }

    // ------------------------------------------------------------------------

    /// Recursively flatten a Structure node
    ///
    fn flatten_structure_node(
        flat: &mut HashMap<String, AttributeMetadata>,
        current_path: String,
        node: &crate::fbs::panduza_generated::panduza::Structure,
    ) {
        // Get node name, if empty skip this node
        let node_name = match node.name() {
            Some(name) if !name.is_empty() => name,
            _ => return,
        };

        // Build the current topic path
        let new_path = if current_path.is_empty() {
            node_name.to_string()
        } else {
            format!("{}/{}", current_path, node_name)
        };

        // If this node has both type and mode, it's a leaf attribute
        if let (Some(attr_type), Some(attr_mode)) = (node.type_(), node.mode()) {
            // Create AttributeMetadata from the structure information
            if let Ok(metadata) = crate::AttributeMetadata::from_structure_buffer_attribute(
                new_path.clone(),
                attr_type,
                attr_mode,
            ) {
                flat.insert(new_path.clone(), metadata);
            }
        }

        // Process children nodes recursively
        if let Some(children) = node.children() {
            for i in 0..children.len() {
                let child = children.get(i);
                Self::flatten_structure_node(flat, new_path.clone(), &child);
            }
        }
    }

    // ------------------------------------------------------------------------

    /// Waits for a specific StructureBuffer value matching predicate
    ///
    pub async fn wait_for_value<F>(
        &self,
        predicate: F,
        timeout: Option<Duration>,
    ) -> Result<(), String>
    where
        F: Fn(&StructureBuffer) -> bool + Send + Sync + 'static,
    {
        // Use the inner implementation and discard the actual value
        self.inner.wait_for_value(predicate, timeout).await?;
        Ok(())
    }

    // ------------------------------------------------------------------------

    /// Registers a callback triggered on StructureBuffer reception
    ///
    pub async fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(StructureBuffer) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static,
        C: Fn(&StructureBuffer) -> bool + Send + Sync + 'static,
    {
        self.inner.add_callback(callback, condition).await
    }

    // ------------------------------------------------------------------------

    /// Removes a callback by its ID
    ///
    pub async fn remove_callback(&self, callback_id: CallbackId) -> bool {
        self.inner.remove_callback(callback_id).await
    }

    // ------------------------------------------------------------------------

    /// Provides read-only access to attribute metadata
    ///
    pub fn metadata(&self) -> &AttributeMetadata {
        self.inner.metadata()
    }

    // ------------------------------------------------------------------------

    /// Use the flat field to find the topic that match the wildcard pattern
    ///
    pub async fn find_attribute<A: Into<String>>(&self, pattern: A) -> Option<AttributeMetadata> {
        let pattern_str = pattern.into();
        let flat_guard = self.flat.lock().await;

        // Simple pattern matching - exact match for now
        // Could be extended to support wildcards like * and ?
        if let Some(metadata) = flat_guard.get(&pattern_str) {
            return Some(metadata.clone());
        }

        // If exact match fails, try pattern matching with wildcards
        for (topic, metadata) in flat_guard.iter() {
            if Self::wildcard_match(&pattern_str, topic) {
                return Some(metadata.clone());
            }
        }

        None
    }

    // ------------------------------------------------------------------------

    /// Simple wildcard pattern matching helper
    ///
    fn wildcard_match(pattern: &str, text: &str) -> bool {
        // Simple implementation supporting * and ? wildcards
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let text_chars: Vec<char> = text.chars().collect();

        Self::wildcard_match_recursive(&pattern_chars, &text_chars, 0, 0)
    }

    // ------------------------------------------------------------------------

    /// Recursive helper for wildcard matching
    ///
    fn wildcard_match_recursive(
        pattern: &[char],
        text: &[char],
        p_idx: usize,
        t_idx: usize,
    ) -> bool {
        // End of pattern
        if p_idx == pattern.len() {
            return t_idx == text.len();
        }

        // End of text but pattern has more characters
        if t_idx == text.len() {
            // Check if remaining pattern is all '*'
            return pattern[p_idx..].iter().all(|&c| c == '*');
        }

        match pattern[p_idx] {
            '*' => {
                // Try matching zero characters
                if Self::wildcard_match_recursive(pattern, text, p_idx + 1, t_idx) {
                    return true;
                }
                // Try matching one or more characters
                Self::wildcard_match_recursive(pattern, text, p_idx, t_idx + 1)
            }
            '?' => {
                // Match any single character
                Self::wildcard_match_recursive(pattern, text, p_idx + 1, t_idx + 1)
            }
            c => {
                // Exact character match
                if text[t_idx] == c {
                    Self::wildcard_match_recursive(pattern, text, p_idx + 1, t_idx + 1)
                } else {
                    false
                }
            }
        }
    }

    // ------------------------------------------------------------------------
}
