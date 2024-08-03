use std::sync::Weak;

use tokio::sync::Mutex;

use super::attribute::message::boolean::BuilderBoolean;
use super::attribute::message::AttributeId;
pub use super::MessageClient;
pub use super::MessageDispatcher;

static mut ID_POOL: AttributeId = 0;

/// Object that allow to build an generic attribute
///
pub struct AttributeBuilder {
    pub id: AttributeId,
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
        unsafe {
            let id = ID_POOL;
            ID_POOL += 1;
            AttributeBuilder {
                id: id,
                message_client,
                message_dispatcher,
                topic: None,
            }
        }
    }

    /// Attach a topic
    pub fn with_topic<T: Into<String>>(mut self, topic: T) -> Self {
        self.topic = Some(topic.into());
        self
    }

    pub fn with_type_boolean(self) -> BuilderBoolean {
        BuilderBoolean::new(self)
    }
}
