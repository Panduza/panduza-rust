use rumqttc::QoS;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::Mutex;

use crate::asyncv::MessageDispatcher;
use crate::AttributeError;

use super::MessageClient;
use super::OnMessageHandler;

use bytes::Bytes;

use async_trait::async_trait;

use crate::asyncv::AttributeBuilder;

use tokio::sync::Notify;

use crate::MessagePayloadManager;

/// Read Only Inner implementation of the message attribute
/// This inner implementation allow the public part to be cloneable easly
pub struct MessageAttributeRoInner<TYPE: MessagePayloadManager> {
    /// Reactor message dispatcher
    /// (to attach this attribute to the incoming messages)
    message_dispatcher: Weak<Mutex<MessageDispatcher>>,
    /// The message client (MQTT)
    pub message_client: MessageClient,

    /// The topic of the attribute
    topic: String,

    /// Current value of the attribute
    pub value: Option<TYPE>,

    ///
    change_notifier: Arc<Notify>,
}

impl<TYPE: MessagePayloadManager> MessageAttributeRoInner<TYPE> {
    /// Initialize the attribute
    /// Register the attribute on the message dispatcher then subscribe to att topic
    pub async fn init(
        &self,
        attribute: Arc<Mutex<dyn OnMessageHandler>>,
    ) -> Result<(), AttributeError> {
        self.register(attribute).await?;
        self.subscribe().await
    }

    /// Get the value of the attribute
    /// If None, the first value is not yet received
    pub fn get(&self) -> Option<TYPE> {
        return self.value;
    }

    /// Subscribe to the topic
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

    ///
    ///
    pub fn clone_change_notifier(&self) -> Arc<Notify> {
        self.change_notifier.clone()
    }
}

/// Allow creation from the builder
impl<TYPE: MessagePayloadManager> From<AttributeBuilder> for MessageAttributeRoInner<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        MessageAttributeRoInner {
            message_dispatcher: builder.message_dispatcher,
            message_client: builder.message_client,
            topic: builder.topic.as_ref().unwrap().clone(),
            value: None,
            change_notifier: Arc::new(Notify::new()),
        }
    }
}

/// Allow mutation into Arc pointer
impl<TYPE: MessagePayloadManager> Into<Arc<Mutex<MessageAttributeRoInner<TYPE>>>>
    for MessageAttributeRoInner<TYPE>
{
    fn into(self) -> Arc<Mutex<MessageAttributeRoInner<TYPE>>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl<TYPE: MessagePayloadManager> OnMessageHandler for MessageAttributeRoInner<TYPE> {
    async fn on_message(&mut self, data: &Bytes) {
        println!("ro_inner::on_message");
        let new_value = TYPE::from(data.to_vec());
        self.value = Some(new_value);
        self.change_notifier.notify_waiters();
    }
}
