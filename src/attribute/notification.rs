use super::CallbackId;
use crate::fbs::NotificationBuffer;
use crate::fbs::NotificationType;
use crate::AttributeMetadata;
use crate::GenericAttribute;
use zenoh::Session;

#[derive(Clone, Debug, Default)]
/// A pack of notifications, used to group multiple NotificationBuffer objects together.
pub struct NotificationPack {
    pub notifications: Vec<NotificationBuffer>,
}

impl NotificationPack {
    /// Create a new instance
    ///
    pub fn new(notifications: Vec<NotificationBuffer>) -> Self {
        Self { notifications }
    }

    /// Push a new notification into the pack
    ///
    pub fn push(&mut self, notification: NotificationBuffer) {
        self.notifications.push(notification);
    }

    /// Reset the pack, clearing all notifications
    ///
    pub fn reset(&mut self) {
        self.notifications.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    pub fn has_alert(&self) -> bool {
        for notification in &self.notifications {
            if notification.notification_type().unwrap() == NotificationType::Alert {
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
    pub inner: GenericAttribute<NotificationBuffer>,
}

impl NotificationAttribute {
    /// Create a new instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        let inner = GenericAttribute::<NotificationBuffer>::new(session, metadata).await;
        Self { inner }
    }

    /// Send command and do not wait for validation
    ///
    #[inline]
    pub async fn shoot(&mut self, value: NotificationBuffer) {
        self.inner.shoot(value).await;
    }

    ///
    ///
    #[inline]
    pub async fn set(&mut self, value: NotificationBuffer) -> Result<(), String> {
        self.inner.set(value).await
    }

    /// Get the last received value
    ///
    #[inline]
    pub async fn get(&self) -> Option<NotificationBuffer> {
        self.inner.get().await
    }

    /// Wait for a specific notification value to be received
    ///
    #[inline]
    pub async fn wait_for_value<F>(
        &self,
        condition: F,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String>
    where
        F: Fn(&NotificationBuffer) -> bool + Send + Sync + 'static,
    {
        self.inner
            .wait_for_value(condition, timeout)
            .await
            .map(|_| ())
    }

    /// Add a callback that will be triggered when receiving NotificationBuffer messages
    /// Optionally, a condition can be provided to filter when the callback is triggered
    #[inline]
    pub async fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(
                NotificationBuffer,
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
        C: Fn(&NotificationBuffer) -> bool + Send + Sync + 'static,
    {
        self.inner.add_callback(callback, condition).await
    }

    /// Remove a callback by its ID
    ///
    #[inline]
    pub async fn remove_callback(&self, callback_id: CallbackId) -> bool {
        self.inner.remove_callback(callback_id).await
    }

    /// Get attribute metadata
    ///
    #[inline]
    pub fn metadata(&self) -> &AttributeMetadata {
        self.inner.metadata()
    }
}
