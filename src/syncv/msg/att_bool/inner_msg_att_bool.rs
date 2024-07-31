use std::sync::{Arc, Mutex};

use bytes::Bytes;

use crate::AttributeError;

use super::CoreMembers;
use super::OnChangeHandler;
use super::OnMessageHandler;

use monitor::Monitor;

/// Inner implementation of the boolean message attribute
///
pub struct InnerAttBool {
    /// Members at the core of each attribute
    core: CoreMembers,
    /// Current value of the attribute
    value: Option<bool>,
    /// Requested value of the attribute (set by the user)
    requested_value: Option<bool>,
    /// Handler to call when the value change
    on_change_handler: Option<Box<dyn OnChangeHandler>>,

    set_ensure_lock: Arc<Monitor<bool>>,
}

impl InnerAttBool {
    pub fn to_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    pub fn set_ensure_lock_clone(&mut self) -> Arc<Monitor<bool>> {
        return self.set_ensure_lock.clone();
    }

    fn set_ensure_update(&mut self) {
        if self.set_ensure_ok() {
            self.set_ensure_lock.with_lock(|mut done| {
                *done = true;
                done.notify_one();
            });
        }
    }

    fn set_ensure_ok(&self) -> bool {
        return self.requested_value == self.value;
    }

    /// Set the value of the attribute
    ///
    pub fn set(&mut self, new_value: bool) -> Result<(), AttributeError> {
        // Do not go further if the value is already set
        if let Some(current_value) = self.value {
            if current_value == new_value {
                return Ok(());
            }
        }

        // Set the requested value and publish the request
        self.requested_value = Some(new_value);
        match self.requested_value {
            Some(requested_value) => match requested_value {
                true => self.core.publish("1"),
                false => self.core.publish("0"),
            },
            None => Err(AttributeError::Unkonwn),
        }
    }

    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub fn get(&self) -> Option<bool> {
        return self.value;
    }

    pub fn set_on_change_handler(&mut self, handler: Box<dyn OnChangeHandler>) {
        self.on_change_handler = Some(handler);
    }

    /// Register the attribute to the reactor
    ///
    pub fn register(
        &self,
        attribute: Arc<Mutex<dyn OnMessageHandler>>,
    ) -> Result<(), AttributeError> {
        self.core.register(attribute)
    }
}

impl OnMessageHandler for InnerAttBool {
    fn on_message(&mut self, data: &Bytes) {
        println!("boolean");
        if data.len() == 1 {
            match data[0] {
                b'1' => {
                    self.value = Some(true);
                    self.set_ensure_update();
                }
                b'0' => {
                    self.value = Some(false);
                    self.set_ensure_update();
                }
                _ => {
                    println!("unexcpedted payload {:?}", data);
                    return;
                }
            };
            // Do something with the value
        } else {
            println!("wierd payload {:?}", data);
        }
    }
}

impl From<CoreMembers> for InnerAttBool {
    fn from(core_data: CoreMembers) -> Self {
        return Self {
            core: core_data,
            value: None,
            requested_value: None,
            on_change_handler: None,
            set_ensure_lock: Arc::new(Monitor::new(false)),
        };
    }
}
