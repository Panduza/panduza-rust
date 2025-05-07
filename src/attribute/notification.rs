use crate::fbs::notification_v0::NotificationBuffer;
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
/// Object to manage the NotificationAttribute
///
pub struct NotificationAttribute {
    ///
    /// TODO: maybe add this into the data pack
    topic: String,

    mode: AttributeMode,

    /// Object that all the attribute to publish
    ///
    cmd_publisher: Publisher,

    /// Initial data
    ///
    pack: Arc<Mutex<AttributeDataPack<NotificationBuffer>>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl NotificationAttribute {
    /// Create a new instance
    ///
    pub async fn new(
        topic: String,
        mode: AttributeMode,
        cmd_publisher: Publisher,
        mut att_receiver: DataReceiver,
    ) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(
            AttributeDataPack::<NotificationBuffer>::default()
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

                    println!("new message on topic {:?}: {:?}", &topic, message);

                    // Manage message
                    if let Some(message) = message {
                        // Deserialize
                        let value = NotificationBuffer::from_raw_data(message.clone());
                        // Push into pack
                        pack_2.lock().unwrap().push(value);
                    }
                    // None => no more message
                    else {
                        break;
                    }
                }
            }
        });

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
    pub async fn shoot(&mut self, value: NotificationBuffer) {
        // Wrap value into payload
        let pyl = value.raw_data().clone();

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
    pub fn get(&self) -> Option<NotificationBuffer> {
        self.pack.lock().unwrap().last()
    }

    ///
    ///
    pub fn pop(&self) -> Option<NotificationBuffer> {
        self.pack.lock().unwrap().pop()
    }
}
