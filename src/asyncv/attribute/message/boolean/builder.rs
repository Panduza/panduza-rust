use std::sync::Weak;

use tokio::sync::Mutex;

use crate::asyncv::attribute::message::AttributeId;

use super::AttributeBoolean;
use super::AttributeBuilder;

pub use super::MessageClient;
pub use super::MessageDispatcher;

pub struct BuilderBoolean {
    pub id: AttributeId,
    /// The mqtt client
    pub message_client: MessageClient,

    /// The Object that allow the reactor to dispatch
    /// incoming messages on attributes
    pub message_dispatcher: Weak<Mutex<MessageDispatcher>>,

    /// Topic of the attribute
    pub topic: Option<String>,
}

impl BuilderBoolean {
    /// New boolean builder
    pub fn new(parent_builder: AttributeBuilder) -> BuilderBoolean {
        BuilderBoolean {
            id: parent_builder.id,
            message_client: parent_builder.message_client,
            message_dispatcher: parent_builder.message_dispatcher,
            topic: parent_builder.topic,
        }
    }

    pub fn finish(self) -> AttributeBoolean {
        AttributeBoolean::new(self)
    }
}
