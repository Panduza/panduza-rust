use super::std_obj::StdObjAttribute;
use super::CallbackId;
use crate::fbs::BooleanBuffer;
use crate::AttributeMetadata;
use zenoh::Session;

#[derive(Clone, Debug)]
/// Object to manage the BooleanAttribute
///
pub struct BooleanAttribute {
    pub inner: StdObjAttribute<BooleanBuffer>,
}

impl BooleanAttribute {
    /// Create a new instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        Self {
            inner: StdObjAttribute::<BooleanBuffer>::new(session, metadata).await,
        }
    }

    /// Send command and do not wait for validation
    ///
    #[inline]
    pub async fn shoot(&mut self, value: bool) {
        self.inner
            .shoot(
                BooleanBuffer::builder()
                    .with_value(value)
                    .with_source(0)
                    .with_random_sequence()
                    .build()
                    .expect("Failed to build BooleanBuffer"),
            )
            .await;
    }

    ///
    ///
    #[inline]
    pub async fn set(&mut self, value: bool) -> Result<(), String> {
        self.inner
            .set(
                BooleanBuffer::builder()
                    .with_value(value)
                    .with_source(0)
                    .with_random_sequence()
                    .build()
                    .expect("Failed to build BooleanBuffer"),
            )
            .await
    }

    // ------------------------------------------------------------------------

    #[inline]
    pub async fn wait_for_value(&self, value: bool, timeout: Option<std::time::Duration>) {
        self.inner
            .wait_for_value(
                move |buf: &BooleanBuffer| buf.value() == Some(value),
                timeout,
            )
            .await
            .ok();
    }

    // ------------------------------------------------------------------------

    /// Get the last received value
    ///
    #[inline]
    pub async fn get(&self) -> Option<BooleanBuffer> {
        self.inner.get().await
    }

    /// Add a callback that will be triggered when receiving BooleanBuffer messages
    /// Optionally, a condition can be provided to filter when the callback is triggered
    #[inline]
    pub async fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(BooleanBuffer) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
        C: Fn(&BooleanBuffer) -> bool + Send + Sync + 'static,
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
