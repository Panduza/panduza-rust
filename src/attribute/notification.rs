use crate::fbs::notification_v0::NotificationBuffer;
use crate::fbs::notification_v0::NotificationType;
use crate::pubsub::Publisher;
use crate::reactor::DataReceiver;
use crate::AttributeMode;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::Notify;

use super::data_pack::AttributeDataPack;

/// Type alias pour les callbacks async
pub type NotificationCallback =
    Arc<dyn Fn(NotificationBuffer) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

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

#[derive(Clone)]
/// Object to manage the NotificationAttribute
///
pub struct NotificationAttribute {
    ///
    /// TODO: maybe add this into the data pack
    topic: String,

    mode: AttributeMode,

    /// Object that all the attribute to publish
    ///
    cmd_publisher: Publisher,

    /// Initial data
    ///
    pack: Arc<Mutex<AttributeDataPack<NotificationBuffer>>>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,

    /// Liste des callbacks à appeler lors de la réception de nouveaux messages
    ///
    callbacks: Arc<Mutex<Vec<NotificationCallback>>>,
}

impl NotificationAttribute {
    /// Create a new instance
    ///
    pub async fn new(
        topic: String,
        mode: AttributeMode,
        cmd_publisher: Publisher,
        mut att_receiver: DataReceiver,
    ) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(
            AttributeDataPack::<NotificationBuffer>::default(),
        ));

        //
        // Create callbacks list
        let callbacks = Arc::new(Mutex::new(Vec::<NotificationCallback>::new()));

        //
        //
        let update_1 = pack.lock().unwrap().update_notifier(); //
                                                               // Create the recv task
        let pack_2 = pack.clone();
        let callbacks_2 = callbacks.clone();
        tokio::spawn({
            let topic = topic.clone();
            async move {
                loop {
                    //
                    let message = att_receiver.recv().await;

                    // println!("new message on topic {:?}", message);

                    // Manage message
                    if let Some(message) = message {
                        // Deserialize
                        let value = NotificationBuffer::from_raw_data(message.clone());

                        // Push into pack
                        pack_2.lock().unwrap().push(value.clone());

                        // Call all registered callbacks
                        let callbacks_list = callbacks_2.lock().unwrap().clone();
                        for callback in callbacks_list {
                            let value_clone = value.clone();
                            // tokio::spawn(async move {
                            callback(value_clone).await;
                            // });
                        }
                    }
                    // None => no more message
                    else {
                        break;
                    }
                }
            }
        });

        //
        // Return attribute
        Self {
            topic: topic,
            cmd_publisher: cmd_publisher,
            pack: pack,
            update_notifier: update_1,
            mode: mode,
            callbacks: callbacks,
        }
    }

    /// Notify when new data have been received
    ///
    pub fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }

    /// Ajoute un callback qui sera appelé à chaque nouveau message reçu
    ///
    pub fn add_callback<F, Fut>(&self, callback: F)
    where
        F: Fn(NotificationBuffer) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let wrapped_callback: NotificationCallback =
            Arc::new(move |notification| Box::pin(callback(notification)));

        self.callbacks.lock().unwrap().push(wrapped_callback);
    }

    /// Supprime tous les callbacks enregistrés
    ///
    pub fn clear_callbacks(&self) {
        self.callbacks.lock().unwrap().clear();
    }

    /// Retourne le nombre de callbacks enregistrés
    ///
    pub fn callback_count(&self) -> usize {
        self.callbacks.lock().unwrap().len()
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
