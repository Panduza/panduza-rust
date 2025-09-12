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
    pub root_ca_certificate: String,
    pub connect_certificate: String,
    pub connect_private_key: String,
    pub namespace: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            ip: "127.0.0.1".to_string(),
            port: 7447,
            root_ca_certificate: "./credentials/certificates/root_ca_certificate.pem".to_string(),
            connect_certificate: "./credentials/certificates/writer_certificate.pem".to_string(),
            connect_private_key: "./credentials/keys/writer_private_key.pem".to_string(),
            namespace: None,
        }
    }
}

impl Options {
    pub fn new<T: Into<String>>(
        ip: T,
        port: u16,
        root_ca_certificate: T,
        connect_certificate: T,
        connect_private_key: T,
        namespace: Option<T>,
    ) -> Self {
        Self {
            ip: ip.into(),
            port,
            root_ca_certificate: root_ca_certificate.into(),
            connect_certificate: connect_certificate.into(),
            connect_private_key: connect_private_key.into(),
            namespace: namespace.map(|n| n.into()),
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
