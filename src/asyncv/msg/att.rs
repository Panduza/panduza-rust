mod inner_msg_att;
pub type InnerAtt = inner_msg_att::InnerAtt;

use rumqttc::AsyncClient;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    pub fn new(
        reactor_data: Arc<Mutex<ReactorData>>,
        topic: String,
        mqtt_client: AsyncClient,
    ) -> Self {
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
    pub async fn init(self) -> Result<Self, AttributeError> {
        self.inner.lock().await.init(self.inner.clone()).await?;
        Ok(self)
    }

    /// Take the inner core data
    ///
    pub async fn take_core_members(self) -> Result<CoreMembers, AttributeError> {
        Ok(self.inner.lock().await.clone_core())
    }

    /// Easy conversion to AttBool
    ///
    pub async fn into_att_bool(self) -> AttBool {
        match self.take_core_members().await {
            Ok(core_data) => AttBool::from_core_members(core_data).await,
            Err(_) => panic!("Error"),
        }
    }
}
