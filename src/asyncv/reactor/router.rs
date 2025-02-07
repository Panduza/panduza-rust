use std::collections::HashMap;
use std::sync::Arc;

use bytes::Bytes;
use rumqttc::QoS;
use tokio::sync::Mutex;

type MessageEventLoop = rumqttc::EventLoop;

use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use crate::asyncv::MessageClient;

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

pub struct Router {
    /// The mqtt client
    pub message_client: MessageClient,

    /// The message client
    ///
    message_event_loop: MessageEventLoop,

    /// Channel to receive the rules
    ///
    rules_receiver: Receiver<Rule>,

    /// List of rules to dispatch the messages
    ///
    routes: HashMap<String, Sender<Bytes>>,
}

impl Router {
    pub fn new(
        message_client: MessageClient,
        message_event_loop: MessageEventLoop,
        rules_receiver: Receiver<Rule>,
    ) -> Router {
        Router {
            message_client: message_client,
            message_event_loop: message_event_loop,
            rules_receiver: rules_receiver,
            routes: HashMap::new(),
        }
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(rule) = self.rules_receiver.recv() => {
                    println!("??? Rule = {:?}", rule);
                    self.routes.insert(rule.topic, rule.sender);

                }
                Ok(event) = self.message_event_loop.poll() => {
                    // println!("*** Notification = {:?}", event);
                    match event {
                    rumqttc::Event::Incoming(incoming) => {
                        match incoming {
                        rumqttc::Packet::Publish(packet) => {
                            let payload = packet.payload;
                            let payload_str = std::str::from_utf8(&payload).unwrap();
                            // println!("!!! Received = {:?} {:?}", payload_str, packet.topic);

                            if let Some(sender) = self.routes.get(&packet.topic) {
                                println!("............ROUUUTe");
                                sender.send(Bytes::from(payload)).await.unwrap();
                            }
                            else {
                                println!("-------- !!! No route for {:?}", packet.topic);
                            }

                        }
                        _ => {}
                        }
                    }
                    rumqttc::Event::Outgoing(outgoing) => {
                        match outgoing {
                        rumqttc::Outgoing::Publish(packet) => {
                            // println!("Publish = {:?}", packet);
                        }
                        rumqttc::Outgoing::Subscribe(p) => {
                            // println!("Subscribe = {:?}", p);
                        }
                        _ => {}
                        }
                    }
                    }
                }
            }
        }
    }
}
