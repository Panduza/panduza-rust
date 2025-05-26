use std::clone;
use std::time::Duration;
use tokio::time::sleep;

use super::{Error, IncomingUpdate, Options, PubSubEvent};
use bytes::Bytes;
use thiserror::Error as ThisError;
use zenoh::pubsub::Subscriber as ZenohSubscriber;
use zenoh::{
    handlers::DefaultHandler, matching::MatchingListener, open, sample::Sample, Config, Session,
};
use zenoh::{handlers::FifoChannelHandler, Result as ZenohResult};

#[derive(ThisError, Debug, Clone)]
pub enum SessionError {
    #[error("Cannot create session because {cause:?}")]
    SessionError { cause: String },
}

/// Set client Options
///
fn client_config(options: Options) -> Config {
    let conf = format!(
        r#"{{
            "mode": "client",
            "connect": {{
                "endpoints": ["quic/{}:{}"]
            }},
            "transport": {{
                "link": {{
                    "tls": {{
                        "root_ca_certificate": "{}"
                    }}
                }}
            }}
        }}"#,
        options.ip, options.port, options.ca_certificate
    );
    let config = Config::from_json5(&conf).unwrap();

    config
}

/// Start a Zenoh session
///
pub async fn new_connection(options: Options) -> Result<Session, SessionError> {
    let config = client_config(options);

    match open(config).await {
        Ok(session) => Ok(session),
        Err(e) => Err(SessionError::SessionError {
            cause: e.to_string(),
        }),
    }
}

// #[derive(Debug)]

// pub struct ZenohListener {
//     subscriber: ZenohSubscriber<FifoChannelHandler<Sample>>,
// }

// impl ZenohListener {
//     pub async fn new<A: Into<String>>(session: &Session, key_expr: A) -> ZenohResult<Self> {
//         let subscriber = session.declare_subscriber(key_expr.into()).await?;
//         Ok(Self { subscriber })
//     }

//     /// Écoute les messages et appelle le callback pour chaque message reçu.
//     pub async fn listen<F>(&self, mut callback: F)
//     where
//         F: FnMut(Sample) + Send,
//     {
//         while let Ok(sample) = self.subscriber.recv_async().await {
//             callback(sample);
//         }
//     }
// }

// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------

// #[derive(Clone)]
// ///
// ///
// pub struct Operator {
//     session: Session,
// }

// impl Operator {
//     pub fn new(session: Session) -> Self {
//         Self { session: session }
//     }

//     ///
//     ///
//     pub fn declare_publisher(&self, topic: String) -> Result<Publisher, Error> {
//         Ok(Publisher::new(self.session.clone(), topic))
//     }

//     ///
//     ///
//     pub fn declare_subscriber(&self) -> Result<Subscriber, Error> {
//         Ok(Subscriber::new(self.session.clone()))
//     }
// }

// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------

// #[derive(Clone, Debug)]
// ///
// ///
// pub struct Publisher {
//     session: Session,
//     topic: String,
// }

// impl Publisher {
//     pub fn new(session: Session, topic: String) -> Self {
//         Self {
//             session: session,
//             topic: topic,
//         }
//     }

//     /// Publish the payload
//     ///
//     pub async fn publish(&self, payload: Bytes) -> Result<(), Error> {
//         let publisher = self
//             .session
//             .declare_publisher(self.topic.clone())
//             .await
//             .unwrap();
//         publisher
//             .put(payload)
//             .await
//             .map_err(|e| Error::PublishError {
//                 topic: self.topic.clone(),
//                 cause: e.to_string(),
//             })
//     }
// }

// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------

// ///
// ///
// pub struct Subscriber {
//     session: Session,
// }

// impl Subscriber {
//     pub fn new(session: Session) -> Self {
//         Self { session: session }
//     }

//     ///
//     ///
//     pub async fn subscribe(&mut self, topic: String) -> Result<(), Error> {
//         self.session
//             .declare_subscriber(topic.clone())
//             .await
//             .map(|_| ())
//             .map_err(|e| Error::SubscribeError {
//                 topic: topic,
//                 cause: e.to_string(),
//             })
//     }
// }
