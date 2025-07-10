use super::ro_stream::RoStreamAttribute;
use super::CallbackId;
use crate::fbs::NotificationBuffer;
use crate::AttributeMetadata;
use zenoh::Session;

pub mod notification_pack;

#[derive(Clone, Debug)]
/// Object to manage the NotificationAttribute
///
pub struct NotificationAttribute {
    pub inner: RoStreamAttribute<NotificationBuffer>,
}

impl NotificationAttribute {
    /// Create a new instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        let inner = RoStreamAttribute::new(session, metadata).await;
        Self { inner }
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
    pub async fn add_callback<F>(&self, callback: F) -> CallbackId
    where
        F: Fn(
                NotificationBuffer,
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
    {
        self.inner
            .add_callback(callback, None::<fn(&NotificationBuffer) -> bool>)
            .await
    }

    /// Add a callback that will be triggered when receiving NotificationBuffer messages
    /// Optionally, a condition can be provided to filter when the callback is triggered
    #[inline]
    pub async fn add_callback_with_condition<F, C>(
        &self,
        callback: F,
        condition: Option<C>,
    ) -> CallbackId
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
