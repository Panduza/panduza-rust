use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::AttributeError;
use crate::MessagePayloadManager;

use super::AttributeBuilder;
use super::MessageAttributeRoInner;

/// Attribute to manage a boolean
#[derive(Clone)]
pub struct MessageAttributeRo<TYPE: MessagePayloadManager> {
    /// Inner implementation
    inner: Arc<Mutex<MessageAttributeRoInner<TYPE>>>,
}

impl<TYPE: MessagePayloadManager> MessageAttributeRo<TYPE> {
    /// Initialize the attribute
    pub async fn init(self) -> Result<Self, AttributeError> {
        self.inner.lock().await.init(self.inner.clone()).await?;
        Ok(self)
    }

    pub async fn wait_change(&self) {
        let change_notifier = self.inner.lock().await.clone_change_notifier();
        change_notifier.notified().await
    }

    pub async fn wait_change_then<F>(&self, function: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let change_notifier = self.inner.lock().await.clone_change_notifier();
        change_notifier.notified().await;
        function.await;
    }

    /// Get the value of the attribute
    ///
    pub async fn get(&self) -> Option<TYPE> {
        self.inner.lock().await.get()
    }
}

/// Allow creation from the builder
impl<TYPE: MessagePayloadManager> From<AttributeBuilder> for MessageAttributeRo<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        MessageAttributeRo {
            inner: MessageAttributeRoInner::from(builder).into(),
        }
    }
}
