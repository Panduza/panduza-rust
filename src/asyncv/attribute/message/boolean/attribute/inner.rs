use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::sync::Mutex;

use bytes::Bytes;

use crate::AttributeError;

use super::MessageCoreMembers;
// use super::OnChangeHandler;
use super::OnMessageHandler;

pub use super::BuilderBoolean;
use monitor::Monitor;
// pub type OnChangeHandlerFunction = Arc<Box<dyn Future<Output = bool> + Send + Sync>>;
// pub type OnChangeHandlerFunction = Pin<Box<dyn Future<Output = bool> + Send + Sync>>;
// pub type OnChangeHandlerFunction = BoxFuture<'static, ()>;

// type OnChangeHandlerFunction = Arc<dyn 'static + Send + Sync + Fn(bool) -> BoxFuture<'static, ()>>;

/// Inner implementation of the boolean message attribute
///
pub struct InnerBoolean {
    /// Members at the core of each attribute
    core: MessageCoreMembers,
    /// Current value of the attribute
    value: Option<bool>,
    /// Requested value of the attribute (set by the user)
    requested_value: Option<bool>,
    // / Handler to call when the value change
    // on_change_handler: Option<Box<dyn OnChangeHandler>>,
    // on_change_handler: Option<OnChangeHandlerFunction>,
    // set_ensure_lock: Arc<Monitor<bool>>,
}

impl InnerBoolean {
    ///
    pub fn new(builder: BuilderBoolean) -> InnerBoolean {
        InnerBoolean {
            core: MessageCoreMembers::new(
                builder.message_client,
                builder.message_dispatcher,
                builder.topic.unwrap(),
            ),
            value: None,
            requested_value: None,
            // set_ensure_lock: None,
        }
    }

    pub fn to_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    // pub fn set_ensure_lock_clone(&mut self) -> Arc<Monitor<bool>> {
    //     return self.set_ensure_lock.clone();
    // }

    // fn set_ensure_update(&mut self) {
    //     if self.set_ensure_ok() {
    //         self.set_ensure_lock.with_lock(|mut done| {
    //             *done = true;
    //             done.notify_one();
    //         });
    //     }
    // }

    fn set_ensure_ok(&self) -> bool {
        return self.requested_value == self.value;
    }

    /// Set the value of the attribute
    ///
    pub async fn set(&mut self, new_value: bool) -> Result<(), AttributeError> {
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
                true => self.core.publish("1").await,
                false => self.core.publish("0").await,
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

    // pub fn on_change_handler(&mut self, handler: Box<dyn OnChangeHandler>) {
    //     self.on_change_handler = Some(handler);
    // }

    // pub fn on_change(&mut self, handler: OnChangeHandlerFunction) {
    //     self.on_change_handler = Some(handler);
    // }

    /// Register the attribute to the reactor
    ///
    pub async fn register(
        &self,
        attribute: Arc<Mutex<dyn OnMessageHandler>>,
    ) -> Result<(), AttributeError> {
        self.core.register(attribute).await
    }
}

// impl OnMessageHandler for InnerBoolean {
//     fn on_message(&mut self, data: &Bytes) {
//         println!("boolean");

//         // OnChangeHandlerFunction

//         // let a = async { true };
//         // let b = || a;

//         // self.on_change_handler = Some(Arc::new(|| a));
//         // tokio::spawn(b.clone()());
//         // tokio::spawn(b());
//         // tokio::spawn(b());
//         // tokio::spawn(pp55);
//         // tokio::spawn(pp55);

//         // let pp: Pin<Box<dyn Future<Output = bool> + Send>> = async move { true }.boxed();
//         // tokio::spawn(pp);
//         // tokio::spawn(pp);
//         // tokio::spawn(pp);

//         // if let Some(handler) = self.on_change_handler.as_ref() {
//         //     tokio::spawn(*(handler.clone()));
//         // }
//         if data.len() == 1 {
//             match data[0] {
//                 b'1' => {
//                     self.value = Some(true);
//                     self.set_ensure_update();
//                 }
//                 b'0' => {
//                     self.value = Some(false);
//                     self.set_ensure_update();
//                 }
//                 _ => {
//                     println!("unexcpedted payload {:?}", data);
//                     return;
//                 }
//             };
//             // Do something with the value
//         } else {
//             println!("wierd payload {:?}", data);
//         }
//     }
// }

// impl From<CoreMembers> for InnerBoolean {
//     fn from(core_data: CoreMembers) -> Self {
//         return Self {
//             core: core_data,
//             value: None,
//             requested_value: None,
//             on_change_handler: None,
//             set_ensure_lock: Arc::new(Monitor::new(false)),
//         };
//     }
// }
