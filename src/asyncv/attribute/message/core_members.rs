use rumqttc::QoS;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::Mutex;

use crate::AttributeError;

use super::MessageClient;
use super::MessageDispatcher;
use super::OnMessageHandler;

/// The core data of the attribute
/// Those attributes will be moved between Att types
#[derive(Clone)]
pub struct MessageCoreMembers {
    /// The data of the reactor, to be able to subscribe to the
    /// reactor and route messages to the attribute
    message_dispatcher: Weak<Mutex<MessageDispatcher>>,

    /// The mqtt client
    message_client: MessageClient,

    /// The topic of the attribute
    topic: String,

    /// The topic for commands
    topic_cmd: String,
}

impl MessageCoreMembers {
    /// Create a new core data
    pub fn new(
        message_client: MessageClient,
        message_dispatcher: Weak<Mutex<MessageDispatcher>>,
        topic: String,
    ) -> Self {
        Self {
            message_dispatcher: message_dispatcher,
            message_client: message_client,
            topic: topic.clone(),
            topic_cmd: format!("{}/cmd", topic),
        }
    }

    /// Initialize the attribute
    ///
    pub async fn init(
        &self,
        attribute: Arc<Mutex<dyn OnMessageHandler>>,
    ) -> Result<(), AttributeError> {
        self.register(attribute).await?;
        self.subscribe().await
    }

    /// Publish a command
    ///
    pub async fn publish<V>(&self, value: V) -> Result<(), AttributeError>
    where
        V: Into<Vec<u8>>,
    {
        self.message_client
            .publish(&self.topic_cmd, QoS::AtMostOnce, true, value)
            .await
            .map_err(|e| AttributeError::Message(e))
    }

    /// Subscribe to the topic
    ///
    pub async fn subscribe(&self) -> Result<(), AttributeError> {
        // no need to store the att topic
        let topic_att = format!("{}/att", self.topic);
        self.message_client
            .subscribe(topic_att, QoS::AtMostOnce)
            .await
            .map_err(|e| AttributeError::Message(e))
    }

    /// Register the attribute to the reactor
    ///
    pub async fn register(
        &self,
        attribute: Arc<Mutex<dyn OnMessageHandler>>,
    ) -> Result<(), AttributeError> {
        // no need to store the att topic
        let topic_att = format!("{}/att", self.topic);
        self.message_dispatcher
            .upgrade()
            .ok_or(AttributeError::InternalPointerUpgrade)?
            .lock()
            .await
            // .map_err(|e| AttributeError::InternalMutex(e.to_string()))?
            .register_message_attribute(topic_att, attribute);
        Ok(())
    }
}
