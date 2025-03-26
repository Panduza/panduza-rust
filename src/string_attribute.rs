use crate::AttributeMode;
use bytes::Bytes;
use rumqttc::tokio_rustls::rustls::internal::msgs::message;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::Notify;
use tokio::time::timeout;
use zenoh::handlers::FifoChannelHandler;
use zenoh::pubsub::Subscriber;
use zenoh::query::Reply;
use zenoh::sample::Sample;
use zenoh::{pubsub::Publisher, session, Session};

#[derive(Debug)]
struct StringDataPack {
    /// Last value received
    ///
    last: Option<String>,

    /// Queue of value (need to be poped)
    ///
    queue: Vec<String>,

    /// If True we are going to use the input queue
    ///
    pub use_input_queue: bool,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl StringDataPack {
    ///
    ///
    pub fn push(&mut self, string: String) {
        if self.use_input_queue {
            self.queue.push(string.clone());
        }
        self.last = Some(string);
        self.update_notifier.notify_waiters();
    }

    ///
    ///
    pub fn last(&self) -> Option<String> {
        self.last.clone()
    }

    ///
    ///
    pub fn pop(&mut self) -> Option<String> {
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

impl Default for StringDataPack {
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
/// Object to manage the StringAttribute
///
pub struct StringAttribute {
    /// Object that all the attribute to publish
    ///
    session: Session,

    /// Initial data
    ///
    pack: Arc<Mutex<StringDataPack>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,

    /// topic
    ///
    topic: String,

    mode: AttributeMode,
}

impl StringAttribute {
    /// Create a new instance
    ///
    pub async fn new(
        session: Session,
        mode: AttributeMode,
        mut att_receiver: Subscriber<FifoChannelHandler<Sample>>,
        // query: FifoChannelHandler<Reply>,
        topic: String,
    ) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(StringDataPack::default()));

        //
        //
        let update_1 = pack.lock().unwrap().update_notifier();

        //
        // Create the recv task
        let pack_2 = pack.clone();

        // while let Ok(sample) = query.recv_async().await {
        //     println!("LE SAMPLE : {:?}", sample.clone());
        //     let message: String =
        //         String::from_utf8_lossy(&sample.result().unwrap().payload().to_bytes()).to_string();
        //     println!("LE MESSAGE : {:?}", message.clone());
        //     pack_2.lock().unwrap().push(message);
        //     println!("PUSH DONE");
        // }

        tokio::spawn(async move {
            while let Ok(sample) = att_receiver.recv_async().await {
                let value = sample.payload().try_to_string().unwrap().to_string();
                // Push into pack
                pack_2.lock().unwrap().push(value);
            }
            // loop {
            //     //
            //     let message = match att_receiver.recv() {
            //         Ok(msg) => msg,
            //         Err(e) => {
            //             eprintln!("Error receiving message: {}", e);
            //             break;
            //         }
            //     };
            //     let value = message.payload().try_to_string().unwrap().to_string();
            //     // Push into pack
            //     pack_2.lock().unwrap().push(value);

            //         // println!("new message {:?}", message);

            //         // // Manage message
            //         // if let Some(message) = message {
            //         //     // Deserialize
            //         //     let value: String = serde_json::from_slice(&message).unwrap();
            //         //     // Push into pack
            //         //     pack_2.lock().unwrap().push(value);
            //         // }
            //         // // None => no more message
            //         // else {
            //         //     break;
            //         // }
            // }
        });

        //
        // Return attribute
        Self {
            session,
            pack: pack,
            update_notifier: update_1,
            topic,
            mode: mode,
        }
    }

    /// Enable the input queue buffer (to use pop feature)
    ///
    pub fn enable_input_queue(&mut self, enable: bool) {
        self.pack.lock().unwrap().use_input_queue = enable;
    }

    /// Send command and do not wait for validation
    ///
    pub async fn shoot(&mut self, value: String) {
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
    pub async fn set(&mut self, value: String) -> Result<(), String> {
        //
        self.shoot(value.clone()).await;

        Ok(())

        // get last // if same as value ok else wait or error ?
    }

    ///
    ///
    pub fn get(&self) -> Option<String> {
        self.pack.lock().unwrap().last()
    }

    ///
    ///
    pub fn pop(&self) -> Option<String> {
        self.pack.lock().unwrap().pop()
    }
}
