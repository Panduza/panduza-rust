mod inner_msg_att;
pub type InnerAtt = inner_msg_att::InnerAtt;

use rumqttc::Client;
use std::sync::{Arc, Mutex};

use crate::AttributeError;

use super::att_bool::AttBool;
pub use super::CoreMembers;
pub use super::OnMessageHandler;
pub use super::ReactorData;

/// Generic Message Attribute
pub struct Att {
    /// Attribute are mainly a wrapper for the inner manager
    inner: Arc<Mutex<InnerAtt>>,
}

impl Att {
    /// Create a new Message Attribute
    pub fn new(reactor_data: Arc<Mutex<ReactorData>>, topic: String, mqtt_client: Client) -> Self {
        // Create a new inner manager
        let inner = InnerAtt::new(
            Arc::downgrade(&reactor_data),
            topic.clone(),
            mqtt_client.clone(),
        )
        .into_arc_mutex();

        Self { inner: inner }
    }

    /// Initialize the attribute
    ///
    pub fn init(self) -> Result<Self, AttributeError> {
        self.inner
            .lock()
            .map_err(|e| AttributeError::InternalMutex(e.to_string()))?
            .init(self.inner.clone())?;
        Ok(self)
    }

    /// Take the inner core data
    ///
    pub fn take_core_members(self) -> Result<CoreMembers, AttributeError> {
        Ok(self
            .inner
            .lock()
            .map_err(|e| AttributeError::InternalMutex(e.to_string()))?
            .clone_core())
    }

    /// Easy conversion to AttBool
    ///
    pub fn into_att_bool(self) -> AttBool {
        self.into()
    }
}
