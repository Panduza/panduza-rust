use crate::pubsub::Publisher;
use crate::reactor::DataReceiver;
use crate::Topic;
use bytes::Bytes;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::Notify;
use tokio::time::timeout;
use serde_json::Value as JsonValue;

use super::data_pack::AttributeDataPack;


#[derive(Clone, Debug)]
/// Object to manage the JsonAttribute
///
pub struct JsonAttribute {
        ///
    /// TODO: maybe add this into the data pack
    topic: String,

    
    /// Object that all the attribute to publish
    ///
    cmd_publisher: Publisher,

    /// Initial data
    ///
    pack: Arc<Mutex<AttributeDataPack<JsonValue>>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl JsonAttribute {
    /// Create a new instance
    ///
    pub async fn new(topic: String,cmd_publisher: Publisher, mut att_receiver: DataReceiver) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(
            AttributeDataPack::<JsonValue>::default()
        ));

        //
        //
        let update_1 = pack.lock().unwrap().update_notifier();

        //
        // Create the recv task
        let pack_2 = pack.clone();
        tokio::spawn(async move {
            loop {
                //
                let message = att_receiver.recv().await;

                println!("new message {:?}", message);

                // Manage message
                if let Some(message) = message {
                    // Deserialize
                    let value = serde_json::from_slice(&message).unwrap();
                    // Push into pack
                    pack_2.lock().unwrap().push(value);
                }
                // None => no more message
                else {
                    break;
                }
            }
        });

        // Wait for the first message
        // Need a timeout here
        update_1.notified().await;

        //
        // Return attribute
        Self {
            topic: topic,
            cmd_publisher: cmd_publisher,
            pack: pack,
            update_notifier: update_1,
        }
    }


    /// Send command and do not wait for validation
    ///
    pub async fn shoot(&mut self, value: bool) {
        // Wrap value into payload
        let pyl = Bytes::from(serde_json::to_string(&value).unwrap());

        // Send the command
        self.cmd_publisher.publish(pyl).await.unwrap();
    }

    /// Notify when new data have been received
    ///
    pub fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }

    ///
    ///
    pub async fn set(&mut self, value: bool) -> Result<(), String> {
        //
        self.shoot(value).await;

        let delay = Duration::from_secs(5);

        // Wait for change in the data pack
        timeout(delay, self.update_notifier.notified())
            .await
            .map_err(|e| e.to_string())?;

        while value != self.get().unwrap() {
            // append 3 retry before failling if update received but not good
            timeout(delay, self.update_notifier.notified())
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    ///
    ///
    pub fn get(&self) -> Option<JsonValue> {
        self.pack.lock().unwrap().last()
    }

    ///
    ///
    pub fn pop(&self) -> Option<JsonValue> {
        self.pack.lock().unwrap().pop()
    }

    pub fn get_instance_status_topic(&self) -> String {
        format!("pza/_/devices/{}", Topic::from_string(self.topic.clone(), true).instance_name())
    }
}
