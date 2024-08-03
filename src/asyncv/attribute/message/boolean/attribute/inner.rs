use rumqttc::QoS;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::Mutex;

use crate::AttributeError;

use super::MessageClient;
use super::OnMessageHandler;

use std::future::Future;
use std::pin::Pin;

use futures::future::BoxFuture;
use futures::FutureExt;

use bytes::Bytes;

use async_trait::async_trait;

use crate::asyncv::AttributeBuilder;

use super::MessageCoreMembers;
// use super::OnChangeHandler;
use super::OnMessageHandler;

use tokio::sync::Notify;

use super::AttributePayloadManager;

/// Inner implementation of the boolean message attribute
///
pub struct AttributeInner<TYPE: AttributePayloadManager> {
    /// The data of the reactor, to be able to subscribe to the
    /// reactor and route messages to the attribute
    message_dispatcher: Weak<Mutex<MessageDispatcher>>,

    /// The mqtt client
    message_client: MessageClient,

    /// The topic of the attribute
    topic: String,

    /// The topic for commands
    topic_cmd: String,

    /// Current value of the attribute
    value: Option<TYPE>,
    /// Requested value of the attribute (set by the user)
    requested_value: Option<TYPE>,
    // / Handler to call when the value change
}

impl<TYPE: AttributePayloadManager> AttributeInner<TYPE> {
    ///
    pub fn new(builder: AttributeBuilder) -> AttributeInner<TYPE> {
        AttributeInner {
            message_dispatcher: builder.message_dispatcher,
            message_client: builder.message_client,
            topic: builder.topic.as_ref().unwrap().clone(),
            topic_cmd: format!("{}/cmd", builder.topic.as_ref().unwrap()),
            value: None,
            requested_value: None,
        }
    }

    pub fn to_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    // pub fn set_ensure_lock_clone(&mut self) -> Arc<Monitor<bool>> {
    //     return self.set_ensure_lock.clone();
    // }

    // fn set_ensure_update(&mut self) {
    //     if self.set_ensure_ok() {
    //         self.set_ensure_lock.with_lock(|mut done| {
    //             *done = true;
    //             done.notify_one();
    //         });
    //     }
    // }

    /// Initialize the attribute
    ///
    pub async fn init(
        &self,
        attribute: Arc<Mutex<dyn OnMessageHandler>>,
    ) -> Result<(), AttributeError> {
        self.register(attribute).await?;
        self.subscribe().await
    }

    // fn set_ensure_ok(&self) -> bool {
    //     return self.requested_value == self.value;
    // }

    /// Set the value of the attribute
    ///
    pub async fn set(&mut self, new_value: TYPE) -> Result<(), AttributeError> {
        // Do not go further if the value is already set
        if let Some(current_value) = self.value {
            if current_value == new_value {
                return Ok(());
            }
        }

        // Set the requested value and publish the request
        self.requested_value = Some(new_value);
        match self.requested_value {
            Some(requested_value) => {
                self.publish(requested_value.into()).await;
                Ok(())
            }
            None => Err(AttributeError::Unkonwn),
        }
    }

    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub fn get(&self) -> Option<TYPE> {
        return self.value;
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

#[async_trait]
impl<TYPE: AttributePayloadManager> OnMessageHandler for AttributeInner<TYPE> {
    async fn on_message(&mut self, data: &Bytes) {
        println!("boolean");

        // OnChangeHandlerFunction

        // if data.len() == 1 {
        //     match data[0] {
        //         b'1' => {
        //             self.value = Some(true);
        //             // self.set_ensure_update();
        //         }
        //         b'0' => {
        //             self.value = Some(false);
        //             // self.set_ensure_update();
        //         }
        //         _ => {
        //             println!("unexcpedted payload {:?}", data);
        //             return;
        //         }
        //     };
        //     // Do something with the value
        // } else {
        //     println!("wierd payload {:?}", data);
        // }
    }
}
