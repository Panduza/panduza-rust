use crate::fbs::status_v0::StatusBuffer;
use crate::reactor::DataReceiver;
use bytes::Bytes;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::Notify;
use tokio::time::timeout;
use zenoh::handlers::FifoChannelHandler;
use zenoh::pubsub::Subscriber;
use zenoh::query::Reply;
use zenoh::sample::Sample;
use zenoh::Session;

use super::data_pack::AttributeDataPack;

#[derive(Clone, Debug)]
/// Object to manage the StatusAttribute
///
pub struct StatusAttribute {
    ///
    /// TODO: maybe add this into the data pack
    topic: String,

    /// Global Session
    ///
    session: Session,

    /// Initial data
    ///
    pack: Arc<Mutex<AttributeDataPack<StatusBuffer>>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl StatusAttribute {
    /// Create a new instance
    ///
    pub async fn new(
        session: Session,
        topic: String,
        mut att_receiver: Subscriber<FifoChannelHandler<Sample>>,
    ) -> Self {
        // Create data pack
        let pack = Arc::new(Mutex::new(AttributeDataPack::<StatusBuffer>::default()));
        let update_1 = pack.lock().unwrap().update_notifier();

        // Create the recv task
        let pack_2 = pack.clone();

        tokio::spawn(async move {
            while let Ok(sample) = att_receiver.recv_async().await {
                let value = StatusBuffer::from_raw_data(Bytes::copy_from_slice(
                    &sample.payload().to_bytes(),
                ));
                // Push into pack
                pack_2.lock().unwrap().push(value);
            }
        });

        let query = session.get(topic.clone()).await.unwrap();
        let result = query.recv_async().await.unwrap();
        let value: StatusBuffer = StatusBuffer::from_raw_data(Bytes::copy_from_slice(
            &result.result().unwrap().payload().to_bytes(),
        ));
        pack.lock().unwrap().push(value);

        // Return attribute
        Self {
            topic: topic,
            session: session,
            pack: pack,
            update_notifier: update_1,
        }
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
            let result = value.all_instances_are_running();
            return result;
        }

        // Return an error if no buffer is available
        Err("No buffer available")
    }

    ///
    ///
    pub fn at_least_one_instance_is_not_running(&self) -> Result<bool, &'static str> {
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
            match self.all_instances_are_running() {
                Ok(flag) => {
                    if flag {
                        return Ok(());
                    }
                }
                Err(e) => println!("Error checking instances status: {}", e),
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
