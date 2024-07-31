mod inner_msg_att_bool;
use inner_msg_att_bool::InnerAttBool;

use std::sync::{Arc, Mutex};

use crate::AttributeError;

use super::att::Att;
pub use super::CoreMembers;
pub use super::OnMessageHandler;
pub use super::ReactorData;

pub trait OnChangeHandler: Send + Sync {
    fn on_change(&self, new_value: bool);
}

pub struct AttBool {
    inner: Arc<Mutex<InnerAttBool>>,
}

impl AttBool {
    pub fn set_on_change_handler(&self, handler: Box<dyn OnChangeHandler>) {
        self.inner.lock().unwrap().set_on_change_handler(handler);
    }

    /// Set the value of the attribute
    /// 
    pub fn set(&self, value: bool) -> Result<(), AttributeError> {
        self.inner.lock().unwrap().set(value)?;
        let cv = self.inner.lock().unwrap().set_ensure_lock_clone();
        cv.with_lock(|mut done| {
            while !*done {
                done.wait();
            }
        });
        Ok(())
    }

    /// Get the value of the attribute
    ///
    pub fn get(&self) -> Option<bool> {
        self.inner.lock().unwrap().get()
    }
}

impl From<CoreMembers> for AttBool {
    fn from(core_data: CoreMembers) -> Self {
        let inner = InnerAttBool::from(core_data).to_arc_mutex();
        inner.lock().unwrap().register(inner.clone()).unwrap();
        Self { inner: inner }
    }
}

impl Into<AttBool> for Att {
    fn into(self) -> AttBool {
        match self.take_core_members() {
            Ok(core_data) => AttBool::from(core_data),
            Err(_) => panic!("Error"),
        }
    }
}
