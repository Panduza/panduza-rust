mod inner;
use inner::AttributeInner;

use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::asyncv::attribute::message::OnBooleanMessage;
use crate::AttributeError;
use async_trait::async_trait;

use super::{AttributeBuilder, MessageCoreMembers};

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

    // pub async fn on_change_handler(&self, handler: Box<dyn OnChangeHandler>) {
    //     self.inner.lock().await.on_change_handler(handler);
    // }
    // pub async fn on_change(&self, handler: OnChangeHandlerFunction) {
    //     self.inner.lock().await.on_change(handler);
    // }

    /// Set the value of the attribute
    ///
    pub async fn set<INTO_TYPE: Into<TYPE>>(&self, value: INTO_TYPE) -> Result<(), AttributeError> {
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
