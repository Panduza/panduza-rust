use super::data_pack::AttributeDataPack;

use crate::AttributeMode;
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use zenoh::handlers::FifoChannelHandler;
use zenoh::pubsub::Subscriber;
use zenoh::sample::Sample;
use zenoh::Session;

#[derive(Debug)]
/// Object to manage the BytesAttribute
///
pub struct NumberAttribute {
    /// Object that all the attribute to publish
    ///
    session: Session,

    /// Initial data
    ///
    pack: Arc<Mutex<AttributeDataPack<u32>>>,

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
        let pack = Arc::new(Mutex::new(AttributeDataPack::<u32>::default()));

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
