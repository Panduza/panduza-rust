use super::CallbackId;
use crate::fbs::NumberBuffer;
use crate::AttributeMetadata;
use crate::GenericAttribute;
use zenoh::Session;

#[derive(Clone, Debug)]
/// Object to manage the NumberAttribute
///
pub struct NumberAttribute {
    pub inner: GenericAttribute<NumberBuffer>,
}

impl NumberAttribute {
    /// Create a new instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        let inner = GenericAttribute::<NumberBuffer>::new(session, metadata).await;
        Self { inner }
    }

    /// Send command and do not wait for validation
    ///
    #[inline]
    pub async fn shoot(&mut self, value: f64) {
        self.inner.shoot(value).await;
    }

    ///
    ///
    #[inline]
    pub async fn set(&mut self, value: f64) -> Result<(), String> {
        self.inner.set(value).await
    }

    /// Get the last received value
    ///
    #[inline]
    pub async fn get(&self) -> Option<NumberBuffer> {
        self.inner.get().await
    }

    /// Wait for a specific numeric value to be received
    ///
    #[inline]
    pub async fn wait_for_value(
        &self,
        value: f64,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String> {
        self.inner
            .wait_for_value(
                move |buf: &NumberBuffer| (buf.value() - value).abs() < f64::EPSILON,
                timeout,
            )
            .await
            .map(|_| ())
    }

    /// Wait for a numeric value within a range to be received
    ///
    #[inline]
    pub async fn wait_for_range(
        &self,
        min: f64,
        max: f64,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String> {
        self.inner
            .wait_for_value(
                move |buf: &NumberBuffer| {
                    let val = buf.value();
                    val >= min && val <= max
                },
                timeout,
            )
            .await
            .map(|_| ())
    }

    /// Add a callback that will be triggered when receiving NumberBuffer messages
    /// Optionally, a condition can be provided to filter when the callback is triggered
    #[inline]
    pub async fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(NumberBuffer) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
        C: Fn(&NumberBuffer) -> bool + Send + Sync + 'static,
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
