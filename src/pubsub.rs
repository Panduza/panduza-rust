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

#[derive(Debug)]
///
///
pub struct Options {
    pub ip: String,
    pub port: u16,
}

impl Options {
    pub fn new<T: Into<String>>(ip: T, port: u16) -> Self {
        Self {
            ip: ip.into(),
            port,
        }
    }
}

// impl Default for Options {
//     fn default() -> Self {
//         Self {
//             ip: "127.0.0.1".to_string(),
//             port: 1883,
//         }
//     }
// }

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
