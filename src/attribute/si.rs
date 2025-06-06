use crate::fbs::number::NumberBuffer;
use crate::pubsub::Publisher;
use crate::reactor::DataReceiver;
use crate::AttributeMode;
use crate::Topic;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::Notify;
use tokio::time::timeout;

use super::data_pack::AttributeDataPack;

#[derive(Clone, Debug)]
/// Object to manage the SiAttribute
///
pub struct SiAttribute {
    ///
    /// TODO: maybe add this into the data pack
    topic: String,

    
    mode : AttributeMode,
    
    /// Object that all the attribute to publish
    ///
    cmd_publisher: Publisher,

    /// Initial data
    ///
    pack: Arc<Mutex<AttributeDataPack<NumberBuffer>>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl SiAttribute {
    /// Create a new instance
    ///
    pub async fn new(topic: String, mode:AttributeMode, cmd_publisher: Publisher, mut att_receiver: DataReceiver) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(
            AttributeDataPack::<NumberBuffer>::default()
        ));

        //
        //
        let update_1 = pack.lock().unwrap().update_notifier();

        //
        // Create the recv task
        let pack_2 = pack.clone();
        tokio::spawn({
            let topic = topic.clone();
            async move {
            loop {
                //
                let message = att_receiver.recv().await;

                // println!("new message on topic {:?}: {:?}", &topic, message);

                // Manage message
                if let Some(message) = message {
                    // Deserialize
                    let value = NumberBuffer::from_raw_data(message.clone());
                    // Push into pack
                    pack_2.lock().unwrap().push(value);
                }
                // None => no more message
                else {
                    break;
                }
            }
        }});

        // Wait for the first message if mode is not readonly
        if mode != AttributeMode::WriteOnly {
            // Need a timeout here
            update_1.notified().await;
        }

        //
        // Return attribute
        Self {
            topic: topic,
            cmd_publisher: cmd_publisher,
            pack: pack,
            update_notifier: update_1,
            mode: mode,
        }
    }


    /// Send command and do not wait for validation
    ///
    pub async fn shoot(&mut self, value: NumberBuffer) {
        // Wrap value into payload
        let pyl = value.raw_data();

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
    pub async fn set(&mut self, value: NumberBuffer) -> Result<(), String> {
        //
        self.shoot(value.clone()).await;

        if self.mode == AttributeMode::ReadWrite {

            let delay = Duration::from_secs(5);

            // Wait for change in the data pack
            timeout(delay, self.update_notifier.notified())
                .await
                .map_err(|e| e.to_string())?;

            while let Some(current_value) = self.get() {
                if value.clone() == current_value {
                    break;
                }
                // append 3 retry before failing if update received but not good
                timeout(delay, self.update_notifier.notified())
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    ///
    ///
    pub fn get(&self) -> Option<NumberBuffer> {
        self.pack.lock().unwrap().last()
    }

    ///
    ///
    pub fn pop(&self) -> Option<NumberBuffer> {
        self.pack.lock().unwrap().pop()
    }

    pub fn get_instance_status_topic(&self) -> String {
        format!("pza/_/devices/{}", Topic::from_string(self.topic.clone(), true).instance_name())
    }

}
