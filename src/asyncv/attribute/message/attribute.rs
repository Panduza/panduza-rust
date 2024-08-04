mod inner;
use inner::AttributeInner;
use tokio::time::sleep;

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub use super::MessageClient;
use crate::AttributeError;

use super::AttributeBuilder;

pub use super::OnMessageHandler;

// use super::att::Att;
// pub use super::CoreMembers;
// pub use super::OnMessageHandler;
// pub use super::ReactorData;

// pub use inner_msg_att_bool::OnChangeHandlerFunction;

pub trait AttributePayloadManager:
    Into<Vec<u8>> + From<Vec<u8>> + PartialEq + Copy + Sync + Send + 'static
{
}

/// Attribute to manage a boolean
pub struct Attribute<TYPE: AttributePayloadManager> {
    ///
    inner: Arc<Mutex<AttributeInner<TYPE>>>,
}

impl<TYPE: AttributePayloadManager> Attribute<TYPE> {
    ///
    pub fn new(builder: AttributeBuilder) -> Attribute<TYPE> {
        Attribute {
            inner: AttributeInner::new(builder).to_arc_mutex(),
        }
    }

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
