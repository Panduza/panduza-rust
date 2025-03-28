use crate::{pubsub::Publisher, reactor::DataReceiver};
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

#[derive(Debug)]
struct BytesDataPack {
    /// Last value received
    ///
    last: Option<Bytes>,

    /// Queue of value (need to be poped)
    ///
    queue: Vec<Bytes>,

    /// If True we are going to use the input queue
    ///
    pub use_input_queue: bool,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl BytesDataPack {
    ///
    ///
    pub fn push(&mut self, bytes: Bytes) {
        if self.use_input_queue {
            self.queue.push(bytes.clone());
        }
        self.last = Some(bytes);
        self.update_notifier.notify_waiters();
    }

    ///
    ///
    pub fn last(&self) -> Option<Bytes> {
        self.last.clone()
    }

    ///
    ///
    pub fn pop(&mut self) -> Option<Bytes> {
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

impl Default for BytesDataPack {
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
/// Object to manage the StringAttribute
///
pub struct BytesAttribute {
    /// Object that all the attribute to publish
    ///
    cmd_publisher: Publisher,

    /// Initial data
    ///
    pack: Arc<Mutex<BytesDataPack>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl BytesAttribute {
    /// Create a new instance
    ///
    pub fn new(cmd_publisher: Publisher, mut att_receiver: DataReceiver) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(BytesDataPack::default()));

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
                    // Push into pack
                    pack_2.lock().unwrap().push(message);
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
    pub async fn shoot(&mut self, value: Bytes) {
        // Send the command
        self.cmd_publisher.publish(value).await.unwrap();
    }

    /// Notify when new data have been received
    ///
    pub fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }

    ///
    ///
    pub async fn set(&mut self, value: Bytes) {
        //
        self.shoot(value).await;

        // Wait for change in the data pack
        self.update_notifier.notified().await;

        // get last // if same as value ok else wait or error ?
    }

    ///
    ///
    pub fn get(&self) -> Option<Bytes> {
        self.pack.lock().unwrap().last()
    }

    ///
    ///
    pub fn pop(&self) -> Option<Bytes> {
        self.pack.lock().unwrap().pop()
    }
}
