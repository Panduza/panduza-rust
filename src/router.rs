// use crate::pubsub;
// use crate::pubsub::new_connection;
// use crate::pubsub::Listener;
// use crate::pubsub::Operator;
// use crate::pubsub::Publisher;
// use crate::task_monitor::TaskMonitorLink;
// use bytes::Bytes;
// use std::collections::HashMap;
// use thiserror::Error as ThisError;
// use tokio::sync::mpsc::Receiver;
// use tokio::sync::mpsc::Sender;

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
// /// Object that allow the router to create new routes
// ///
// pub struct RouterHandler {
//     /// Object that allow network pub/sub operations
//     ///
//     pub operator: Operator,

//     /// Object to send other route to manage
//     ///
//     pub rules_sender: Sender<Rule>,
// }

// impl RouterHandler {
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
//         let sub = self
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
//     pub fn register_publisher(
//         &self,
//         topic: String,
//         retain: bool,
//     ) -> Result<Publisher, pubsub::Error> {
//         self.operator.declare_publisher(topic, retain)
//     }
// }

// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------

// /// Generic Pub/Sub Router for Panduza
// ///
// /// This object manage a pub/sub connection and provide helper to build routes
// ///
// pub struct Router {
//     /// Object that allow network pub/sub operations
//     ///
//     operator: Operator,

//     /// Object to listen incoming messages
//     ///
//     listener: Listener,

//     /// Object to send more rules
//     ///
//     rules_sender: Sender<Rule>,

//     ///
//     ///
//     rules_receiver: Receiver<Rule>,

//     /// List of rules to dispatch the messages
//     ///
//     routes: HashMap<String, Sender<Bytes>>,
// }

// impl Router {
//     /// Create a new instance
//     ///
//     pub fn new(operator: Operator, listener: Listener) -> Self {
//         //
//         // Create the rules channel
//         let (rules_sender, rules_receiver) = tokio::sync::mpsc::channel(100);

//         Self {
//             operator: operator,
//             listener: listener,
//             rules_sender: rules_sender,
//             rules_receiver: rules_receiver,
//             routes: HashMap::new(),
//         }
//     }

//     /// Start the router and just return the handler to interact with him
//     ///
//     pub fn start(
//         self,
//         monitor_link: Option<TaskMonitorLink>,
//     ) -> Result<RouterHandler, RouterError> {
//         // Create an handler before end
//         let handler = self.handler();

//         // Create the task
//         let task = tokio::spawn(async move {
//             self.run().await;
//             Err("router stop !".to_string())
//         });

//         // Send task to a monitor
//         if let Some(link) = monitor_link {
//             if let Err(e) = link.blocking_send(("router".to_string(), task)) {
//                 return Err(RouterError::ChannelError {
//                     cause: e.to_string(),
//                 });
//             }
//         }

//         // Ok
//         Ok(handler)
//     }

//     /// Create a new handler
//     ///
//     pub fn handler(&self) -> RouterHandler {
//         RouterHandler {
//             operator: self.operator.clone(),
//             rules_sender: self.rules_sender.clone(),
//         }
//     }

//     /// Main run loop
//     ///
//     pub async fn run(mut self) {
//         loop {
//             tokio::select! {
//                 Some(rule) = self.rules_receiver.recv() => {
//                     self.routes.insert(rule.topic, rule.sender);
//                 }
//                 Ok(event) = self.listener.poll() => {
//                     match event {
//                         crate::pubsub::PubSubEvent::IncomingUpdate(incoming_update) =>
//                         {
//                             if let Some(sender) = self.routes.get(&incoming_update.topic) {
//                                 if sender.send(incoming_update.payload).await.is_err() {
//                                     // Remove the route if the sender is dead
//                                     self.routes.remove(&incoming_update.topic);
//                                     // println!("-------- !!! Removed dead route for {:?}", incoming_update.topic);
//                                 }
//                             } else {
//                                 println!("-------- !!! No route for {:?}", incoming_update.topic);
//                             }
//                         },
//                     }
//                 }
//             }
//         }
//     }
// }

// /// Create a new MQTT Router
// ///
// pub fn new_router(options: pubsub::Options) -> Result<Router, RouterError> {
//     //
//     // Create a new network connection
//     let (op, li) = new_connection(options).map_err(|e| RouterError::ConnectionError {
//         cause: e.to_string(),
//     })?;

//     //
//     // Return the router
//     Ok(Router::new(op, li))
// }
