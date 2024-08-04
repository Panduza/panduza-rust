use std::sync::Weak;

use tokio::sync::Mutex;

use super::attribute::message::attribute::Attribute;
use super::attribute::message::attribute::AttributePayloadManager;
pub use super::MessageClient;
pub use super::MessageDispatcher;

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
        }
    }

    /// Attach a topic
    pub fn with_topic<T: Into<String>>(mut self, topic: T) -> Self {
        self.topic = Some(topic.into());
        self
    }

    pub fn build_with_payload_type<TYPE: AttributePayloadManager>(self) -> Attribute<TYPE> {
        Attribute::new(self)
    }
}
