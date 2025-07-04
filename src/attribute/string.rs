use super::CallbackId;
use crate::fbs::StringBuffer;
use crate::AttributeMetadata;
use crate::GenericAttribute;
use zenoh::Session;

#[derive(Clone, Debug)]
/// Object to manage the StringAttribute
///
pub struct StringAttribute {
    pub inner: GenericAttribute<StringBuffer>,
}

impl StringAttribute {
    /// Create a new instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        let inner = GenericAttribute::<StringBuffer>::new(session, metadata).await;
        Self { inner }
    }

    /// Send command and do not wait for validation
    ///
    #[inline]
    pub async fn shoot(&mut self, value: String) {
        self.inner.shoot(value).await;
    }

    /// Send command and do not wait for validation (string slice variant)
    ///
    #[inline]
    pub async fn shoot_str(&mut self, value: &str) {
        self.inner.shoot(value.to_string()).await;
    }

    /// Set the value and wait for validation
    ///
    #[inline]
    pub async fn set(&mut self, value: String) -> Result<(), String> {
        self.inner.set(value).await
    }

    /// Set the value and wait for validation (string slice variant)
    ///
    #[inline]
    pub async fn set_str(&mut self, value: &str) -> Result<(), String> {
        self.inner.set(value.to_string()).await
    }

    /// Get the last received value
    ///
    #[inline]
    pub async fn get(&self) -> Option<StringBuffer> {
        self.inner.get().await
    }

    /// Wait for a specific string value to be received
    ///
    #[inline]
    pub async fn wait_for_value(
        &self,
        value: String,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String> {
        self.inner
            .wait_for_value(move |buf: &StringBuffer| buf.value() == value, timeout)
            .await
            .map(|_| ())
    }

    /// Wait for a specific string value to be received (string slice variant)
    ///
    #[inline]
    pub async fn wait_for_value_str(
        &self,
        value: &str,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String> {
        let value = value.to_string();
        self.inner
            .wait_for_value(move |buf: &StringBuffer| buf.value() == value, timeout)
            .await
            .map(|_| ())
    }

    /// Wait for a string value that contains a substring to be received
    ///
    #[inline]
    pub async fn wait_for_contains(
        &self,
        substring: String,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String> {
        self.inner
            .wait_for_value(
                move |buf: &StringBuffer| buf.value().contains(&substring),
                timeout,
            )
            .await
            .map(|_| ())
    }

    /// Wait for a string value that starts with a prefix to be received
    ///
    #[inline]
    pub async fn wait_for_starts_with(
        &self,
        prefix: String,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String> {
        self.inner
            .wait_for_value(
                move |buf: &StringBuffer| buf.value().starts_with(&prefix),
                timeout,
            )
            .await
            .map(|_| ())
    }

    /// Wait for a string value that ends with a suffix to be received
    ///
    #[inline]
    pub async fn wait_for_ends_with(
        &self,
        suffix: String,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String> {
        self.inner
            .wait_for_value(
                move |buf: &StringBuffer| buf.value().ends_with(&suffix),
                timeout,
            )
            .await
            .map(|_| ())
    }

    /// Add a callback that will be triggered when receiving StringBuffer messages
    /// Optionally, a condition can be provided to filter when the callback is triggered
    #[inline]
    pub async fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(StringBuffer) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
        C: Fn(&StringBuffer) -> bool + Send + Sync + 'static,
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