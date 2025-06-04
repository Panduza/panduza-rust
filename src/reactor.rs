use crate::attribute::notification::NotificationAttribute;
use crate::attribute::status::StatusAttribute;
use crate::pubsub::Error;
use crate::structure::Structure;
use std::sync::Arc;

use crate::{
    pubsub::{new_connection, Options},
    AttributeBuilder, AttributeMetadata, AttributeMode,
};
use bytes::Bytes;
use zenoh::{
    handlers::FifoChannelHandler,
    pubsub::{Publisher, Subscriber},
    sample::Sample,
    Session,
};

/// Receiver of data payload
///
pub type DataReceiver = tokio::sync::mpsc::Receiver<Bytes>;

#[derive(Debug)]
pub struct ReactorOptions {
    pubsub_options: Options,
}

impl ReactorOptions {
    pub fn new<T: Into<String>>(ip: T, port: u16, ca_certificate: T) -> Self {
        Self {
            pubsub_options: Options::new(ip, port, ca_certificate),
        }
    }
}

/// The reactor is the main structure that will handle the connections and the events
///
/// All the attribute and objects will be powered by the reactor
///
#[derive(Clone, Debug)]
pub struct Reactor {
    ///
    ///
    pub session: Session,

    structure: Structure,
}

impl Reactor {
    ///
    ///
    pub fn new(session: Session, structure: Structure) -> Self {
        Self { session, structure }
    }

    ///
    ///
    pub fn find_attribute<A: Into<String>>(&self, name: A) -> Option<AttributeBuilder> {
        let n: String = name.into();
        let meta = self.structure.find_attribute(&n);

        if meta.is_none() {
            println!(
                "not found attribute {:?} vs {:?}",
                &n,
                self.structure.list_of_registered_topics()
            );
            return None;
        }

        println!(
            "found attribute: name = {}, meta topic = {:?}",
            n,
            meta.as_ref().map(|m| m.topic.clone())
        );

        Some(AttributeBuilder::new(self.clone(), meta))
    }

    ///
    ///
    pub fn build_instance_status_attribute<A: Into<String>>(&self, topic: A) -> AttributeBuilder {
        let meta = Some(AttributeMetadata::from_topic(
            topic.into(),
            Some("json".to_string()),
            AttributeMode::ReadOnly,
        ));

        return AttributeBuilder::new(self.clone(), meta);
    }

    /// Attribute to get access to the status of the platform
    ///
    pub async fn new_status_attribute(&self) -> StatusAttribute {
        let meta = AttributeMetadata::from_topic(
            "pza/_/status".to_string(),
            Some("status-v0".to_string()),
            AttributeMode::ReadOnly,
        );

        let builder = AttributeBuilder::new(self.clone(), Some(meta));

        builder
            .expect_status()
            .await
            .map_err(|e| e.to_string())
            .unwrap()
    }

    /// Attribute to get access to the streaming notification system of the platform
    ///
    pub async fn new_notification_attribute(&self) -> NotificationAttribute {
        let meta = AttributeMetadata::from_topic(
            "pza/_/notification".to_string(),
            Some("notification-v0".to_string()),
            AttributeMode::ReadOnly,
        );

        let builder = AttributeBuilder::new(self.clone(), Some(meta));

        builder
            .expect_notification()
            .await
            .map_err(|e| e.to_string())
            .unwrap()
    }

    // Listener
    //
    pub async fn register_listener<A: Into<String> + 'static>(
        &self,
        topic: A,
    ) -> Subscriber<FifoChannelHandler<Sample>> {
        self.session.declare_subscriber(topic.into()).await.unwrap()
    }

    ///Publisher
    ///
    pub async fn register_publisher<A: Into<String>>(&self, topic: A) -> Result<Publisher, Error> {
        Ok(self.session.declare_publisher(topic.into()).await.unwrap())
    }
}

/// Start the reactor
///
/// This function initializes the reactor and waits for the structure to be initialized.
/// If the structure initialization times out after 3 seconds, it returns an error.
pub async fn new_reactor(options: ReactorOptions) -> Result<Reactor, String> {
    // println!("0");
    let session = match new_connection(options.pubsub_options).await {
        Ok(session) => session,
        Err(e) => return Err(format!("Connection failed: {}", e)),
    };

    // println!("1");

    // let subscriber = session
    //     .declare_subscriber("pza/_/structure/att")
    //     .await
    //     .unwrap();

    let struct_query = session.get("pza/_/structure/att").await.unwrap();

    // println!("2");

    let structure = Structure::new(struct_query).await;

    // println!("3");
    // let structure_initialized = structure.initialized_notifier();
    // let timeout_duration = std::time::Duration::from_secs(15);
    // let result = tokio::time::timeout(timeout_duration, structure_initialized.notified()).await;

    // match result {
    //     Ok(_) => Ok(Reactor::new(session, structure)),
    //     Err(_) => Err("Timeout while waiting for structure initialization".to_string()),
    // }

    Ok(Reactor::new(session, structure))
}
