use std::sync::Weak;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use bytes::Bytes;

use super::OnMessageHandler;

/// Data used by the core the dispatch input data
///
pub struct ReactorData {
    /// List of attributes to trigger on message
    message_attributes: HashMap<String, Weak<Mutex<dyn OnMessageHandler>>>,
}

impl ReactorData {
    /// Create a new ReactorData
    ///
    pub fn new() -> Self {
        Self {
            message_attributes: HashMap::new(),
        }
    }

    pub fn register_message_attribute(
        &mut self,
        topic: String,
        attribute: Arc<Mutex<dyn OnMessageHandler>>,
    ) {
        self.message_attributes
            .insert(topic, Arc::downgrade(&attribute));
    }

    /// Trigger the on_message of the attribute
    ///
    pub async fn trigger_on_change(&self, topic: &str, new_value: &Bytes) {
        // println!("{:?}", self.message_attributes.keys());
        if let Some(attribute) = self.message_attributes.get(topic) {
            match attribute.upgrade() {
                Some(attribute) => {
                    attribute.lock().await.on_message(new_value);
                }
                None => {
                    println!("Attribute not found");
                }
            }
        }
    }
}
