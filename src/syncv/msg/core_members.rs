use rumqttc::{Client, QoS};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;

use crate::AttributeError;

use super::OnMessageHandler;
use super::ReactorData;

/// The core data of the attribute
/// Those attributes will be moved between Att types
#[derive(Clone)]
pub struct CoreMembers {
    /// The data of the reactor, to be able to subscribe to the
    /// reactor and route messages to the attribute
    reactor_data: Weak<Mutex<ReactorData>>,

    /// The mqtt client
    mqtt_client: Client,

    /// The topic of the attribute
    topic: String,

    /// The topic for commands
    topic_cmd: String,
}

impl CoreMembers {
    /// Create a new core data
    pub fn new(reactor_data: Weak<Mutex<ReactorData>>, topic: String, mqtt_client: Client) -> Self {
        Self {
            reactor_data: reactor_data,
            mqtt_client: mqtt_client,
            topic: topic.clone(),
            topic_cmd: format!("{}/cmd", topic),
        }
    }

    /// Initialize the attribute
    ///
    pub fn init(&self, attribute: Arc<Mutex<dyn OnMessageHandler>>) -> Result<(), AttributeError> {
        self.register(attribute)?;
        self.subscribe()
    }

    /// Publish a command
    ///
    pub fn publish<V>(&self, value: V) -> Result<(), AttributeError>
    where
        V: Into<Vec<u8>>,
    {
        self.mqtt_client
            .publish(&self.topic_cmd, QoS::AtMostOnce, true, value)
            .map_err(|e| AttributeError::Message(e))
    }

    /// Subscribe to the topic
    ///
    pub fn subscribe(&self) -> Result<(), AttributeError> {
        // no need to store the att topic
        let topic_att = format!("{}/att", self.topic);
        self.mqtt_client
            .subscribe(topic_att, QoS::AtMostOnce)
            .map_err(|e| AttributeError::Message(e))
    }

    /// Register the attribute to the reactor
    ///
    pub fn register(
        &self,
        attribute: Arc<Mutex<dyn OnMessageHandler>>,
    ) -> Result<(), AttributeError> {
        // no need to store the att topic
        let topic_att = format!("{}/att", self.topic);
        self.reactor_data
            .upgrade()
            .ok_or(AttributeError::InternalPointerUpgrade)?
            .lock()
            .map_err(|e| AttributeError::InternalMutex(e.to_string()))?
            .register_message_attribute(topic_att, attribute);
        Ok(())
    }
}
