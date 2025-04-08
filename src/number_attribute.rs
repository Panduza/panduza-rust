use crate::{pubsub::Publisher, reactor::DataReceiver};
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

#[derive(Debug)]
struct NumberDataPack {
    /// Last value received
    ///
    last: Option<u32>,

    /// Queue of value (need to be poped)
    ///
    queue: Vec<u32>,

    /// If True we are going to use the input queue
    ///
    pub use_input_queue: bool,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl NumberDataPack {
    ///
    ///
    pub fn push(&mut self, number: u32) {
        if self.use_input_queue {
            self.queue.push(number.clone());
        }
        self.last = Some(number);
        self.update_notifier.notify_waiters();
    }

    ///
    ///
    pub fn last(&self) -> Option<u32> {
        self.last.clone()
    }

    ///
    ///
    pub fn pop(&mut self) -> Option<u32> {
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

impl Default for NumberDataPack {
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
/// Object to manage the BytesAttribute
///
pub struct NumberAttribute {
    /// Object that all the attribute to publish
    ///
    cmd_publisher: Publisher,

    /// Initial data
    ///
    pack: Arc<Mutex<NumberDataPack>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl NumberAttribute {
    /// Create a new instance
    ///
    pub fn new(cmd_publisher: Publisher, mut att_receiver: DataReceiver) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(NumberDataPack::default()));

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
                    let value: u32 = serde_json::from_slice(&message).unwrap();
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
    pub async fn shoot(&mut self, value: u32) {
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
    pub async fn set(&mut self, value: u32) {
        //
        self.shoot(value).await;

        // Wait for change in the data pack
        self.update_notifier.notified().await;

        // get last // if same as value ok else wait or error ?
    }

    ///
    ///
    pub fn get(&self) -> Option<u32> {
        self.pack.lock().unwrap().last()
    }

    ///
    ///
    pub fn pop(&self) -> Option<u32> {
        self.pack.lock().unwrap().pop()
    }
}
