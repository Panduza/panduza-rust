pub mod mqtt;

use async_trait::async_trait;
use bytes::Bytes;
use thiserror::Error as ThisError;

/// Error of the pub/sub protocol API
///
#[derive(ThisError, Debug, Clone)]
pub enum PubSubError {
    #[error("Cannot publish message ({pyl_size:?} bytes) on topic {topic:?} because {cause:?}")]
    PublishError {
        topic: String,
        pyl_size: usize,
        cause: String,
    },
}

pub struct PubSubOptions {}

///
///
pub enum PubSubEvent {}

#[async_trait]
pub trait Publisher {
    ///
    ///
    async fn publish(&self, payload: Bytes) -> Result<(), PubSubError>;
}

#[async_trait]
pub trait Subscriber {
    ///
    ///
    async fn subscribe<S: Into<String>>(&self, topic: S) -> Result<(), PubSubError>;
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
pub trait PubSubOperator {
    ///
    ///
    fn declare_publisher(&self) -> Result<impl Publisher, PubSubError>;

    ///
    ///
    fn declare_subscriber(&self) -> Result<impl Subscriber, PubSubError>;
}
