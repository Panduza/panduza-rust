mod inner_msg_att_bool;
use inner_msg_att_bool::InnerAttBool;

use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::AttributeError;

use super::att::Att;
pub use super::CoreMembers;
pub use super::OnMessageHandler;
pub use super::ReactorData;

// pub trait OnChangeHandler: Send + Sync {
//     fn on_change(&self, new_value: bool);
// }

pub use inner_msg_att_bool::OnChangeHandlerFunction;

pub struct AttBool {
    inner: Arc<Mutex<InnerAttBool>>,
}

impl AttBool {
    // pub async fn set_on_change_handler(&self, handler: Box<dyn OnChangeHandler>) {
    //     self.inner.lock().await.set_on_change_handler(handler);
    // }
    pub async fn on_change(&self, handler: OnChangeHandlerFunction) {
        self.inner.lock().await.on_change(handler);
    }

    /// Set the value of the attribute
    ///
    pub async fn set(&self, value: bool) -> Result<(), AttributeError> {
        self.inner.lock().await.set(value).await?;
        let cv = self.inner.lock().await.set_ensure_lock_clone();
        cv.with_lock(|mut done| {
            while !*done {
                done.wait();
            }
        });
        Ok(())
    }

    /// Get the value of the attribute
    ///
    pub async fn get(&self) -> Option<bool> {
        self.inner.lock().await.get()
    }

    pub async fn from_core_members(core_data: CoreMembers) -> Self {
        let inner = InnerAttBool::from(core_data).to_arc_mutex();
        inner.lock().await.register(inner.clone()).await.unwrap();
        Self { inner: inner }
    }
}

// impl Into<AttBool> for Att {
//     fn into(self) -> AttBool {
//         match self.take_core_members() {
//             Ok(core_data) => AttBool::from(core_data),
//             Err(_) => panic!("Error"),
//         }
//     }
// }
