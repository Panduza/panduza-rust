use crate::fbs::status_v0::StatusBuffer;
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
struct StatusDataPack {
    /// Last value received
    ///
    last: Option<StatusBuffer>,

    /// Queue of value (need to be poped)
    ///
    queue: Vec<StatusBuffer>,

    /// If True we are going to use the input queue
    ///
    pub use_input_queue: bool,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl StatusDataPack {
    ///
    ///
    pub fn push(&mut self, v: StatusBuffer) {
        if self.use_input_queue {
            self.queue.push(v.clone());
        }
        self.last = Some(v);
        self.update_notifier.notify_waiters();
    }

    ///
    ///
    pub fn last(&self) -> Option<StatusBuffer> {
        self.last.clone()
    }

    ///
    ///
    pub fn pop(&mut self) -> Option<StatusBuffer> {
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

impl Default for StatusDataPack {
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
/// Object to manage the StatusAttribute
///
pub struct StatusAttribute {
    ///
    /// TODO: maybe add this into the data pack
    topic: String,

    /// Initial data
    ///
    pack: Arc<Mutex<StatusDataPack>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl StatusAttribute {
    /// Create a new instance
    ///
    pub async fn new(topic: String, mut att_receiver: DataReceiver) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(StatusDataPack::default()));

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
                        let value = StatusBuffer::from_raw_data(message.clone());
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

        // Need a timeout here
        update_1.notified().await;

        //
        // Return attribute
        Self {
            topic: topic,
            pack: pack,
            update_notifier: update_1,
        }
    }

    /// Enable the input queue buffer (to use pop feature)
    ///
    pub fn enable_input_queue(&mut self, enable: bool) {
        self.pack.lock().unwrap().use_input_queue = enable;
    }

    // /// Send command and do not wait for validation
    // ///
    // pub async fn shoot(&mut self, value: StatusBuffer) {
    //     // Wrap value into payload
    //     let pyl = value.raw_data().clone();

    //     // Send the command
    //     self.cmd_publisher.publish(pyl).await.unwrap();
    // }

    /// Notify when new data have been received
    ///
    pub fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }

    ///
    ///
    pub fn get(&self) -> Option<StatusBuffer> {
        self.pack.lock().unwrap().last()
    }

    ///
    ///
    pub fn pop(&self) -> Option<StatusBuffer> {
        self.pack.lock().unwrap().pop()
    }

    /// Control that all instances are in running state
    ///
    pub fn all_instances_are_running(&self) -> Result<bool, &'static str> {
        // Get the last value
        let value = self.get();

        // Check if we have a value
        if let Some(value) = value {
            return value.all_instances_are_running();
        }

        // Return an error if no buffer is available
        Err("No buffer available")
    }

    ///
    ///
    pub fn at_least_one_instance_is_not_running(
        &self,
    ) -> Result<bool, &'static str> {
        // Get the last value
        let value = self.get();

        // Check if we have a value
        if let Some(value) = value {
            return value.at_least_one_instance_is_not_running();
        }

        // Return an error if no buffer is available
        Err("No buffer available")
    }

    /// Wait for all instances to be in running state
    ///
    pub async fn wait_for_all_instances_to_be_running(
        &self,
        timeout_duration: Duration,
    ) -> Result<(), &'static str> {
        loop {
            // Check if all instances are running
            if let Ok(true) = self.all_instances_are_running() {
                return Ok(());
            }

            // Wait for a notification or timeout
            if timeout(timeout_duration, self.update_notifier.notified())
                .await
                .is_err()
            {
                return Err("Timeout while waiting for all instances to be running");
            }
        }
    }

    /// Wait for at least one instance to be not running
    /// 
    pub async fn wait_for_at_least_one_instance_to_be_not_running(
        &self,
        timeout_duration: Duration,
    ) -> Result<(), &'static str> {
        loop {
            // Check if at least one instance is not running
            if let Ok(true) = self.at_least_one_instance_is_not_running() {
                return Ok(());
            }

            // Wait for a notification or timeout
            if timeout(timeout_duration, self.update_notifier.notified())
                .await
                .is_err()
            {
                return Err("Timeout while waiting for at least one instance to be not running");
            }
        }
    }

    //
    // get_list_of_instance_in_error
    //
}
