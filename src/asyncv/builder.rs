use std::sync::Weak;

use tokio::sync::Mutex;

use super::attribute::MessageAttributeRo;
use super::attribute::MessageAttributeRw;
pub use super::MessageClient;
pub use super::MessageDispatcher;
use crate::MessagePayloadManager;

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

/// Builder specialisation for Ro Attribute
pub struct AttributeRoBuilder {
    base: AttributeBuilder,
}

/// Builder specialisation for Rw Attribute
pub struct AttributeRwBuilder {
    base: AttributeBuilder,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

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

    pub fn with_ro_access(self) -> AttributeRoBuilder {
        AttributeRoBuilder { base: self }
    }

    pub fn with_rw_access(self) -> AttributeRwBuilder {
        AttributeRwBuilder { base: self }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

impl AttributeRoBuilder {
    pub async fn finish_with_message_type<TYPE: MessagePayloadManager>(
        self,
    ) -> MessageAttributeRo<TYPE> {
        MessageAttributeRo::from(self.base).init().await.unwrap()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

impl AttributeRwBuilder {
    pub async fn finish_with_message_type<TYPE: MessagePayloadManager>(
        self,
    ) -> MessageAttributeRw<TYPE> {
        MessageAttributeRw::from(self.base).init().await.unwrap()
    }
}
