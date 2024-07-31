pub use super::CoreMembers;
use super::OnMessageHandler;
use super::ReactorData;
use crate::AttributeError;
use bytes::Bytes;
use rumqttc::Client;
use std::sync::{Arc, Mutex, Weak};

/// Inner implementation of the generic message attribute
///
pub struct InnerAtt {
    /// Members at the core of each attribute
    core: CoreMembers,
}

impl InnerAtt {
    /// Create a new InnerAtt
    ///
    pub fn new(reactor_data: Weak<Mutex<ReactorData>>, topic: String, mqtt_client: Client) -> Self {
        Self {
            core: CoreMembers::new(reactor_data, topic, mqtt_client),
        }
    }

    /// Convert the InnerAtt to an Arc<Mutex<InnerAtt>>
    ///
    pub fn into_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    /// Initialize the attribute
    ///
    pub fn init(&self, attribute: Arc<Mutex<dyn OnMessageHandler>>) -> Result<(), AttributeError> {
        self.core.init(attribute)
    }

    /// Clone the core members (to mutate into an other type)
    ///
    pub fn clone_core(&self) -> CoreMembers {
        self.core.clone()
    }
}

impl OnMessageHandler for InnerAtt {
    fn on_message(&mut self, data: &Bytes) {
        println!("generic");
    }
}
