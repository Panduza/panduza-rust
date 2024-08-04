mod dispatcher;
mod ro_attribute;
mod ro_inner;
mod rw_attribute;
mod rw_inner;

use async_trait::async_trait;
use bytes::Bytes;

pub use ro_inner::MessageAttributeRoInner;
pub use rw_inner::MessageAttributeRwInner;

pub use ro_attribute::MessageAttributeRo;
pub use rw_attribute::MessageAttributeRw;

pub use dispatcher::MessageDispatcher;

pub use super::AttributeBuilder;

pub use super::MessageClient;

/// Trait to manage an message attribute (MQTT)
/// Sync version
#[async_trait]
pub trait OnMessageHandler: Send + Sync {
    async fn on_message(&mut self, data: &Bytes);
}
