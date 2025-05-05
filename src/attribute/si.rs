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

#[derive(Debug)]
struct SiDataPack {
    /// Last value received
    ///
    last: Option<NumberBuffer>,

    /// Queue of value (need to be poped)
    ///
    queue: Vec<NumberBuffer>,

    /// If True we are going to use the input queue
    ///
    pub use_input_queue: bool,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl SiDataPack {
    ///
    ///
    pub fn push(&mut self, v: NumberBuffer) {
        if self.use_input_queue {
            self.queue.push(v.clone());
        }
        self.last = Some(v);
        self.update_notifier.notify_waiters();
    }

    ///
    ///
    pub fn last(&self) -> Option<NumberBuffer> {
        self.last.clone()
    }

    ///
    ///
    pub fn pop(&mut self) -> Option<NumberBuffer> {
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

impl Default for SiDataPack {
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
    pack: Arc<Mutex<SiDataPack>>,

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
        let pack = Arc::new(Mutex::new(SiDataPack::default()));

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

    /// Enable the input queue buffer (to use pop feature)
    ///
    pub fn enable_input_queue(&mut self, enable: bool) {
        self.pack.lock().unwrap().use_input_queue = enable;
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
