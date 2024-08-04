use tokio::time::sleep;

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use super::MessageAttributeRwInner;
pub use super::MessageClient;
use crate::AttributeError;
use crate::MessagePayloadManager;

use super::AttributeBuilder;

use super::OnMessageHandler;

// use super::att::Att;
// pub use super::CoreMembers;
// pub use super::OnMessageHandler;
// pub use super::ReactorData;

// pub use inner_msg_att_bool::OnChangeHandlerFunction;

/// Attribute to manage a boolean
#[derive(Clone)]
pub struct MessageAttributeRw<TYPE: MessagePayloadManager> {
    ///
    inner: Arc<Mutex<MessageAttributeRwInner<TYPE>>>,
}

impl<TYPE: MessagePayloadManager> MessageAttributeRw<TYPE> {
    /// Initialize the attribute
    ///
    pub async fn init(self) -> Result<Self, AttributeError> {
        self.inner.lock().await.init(self.inner.clone()).await?;
        Ok(self)
    }

    // wait_change()
    // wait_change_then()

    pub fn when_change<F>(&self, function: F)
    where
        F: Future<Output = ()> + Send + 'static,
        // F: Future<Output = Result<bool, ()>> + Send + 'static,
    {
        tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;
            function.await
        });
    }

    // pub async fn on_change_handler(&self, handler: Box<dyn OnChangeHandler>) {
    //     self.inner.lock().await.on_change_handler(handler);
    // }
    // pub async fn on_change(&self, handler: OnChangeHandlerFunction) {
    //     self.inner.lock().await.on_change(handler);
    // }

    /// Set the value of the attribute
    ///
    pub async fn set<I: Into<TYPE>>(&self, value: I) -> Result<(), AttributeError> {
        self.inner.lock().await.set(value.into()).await?;
        // let cv = self.inner.lock().await.set_ensure_lock_clone();
        // cv.with_lock(|mut done| {
        //     while !*done {
        //         done.wait();
        //     }
        // });
        Ok(())
    }

    /// Get the value of the attribute
    ///
    pub async fn get(&self) -> Option<TYPE> {
        self.inner.lock().await.get()
    }
}

/// Allow creation from the builder
impl<TYPE: MessagePayloadManager> From<AttributeBuilder> for MessageAttributeRw<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        MessageAttributeRw {
            inner: MessageAttributeRwInner::from(builder).into(),
        }
    }
}
