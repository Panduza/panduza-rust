mod zenoh_ps;
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
    pub mode: String,
    pub protocol: String,
    pub ip: String,
    pub port: u16,
    pub root_ca_certificate: String,
}

impl Options {
    pub fn new<T: Into<String>>(
        mode: T,
        protocol: T,
        ip: T,
        port: u16,
        root_ca_certificate: T,
    ) -> Self {
        Self {
            mode: mode.into(),
            protocol: protocol.into(),
            ip: ip.into(),
            port,
            root_ca_certificate: root_ca_certificate.into(),
        }
    }
}

pub use zenoh_ps::new_connection;
