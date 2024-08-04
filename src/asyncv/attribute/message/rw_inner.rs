use rumqttc::QoS;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::Mutex;

use crate::asyncv::MessageDispatcher;
use crate::AttributeError;
use crate::MessagePayloadManager;

use super::MessageAttributeRoInner;
use super::MessageClient;
use super::OnMessageHandler;

use bytes::Bytes;

use async_trait::async_trait;

use crate::asyncv::AttributeBuilder;

use tokio::sync::Notify;

/// Read Only Inner implementation of the message attribute
/// This inner implementation allow the public part to be cloneable easly
pub struct MessageAttributeRwInner<TYPE: MessagePayloadManager> {
    /// Rw is based on Ro
    pub base: MessageAttributeRoInner<TYPE>,

    /// The topic for commands
    topic_cmd: String,

    /// Requested value of the attribute (set by the user)
    requested_value: Option<TYPE>,
}

impl<TYPE: MessagePayloadManager> MessageAttributeRwInner<TYPE> {
    /// Initialize the attribute
    pub async fn init(
        &self,
        attribute: Arc<Mutex<dyn OnMessageHandler>>,
    ) -> Result<(), AttributeError> {
        self.base.init(attribute).await
    }

    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub fn get(&self) -> Option<TYPE> {
        return self.base.get();
    }

    /// Set the value of the attribute
    ///
    pub async fn set(&mut self, new_value: TYPE) -> Result<(), AttributeError> {
        // // Do not go further if the value is already set
        // if let Some(current_value) = self.value {
        //     if current_value == new_value {
        //         return Ok(());
        //     }
        // }

        // // Set the requested value and publish the request
        // self.requested_value = Some(new_value);
        // match self.requested_value {
        //     Some(requested_value) => {
        //         self.publish(requested_value.into()).await;
        //         Ok(())
        //     }
        //     None => Err(AttributeError::Unkonwn),
        // }

        Ok(())
    }

    /// Publish a command
    ///
    pub async fn publish<V>(&self, value: V) -> Result<(), AttributeError>
    where
        V: Into<Vec<u8>>,
    {
        self.base
            .message_client
            .publish(&self.topic_cmd, QoS::AtMostOnce, true, value)
            .await
            .map_err(|e| AttributeError::Message(e))
    }
}

/// Allow creation from the builder
impl<TYPE: MessagePayloadManager> From<AttributeBuilder> for MessageAttributeRwInner<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        let topic_cmd = format!("{}/cmd", builder.topic.as_ref().unwrap());
        MessageAttributeRwInner {
            base: MessageAttributeRoInner::from(builder),
            topic_cmd: topic_cmd,
            requested_value: None,
        }
    }
}

/// Allow mutation into Arc pointer
impl<TYPE: MessagePayloadManager> Into<Arc<Mutex<MessageAttributeRwInner<TYPE>>>>
    for MessageAttributeRwInner<TYPE>
{
    fn into(self) -> Arc<Mutex<MessageAttributeRwInner<TYPE>>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl<TYPE: MessagePayloadManager> OnMessageHandler for MessageAttributeRwInner<TYPE> {
    async fn on_message(&mut self, data: &Bytes) {
        let new_value = TYPE::from(data.to_vec());
        self.base.value = Some(new_value);
        self.base.change_notifier.notify_waiters();
    }
}
