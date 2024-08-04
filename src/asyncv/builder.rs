use std::sync::Weak;

use tokio::sync::Mutex;

use super::attribute::MessageAttribute;

pub use super::MessageClient;
pub use super::MessageDispatcher;
use crate::AttributePayloadManager;

/// Object that allow to build an generic attribute
///
pub struct AttributeBuilder {
    /// The mqtt client
    pub message_client: MessageClient,

    /// The Object that allow the reactor to dispatch
    /// incoming messages on attributes
    pub message_dispatcher: Weak<Mutex<MessageDispatcher>>,

    /// Topic of the attribute
    pub topic: Option<String>,

    /// True if the attribute is readonly
    pub is_read_only: bool,
}

impl AttributeBuilder {
    /// Create a new builder
    pub fn new(
        message_client: MessageClient,
        message_dispatcher: Weak<Mutex<MessageDispatcher>>,
    ) -> AttributeBuilder {
        AttributeBuilder {
            message_client,
            message_dispatcher,
            topic: None,
            is_read_only: true,
        }
    }

    /// Attach a topic
    pub fn with_topic<T: Into<String>>(mut self, topic: T) -> Self {
        self.topic = Some(topic.into());
        self
    }

    pub fn as_read_write(mut self) -> Self {
        self.is_read_only = false;
        self
    }

    pub fn build_with_message_type<TYPE: AttributePayloadManager>(self) -> MessageAttribute<TYPE> {
        MessageAttribute::new(self)
    }
}
