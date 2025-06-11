// use crate::pubsub::Publisher;
use super::CallbackId;
use crate::fbs::BooleanBuffer;
use crate::AttributeMetadata;
use crate::GenericAttribute;
use zenoh::Session;

#[derive(Clone, Debug)]
/// Object to manage the BooleanAttribute
///
pub struct BooleanAttribute {
    inner: GenericAttribute<BooleanBuffer>,
}

impl BooleanAttribute {
    /// Create a new instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        let inner = GenericAttribute::<BooleanBuffer>::new(session, metadata).await;
        Self { inner }
    }

    /// Send command and do not wait for validation
    ///
    #[inline]
    pub async fn shoot(&mut self, value: bool) {
        self.inner.shoot(value).await;
    }

    ///
    ///
    #[inline]
    pub async fn set(&mut self, value: bool) -> Result<(), String> {
        self.inner.set(value).await
    }

    /// Get the last received value
    ///
    #[inline]
    pub fn get(&self) -> Option<BooleanBuffer> {
        self.inner.get()
    }

    /// Get the last received value as a boolean
    ///
    #[inline]
    pub fn get_value(&self) -> Option<bool> {
        self.get().map(|buffer| buffer.value())
    }

    /// Wait for a specific boolean value to be received
    ///
    #[inline]
    pub async fn wait_for_value(
        &self,
        value: bool,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String> {
        let expected_buffer = BooleanBuffer::from(value);
        self.inner.wait_for_buffer(expected_buffer, timeout).await
    }

    /// Add a callback that will be triggered when receiving BooleanBuffer messages
    /// Optionally, a condition can be provided to filter when the callback is triggered
    #[inline]
    pub fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(&BooleanBuffer) + Send + Sync + 'static,
        C: Fn(&BooleanBuffer) -> bool + Send + Sync + 'static,
    {
        self.inner.add_callback(callback, condition)
    }

    /// Remove a callback by its ID
    ///
    #[inline]
    pub fn remove_callback(&self, callback_id: CallbackId) -> bool {
        self.inner.remove_callback(callback_id)
    }

    /// Clear all callbacks
    ///
    #[inline]
    pub fn clear_callbacks(&self) {
        self.inner.clear_callbacks()
    }

    /// Get the number of registered callbacks
    ///
    #[inline]
    pub fn callback_count(&self) -> usize {
        self.inner.callback_count()
    }

    /// Get attribute metadata
    ///
    #[inline]
    pub fn metadata(&self) -> &AttributeMetadata {
        self.inner.metadata()
    }

    /// Get the command topic
    ///
    #[inline]
    pub fn cmd_topic(&self) -> &str {
        self.inner.cmd_topic()
    }
}
