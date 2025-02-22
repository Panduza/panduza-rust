use crate::{pubsub::Publisher, reactor::DataReceiver};
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

#[derive(Debug)]
struct BooleanDataPack {
    /// Last value received
    ///
    last: Option<bool>,

    /// Queue of value (need to be poped)
    ///
    queue: Vec<bool>,

    /// If True we are going to use the input queue
    ///
    pub use_input_queue: bool,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl BooleanDataPack {
    ///
    ///
    pub fn push(&mut self, v: bool) {
        if self.use_input_queue {
            self.queue.push(v);
        }
        self.last = Some(v);
        self.update_notifier.notify_waiters();
    }

    ///
    ///
    pub fn last(&self) -> Option<bool> {
        self.last
    }

    ///
    ///
    pub fn pop(&mut self) -> Option<bool> {
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

impl Default for BooleanDataPack {
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
/// Object to manage the BooleanAttribute
///
pub struct BooleanAttribute {
    /// Object that all the attribute to publish
    ///
    cmd_publisher: Publisher,

    /// Initial data
    ///
    pack: Arc<Mutex<BooleanDataPack>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl BooleanAttribute {
    /// Create a new instance
    ///
    pub fn new(cmd_publisher: Publisher, mut att_receiver: DataReceiver) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(BooleanDataPack::default()));

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

                // println!("new message {:?}", message);

                // Manage message
                if let Some(message) = message {
                    // Deserialize
                    let value: bool = serde_json::from_slice(&message).unwrap();
                    // Push into pack
                    pack_2.lock().unwrap().push(value);
                }
                // None => no more message
                else {
                    break;
                }
            }
        });

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
    pub async fn set(&mut self, value: bool) {
        //
        self.shoot(value).await;

        // Wait for change in the data pack
        self.update_notifier.notified().await;

        // get last // if same as value ok else wait or error ?
    }

    ///
    ///
    pub fn get(&self) -> Option<bool> {
        self.pack.lock().unwrap().last()
    }

    ///
    ///
    pub fn pop(&self) -> Option<bool> {
        self.pack.lock().unwrap().pop()
    }
}
