use crate::attribute::CallbackId;
use crate::fbs::NotificationBuffer;
use crate::fbs::NotificationType;
use crate::NotificationAttribute;
use futures::FutureExt;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug)]
/// A pack of notifications, used to group multiple NotificationBuffer objects together.
pub struct NotificationPack {
    /// Unique identifier for the callback associated with this pack
    ///
    cb_id: CallbackId,

    /// The attribute associated with this pack
    ///
    attribute: NotificationAttribute,

    /// Vector of notifications in the pack
    ///
    pub notifications: Arc<Mutex<Vec<NotificationBuffer>>>,
}

impl Drop for NotificationPack {
    fn drop(&mut self) {
        let attribute = self.attribute.clone();
        let cb_id = self.cb_id;
        // Spawn a task to remove the callback asynchronously
        // (assuming NotificationAttribute::remove_callback is async)
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(async move {
                attribute.remove_callback(cb_id).await;
            });
        });
    }
}

impl NotificationPack {
    /// Create a new instance
    ///
    pub async fn new(attribute: NotificationAttribute) -> Self {
        // Initialize an empty vector for notifications
        let notifications = Arc::new(Mutex::new(Vec::new()));

        // Register a callback on the attribute to handle incoming notifications
        let cb_id = attribute
            .add_callback({
                let notifications = notifications.clone();
                move |notification| {
                    let mut notifications_vec = notifications.lock().unwrap();
                    notifications_vec.push(notification.clone());
                    async move {}.boxed()
                }
            })
            .await;

        // Return the new NotificationPack instance
        Self {
            cb_id,
            attribute,
            notifications,
        }
    }

    /// Push a new notification into the pack
    ///
    pub fn push(&mut self, notification: NotificationBuffer) {
        let mut notifications = self.notifications.lock().unwrap();
        notifications.push(notification);
    }

    /// Reset the pack, clearing all notifications
    ///
    pub fn reset(&mut self) {
        let mut notifications = self.notifications.lock().unwrap();
        notifications.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.notifications.lock().unwrap().is_empty()
    }

    pub fn has_alert(&self) -> bool {
        let notifications = self.notifications.lock().unwrap();
        for notification in notifications.iter() {
            if notification.notification_type().unwrap() == NotificationType::Alert {
                return true;
            }
        }
        false
    }
}
