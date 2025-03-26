// use crate::{pubsub::Publisher, reactor::DataReceiver};
use crate::AttributeMode;
use bytes::Bytes;
use std::{
    char::ToUppercase,
    sync::{Arc, Mutex},
};
use tokio::sync::Notify;
use zenoh::handlers::FifoChannelHandler;
use zenoh::pubsub::Subscriber;
use zenoh::sample::Sample;
use zenoh::{pubsub::Publisher, Session};

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

#[derive(Debug)]
/// Object to manage the BytesAttribute
///
pub struct NumberAttribute {
    /// Object that all the attribute to publish
    ///
    session: Session,

    /// Initial data
    ///
    pack: Arc<Mutex<NumberDataPack>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,

    topic: String,

    mode: AttributeMode,
}

impl NumberAttribute {
    /// Create a new instance
    ///
    pub async fn new(
        session: Session,
        mode: AttributeMode,
        mut att_receiver: Subscriber<FifoChannelHandler<Sample>>,
        topic: String,
    ) -> Self {
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
            while let Ok(sample) = att_receiver.recv_async().await {
                let value: u32 = sample.payload().try_to_string().unwrap().parse().unwrap();
                // Push into pack
                pack_2.lock().unwrap().push(value);
            }

            // loop {
            //     //
            //     let message = att_receiver.recv().unwrap();
            //     let value: u32 = message.payload().try_to_string().unwrap().parse().unwrap();
            //     // Push into pack
            //     pack_2.lock().unwrap().push(value);

            //     // println!("new message {:?}", message);

            //     // // Manage message
            //     // if let Some(message) = message {
            //     //     // Deserialize
            //     //     let value: u32 = serde_json::from_slice(&message).unwrap();
            //     //     // Push into pack
            //     //     pack_2.lock().unwrap().push(value);
            //     // }
            //     // // None => no more message
            //     // else {
            //     //     break;
            //     // }
            // }
        });

        //
        // Return attribute
        Self {
            session,
            pack: pack,
            update_notifier: update_1,
            topic,
            mode,
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
        self.session.put(self.topic.clone(), pyl).await.unwrap();
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
