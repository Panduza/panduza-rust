use std::collections::HashMap;
use std::sync::Arc;

use bytes::Bytes;
use rumqttc::QoS;
use thiserror::Error as ThisError;
use tokio::sync::Mutex;

type MessageEventLoop = rumqttc::EventLoop;

use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use crate::pubsub::mqtt::create_connection;
use crate::pubsub::mqtt::MqttListener;
use crate::pubsub::mqtt::MqttOperator;
use crate::pubsub::PubSubListener;
use crate::pubsub::PubSubOperator;
use crate::PubSubOptions;

/// Error of the pub/sub protocol API
///
#[derive(ThisError, Debug, Clone)]
pub enum RouterError {
    #[error("Cannot create connection because {cause:?}")]
    ConnectionError { cause: String },
}

#[derive(Clone, Debug)]
/// Object that tells to the router how to dispatch the messages
///
pub struct Rule {
    pub topic: String,
    pub sender: Sender<Bytes>,
}

/// Channel Sender of rules
///
pub type RuleSender = tokio::sync::mpsc::Sender<Rule>;

/// Channel Receiver of rules
///
pub type RuleReceiver = tokio::sync::mpsc::Receiver<Rule>;

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct Router<O: PubSubOperator, L: PubSubListener> {
    ///
    ///
    operator: O,

    ///
    ///
    listener: L,

    ///
    ///
    rules_sender: Sender<Rule>,

    ///
    ///
    rules_receiver: Receiver<Rule>,

    /// List of rules to dispatch the messages
    ///
    routes: HashMap<String, Sender<Bytes>>,
}

impl<O: PubSubOperator, L: PubSubListener> Router<O, L> {
    ///
    ///
    pub fn new(operator: O, listener: L) -> Self {
        //
        // Create the rules channel
        let (rules_sender, rules_receiver) = tokio::sync::mpsc::channel(100);

        Self {
            operator: operator,
            listener: listener,
            rules_sender: rules_sender,
            rules_receiver: rules_receiver,
            routes: HashMap::new(),
        }
    }

    ///
    ///
    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(rule) = self.rules_receiver.recv() => {
                    // println!("??? Rule = {:?}", rule);
                    self.routes.insert(rule.topic, rule.sender);

                }
                Ok(event) = self.listener.poll() => {

                    println!("{:?}", event);
                    // let payload = packet.payload;
                            // let payload_str = std::str::from_utf8(&payload).unwrap();
                            // // println!("!!! Received = {:?} {:?}", payload_str, packet.topic);

                            // if let Some(sender) = self.routes.get(&packet.topic) {
                            //     // println!("............ROUUUTe");
                            //     sender.send(Bytes::from(payload)).await.unwrap();
                            // }
                            // else {
                            //     // println!("-------- !!! No route for {:?}", packet.topic);
                            // }


                }
            }
        }
    }
}

///
///
pub fn create_mqtt_router(
    options: PubSubOptions,
) -> Result<Router<impl PubSubOperator, impl PubSubListener>, RouterError> {
    let (op, li) = create_connection(options).map_err(|e| RouterError::ConnectionError {
        cause: e.to_string(),
    })?;

    Ok(Router::new(op, li))
}
