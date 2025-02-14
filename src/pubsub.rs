pub mod mqtt;

use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use bytes::Bytes;
use thiserror::Error as ThisError;

/// Error of the pub/sub protocol API
///
#[derive(ThisError, Debug, Clone)]
pub enum PubSubError {
    #[error("Cannot publish message on topic {topic:?} because {cause:?}")]
    PublishError { topic: String, cause: String },

    #[error("Cannot subscribe to topic {topic:?} because {cause:?}")]
    SubscribeError { topic: String, cause: String },

    #[error("Error listening network because {cause:?}")]
    ListenError { cause: String },
}

#[derive(Default, Debug)]
///
///
pub struct PubSubOptions {}

#[derive(Debug)]
///
///
pub struct IncomingUpdate {
    pub topic: String,
    pub payload: Bytes,
}

#[derive(Debug)]
///
///
pub enum PubSubEvent {
    IncomingUpdate(IncomingUpdate),
}

#[async_trait]
pub trait Publisher: Debug {
    ///
    ///
    async fn publish(&self, payload: Bytes) -> Result<(), PubSubError>;
}

#[async_trait]
pub trait Subscriber {
    ///
    ///
    async fn subscribe(&self, topic: String) -> Result<(), PubSubError>;
}

/// Object that make the connection alive and listen incoming messages
///
#[async_trait]
pub trait PubSubListener {
    ///
    ///
    async fn poll(&mut self) -> Result<PubSubEvent, PubSubError>;
}

/// Entry point to declare operator and use the connection
///
pub trait PubSubOperator: Clone {
    ///
    ///
    fn declare_publisher(
        &self,
        topic: String,
        retain: bool,
    ) -> Result<Arc<dyn Publisher>, PubSubError>;

    ///
    ///
    fn declare_subscriber(&self) -> Result<impl Subscriber, PubSubError>;
}
