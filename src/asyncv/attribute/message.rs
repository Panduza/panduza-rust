pub mod boolean;
mod core_members;
mod dispatcher;

use async_trait::async_trait;
use bytes::Bytes;

pub use core_members::MessageCoreMembers;
pub use dispatcher::MessageDispatcher;

pub use super::AttributeBuilder;

pub type MessageClient = rumqttc::AsyncClient;

/// Trait to manage an message attribute (MQTT)
/// Sync version
#[async_trait]
pub trait OnMessageHandler: Send + Sync {
    async fn on_message(&mut self, data: &Bytes);
}
