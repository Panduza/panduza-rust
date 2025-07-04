use super::CallbackId;
use crate::fbs::BytesBuffer;
use crate::AttributeMetadata;
use crate::GenericAttribute;
use bytes::Bytes;
use zenoh::Session;

#[derive(Clone, Debug)]
/// Object to manage the BytesAttribute
///
pub struct BytesAttribute {
    pub inner: GenericAttribute<BytesBuffer>,
}

impl BytesAttribute {
    /// Create a new instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        let inner = GenericAttribute::<BytesBuffer>::new(session, metadata).await;
        Self { inner }
    }

    /// Send command and do not wait for validation
    ///
    #[inline]
    pub async fn shoot(&mut self, value: Bytes) {
        self.inner.shoot(value).await;
    }

    ///
    ///
    #[inline]
    pub async fn set(&mut self, value: Bytes) -> Result<(), String> {
        self.inner.set(value).await
    }

    /// Get the last received value
    ///
    #[inline]
    pub async fn get(&self) -> Option<BytesBuffer> {
        self.inner.get().await
    }

    /// Wait for a specific bytes value to be received
    ///
    #[inline]
    pub async fn wait_for_value(
        &self,
        value: Bytes,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String>
    {
        self.inner
            .wait_for_value(
                move |buf: &BytesBuffer| buf.value() == value.as_ref(),
                timeout,
            )
            .await
            .map(|_| ())
    }

    /// Add a callback that will be triggered when receiving BytesBuffer messages
    /// Optionally, a condition can be provided to filter when the callback is triggered
    #[inline]
    pub async fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(BytesBuffer) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
        C: Fn(&BytesBuffer) -> bool + Send + Sync + 'static,
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
