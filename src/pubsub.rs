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

#[derive(Debug, Clone)]
///1
///
pub struct Options {
    pub ip: String,
    pub port: u16,
    pub namespace_pub: Option<String>,
    pub namespace_sub: Option<String>,
}

impl Options {
    pub fn new<T: Into<String>>(ip: T, port: u16, namespace_pub: Option<T>, namespace_sub: Option<T>) -> Self {
        Self {
            ip: ip.into(),
            port,
            namespace_pub: namespace_pub.map(|n| n.into()),
            namespace_sub: namespace_sub.map(|n| n.into()),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            ip: "localhost".to_string(),
            port: 1883,
            namespace_pub: None,
            namespace_sub: None,
        }
    }
}

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
