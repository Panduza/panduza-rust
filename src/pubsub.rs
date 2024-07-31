mod mqtt;

use bytes::Bytes;
use std::fmt::Debug;
use thiserror::Error as ThisError;

/// Error of the pub/sub protocol API
///
#[derive(ThisError, Debug, Clone)]
pub enum Error {
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
pub struct Options {}

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

// MQTT Implementation
pub use mqtt::new_connection;
pub use mqtt::Listener;
pub use mqtt::Operator;
pub use mqtt::Publisher;
pub use mqtt::Subscriber;
