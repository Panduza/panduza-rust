// use crate::pubsub::Publisher;
use crate::reactor::DataReceiver;
use crate::AttributeMode;
use crate::Topic;
use bytes::Bytes;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::Notify;
use tokio::time::timeout;
use zenoh::handlers::FifoChannelHandler;
use zenoh::pubsub::Subscriber;
use zenoh::sample::Sample;
use zenoh::Session;

use super::data_pack::AttributeDataPack;

#[derive(Clone, Debug)]
/// Object to manage the StringAttribute
///
pub struct StringAttribute {
    ///
    /// TODO: maybe add this into the data pack
    topic: String,

    mode: AttributeMode,

    /// Global Session
    ///
    session: Session,

    /// Initial data
    ///
    pack: Arc<Mutex<AttributeDataPack<String>>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl StringAttribute {
    /// Create a new instance
    ///
    pub async fn new(
        session: Session,
        topic_cmd: String,
        topic_att: String,
        mode: AttributeMode,
        mut att_receiver: Subscriber<FifoChannelHandler<Sample>>,
    ) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(AttributeDataPack::<String>::default()));

        //
        //
        let update_1 = pack.lock().unwrap().update_notifier();

        //
        // Create the recv task
        let pack_2 = pack.clone();
        tokio::spawn(async move {
            while let Ok(sample) = att_receiver.recv_async().await {
                let value: String = sample.payload().try_to_string().unwrap().to_string();
                // Push into pack
                pack_2.lock().unwrap().push(value);
            }
        });

        // Wait for the first message if mode is not readonly
        if mode != AttributeMode::WriteOnly {
            let query = session.get(topic_att.clone()).await.unwrap();
            let result = query.recv_async().await.unwrap();
            let value: String = result
                .result()
                .unwrap()
                .payload()
                .try_to_string()
                .unwrap()
                .to_string();
            pack.lock().unwrap().push(value);
        }

        //
        // Return attribute
        Self {
            topic: topic_cmd,
            session: session,
            pack: pack,
            update_notifier: update_1,
            mode: mode,
        }
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

        if self.mode == AttributeMode::ReadWrite {
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
        }

        Ok(())
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

    pub fn get_instance_status_topic(&self) -> String {
        format!(
            "pza/_/devices/{}",
            Topic::from_string(self.topic.clone(), true).instance_name()
        )
    }
}
