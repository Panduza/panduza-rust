pub mod attribute;
mod dispatcher;

use async_trait::async_trait;
use bytes::Bytes;

pub use dispatcher::MessageDispatcher;

pub use super::AttributeBuilder;

pub type MessageClient = rumqttc::AsyncClient;

/// Trait to manage an message attribute (MQTT)
/// Sync version
#[async_trait]
pub trait OnMessageHandler: Send + Sync {
    async fn on_message(&mut self, data: &Bytes);
}
