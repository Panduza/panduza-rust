use crate::{pubsub::Publisher, reactor::DataReceiver};
use bytes::Bytes;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::Notify, time::timeout};
use serde_json::Value as JsonValue;

#[derive(Debug)]
struct JsonAttributePack {
    /// Last value received
    ///
    last: Option<JsonValue>,

    /// Queue of value (need to be poped)
    ///
    queue: Vec<JsonValue>,

    /// If True we are going to use the input queue
    ///
    pub use_input_queue: bool,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl JsonAttributePack {
    ///
    ///
    pub fn push(&mut self, v: JsonValue) {
        if self.use_input_queue {
            self.queue.push(v.clone());
        }
        self.last = Some(v);
        self.update_notifier.notify_waiters();
    }

    ///
    ///
    pub fn last(&self) -> Option<JsonValue> {
        self.last.clone()
    }

    ///
    ///
    pub fn pop(&mut self) -> Option<JsonValue> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    ///
    ///
    pub fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}

impl Default for JsonAttributePack {
    fn default() -> Self {
        Self {
            last: Default::default(),
            queue: Default::default(),
            use_input_queue: false,
            update_notifier: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
/// Object to manage the JsonAttribute
///
pub struct JsonAttribute {
    /// Object that all the attribute to publish
    ///
    cmd_publisher: Publisher,

    /// Initial data
    ///
    pack: Arc<Mutex<JsonAttributePack>>,

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
        let pack = Arc::new(Mutex::new(JsonAttributePack::default()));

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
            cmd_publisher: cmd_publisher,
            pack: pack,
            update_notifier: update_1,
        }
    }

    /// Enable the input queue buffer (to use pop feature)
    ///
    pub fn enable_input_queue(&mut self, enable: bool) {
        self.pack.lock().unwrap().use_input_queue = enable;
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
}
