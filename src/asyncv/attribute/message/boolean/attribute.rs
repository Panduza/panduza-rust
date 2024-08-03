mod inner;
use inner::InnerBoolean;

use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::asyncv::attribute::message::OnBooleanMessage;
use crate::AttributeError;
use async_trait::async_trait;

pub use super::BuilderBoolean;
use super::MessageCoreMembers;

pub use super::OnMessageHandler;

// use super::att::Att;
// pub use super::CoreMembers;
// pub use super::OnMessageHandler;
// pub use super::ReactorData;

// pub use inner_msg_att_bool::OnChangeHandlerFunction;

/// Attribute to manage a boolean
pub struct AttributeBoolean {
    ///
    inner: Arc<Mutex<InnerBoolean>>,
}

impl AttributeBoolean {
    ///
    pub fn new(builder: BuilderBoolean) -> AttributeBoolean {
        AttributeBoolean {
            inner: InnerBoolean::new(builder).to_arc_mutex(),
        }
    }

    pub fn with_boolean_message_handler(handler: Arc<Mutex<dyn OnBooleanMessage>>) {
        // let Box<dyn
    }

    // pub async fn on_change_handler(&self, handler: Box<dyn OnChangeHandler>) {
    //     self.inner.lock().await.on_change_handler(handler);
    // }
    // pub async fn on_change(&self, handler: OnChangeHandlerFunction) {
    //     self.inner.lock().await.on_change(handler);
    // }

    /// Set the value of the attribute
    ///
    pub async fn set(&self, value: bool) -> Result<(), AttributeError> {
        self.inner.lock().await.set(value).await?;
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
    pub async fn get(&self) -> Option<bool> {
        self.inner.lock().await.get()
    }
}
