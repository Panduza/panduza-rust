use crate::fbs::notification_v0::NotificationBuffer;
use crate::fbs::notification_v0::NotificationType;
// use crate::pubsub::Publisher;
use crate::reactor::DataReceiver;
use crate::AttributeMode;
use bytes::Bytes;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::Notify;
use zenoh::handlers::FifoChannelHandler;
use zenoh::pubsub::Subscriber;
use zenoh::sample::Sample;
use zenoh::Session;

use super::data_pack::AttributeDataPack;

pub struct NotificationPack {
    pub notifications: Vec<NotificationBuffer>,
}

impl NotificationPack {
    /// Create a new instance
    ///
    pub fn new(notifications: Vec<NotificationBuffer>) -> Self {
        Self { notifications }
    }

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    pub fn has_alert(&self) -> bool {
        for notification in &self.notifications {
            if notification.object().type_() == NotificationType::Alert as u16 {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Debug)]
/// Object to manage the NotificationAttribute
///
pub struct NotificationAttribute {
    ///
    /// TODO: maybe add this into the data pack
    topic: String,

    mode: AttributeMode,

    /// Global Session
    ///
    session: Session,

    /// Initial data
    ///
    pack: Arc<Mutex<AttributeDataPack<NotificationBuffer>>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl NotificationAttribute {
    /// Create a new instance
    ///
    pub async fn new(
        session: Session,
        topic: String,
        mode: AttributeMode,
        mut att_receiver: Subscriber<FifoChannelHandler<Sample>>,
    ) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(
            AttributeDataPack::<NotificationBuffer>::default(),
        ));

        //
        //
        let update_1 = pack.lock().unwrap().update_notifier();

        //
        // Create the recv task
        let pack_2 = pack.clone();

        tokio::spawn(async move {
            while let Ok(sample) = att_receiver.recv_async().await {
                let value = NotificationBuffer::from_raw_data(Bytes::copy_from_slice(
                    &sample.payload().to_bytes(),
                ));
                // Push into pack
                pack_2.lock().unwrap().push(value);
            }
        });

        //
        // Return attribute
        Self {
            topic: topic,
            session: session,
            pack: pack,
            update_notifier: update_1,
            mode: mode,
        }
    }

    /// Notify when new data have been received
    ///
    pub fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }

    ///
    ///
    pub fn get(&self) -> Option<NotificationBuffer> {
        self.pack.lock().unwrap().last()
    }

    ///
    ///
    pub fn pop(&self) -> Option<NotificationBuffer> {
        self.pack.lock().unwrap().pop()
    }

    ///
    ///
    pub fn pop_all(&self) -> NotificationPack {
        let mut pack = self.pack.lock().unwrap();
        let mut notifications = Vec::new();
        while let Some(value) = pack.pop() {
            notifications.push(value);
        }
        NotificationPack::new(notifications)
    }
}
