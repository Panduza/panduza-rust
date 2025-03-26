// use crate::pubsub;
// use crate::pubsub::new_connection;
// // use crate::pubsub::Listener;
// use crate::pubsub::Operator;
// use crate::pubsub::Publisher;
// use crate::task_monitor::TaskMonitorLink;
// use bytes::Bytes;
// use std::collections::HashMap;
// use std::sync::Arc;
// use thiserror::Error as ThisError;
// use tokio::sync::mpsc::Receiver;
// use tokio::sync::mpsc::Sender;
// use zenoh::handlers::DefaultHandler;
// use {zenoh::handlers::FifoChannelHandler, zenoh::sample::Sample, zenoh::Session};

// /// Error of the pub/sub protocol API
// ///
// #[derive(ThisError, Debug, Clone)]
// pub enum RouterError {
//     #[error("Cannot create connection because {cause:?}")]
//     ConnectionError { cause: String },
//     #[error("A channel refuse operation because {cause:?}")]
//     ChannelError { cause: String },
// }

// #[derive(Clone, Debug)]
// /// Object that tells to the router how to dispatch the messages
// ///
// pub struct Rule {
//     pub topic: String,
//     pub sender: Sender<Bytes>,
// }

// /// Channel Sender of rules
// ///
// pub type RuleSender = tokio::sync::mpsc::Sender<Rule>;

// /// Channel Receiver of rules
// ///
// pub type RuleReceiver = tokio::sync::mpsc::Receiver<Rule>;

// /// Receiver of data payload
// ///
// pub type DataReceiver = tokio::sync::mpsc::Receiver<Bytes>;

// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------

// #[derive(Clone)]

// /// Object that allow the session to create new routes
// ///
// pub struct SessionHandler {
//     pub operator: Operator,

//     pub rules_sender: Sender<Rule>,
// }
// impl SessionHandler {
//     /// Register a new route
//     ///
//     pub async fn register_listener<A: Into<String>>(
//         &self,
//         topic: A,
//         channel_size: usize,
//     ) -> Result<DataReceiver, String> {
//         //
//         // Create the data channel
//         let (data_sender, data_receiver) = tokio::sync::mpsc::channel::<Bytes>(channel_size);
//         //
//         //
//         let t = topic.into();

//         //
//         // Create the rules on the router
//         self.rules_sender
//             .send(Rule {
//                 topic: t.clone(),
//                 sender: data_sender,
//             })
//             .await
//             .map_err(|e| e.to_string())?;

//         //
//         //
//         let mut sub = self
//             .operator
//             .declare_subscriber()
//             .map_err(|e| e.to_string())?;
//         sub.subscribe(t).await.map_err(|e| e.to_string())?;

//         //
//         // Return the receiver
//         Ok(data_receiver)
//     }

//     ///
//     ///
//     pub fn register_publisher(&self, topic: String) -> Result<Publisher, pubsub::Error> {
//         self.operator.declare_publisher(topic)
//     }
// }
