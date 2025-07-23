use super::std_obj::StdObjAttribute;
use super::CallbackId;
use crate::fbs::StringBuffer;
use crate::AttributeMetadata;
use zenoh::Session;

#[derive(Clone, Debug)]
/// Object to manage the StringAttribute
///
pub struct StringAttribute {
    pub inner: StdObjAttribute<StringBuffer>,
}

impl StringAttribute {
    /// Create a new instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        let inner = StdObjAttribute::<StringBuffer>::new(session, metadata).await;
        Self { inner }
    }

    /// Set the value and wait for validation
    ///
    #[inline]
    pub async fn set(&mut self, value: String) -> Result<(), String> {
        self.inner
            .set(
                StringBuffer::builder()
                    .with_random_sequence()
                    .with_source(0)
                    .with_value(value)
                    .build()
                    .expect("Failed to build StringBuffer"),
            )
            .await
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
            .wait_for_value(
                move |buf: &StringBuffer| buf.value() == Some(value.as_str()),
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
