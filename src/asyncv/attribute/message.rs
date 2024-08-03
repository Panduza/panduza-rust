pub mod boolean;
mod core_members;
mod dispatcher;

use async_trait::async_trait;
use bytes::Bytes;

pub use core_members::MessageCoreMembers;
pub use dispatcher::MessageDispatcher;

pub use super::AttributeBuilder;

pub type MessageClient = rumqttc::AsyncClient;

pub type AttributeId = u32;

/// Trait to manage an message attribute (MQTT)
/// Sync version
#[async_trait]
pub trait OnMessageHandler: Send + Sync {
    async fn on_message(&mut self, data: &Bytes);
}

#[async_trait]
pub trait OnBooleanMessage: Send + Sync {
    async fn on_message_boolean(&mut self, id: AttributeId, data: bool);
}
