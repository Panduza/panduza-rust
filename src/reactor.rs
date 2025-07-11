use crate::attribute::notification::NotificationAttribute;
use crate::attribute::status::StatusAttribute;
use crate::pubsub::Publisher;
use crate::router::{new_router, RouterHandler};
use crate::structure::Structure;
use crate::{pubsub, pubsub::Options, AttributeBuilder};
use crate::{AttributeMetadata, AttributeMode};
use bytes::Bytes;

/// Receiver of data payload
///
pub type DataReceiver = tokio::sync::mpsc::Receiver<Bytes>;

pub struct ReactorOptions {
    pubsub_options: Options,
}

impl ReactorOptions {
    pub fn new<T: Into<String>>(ip: T, port: u16) -> Self {
        Self {
            pubsub_options: Options::new(ip, port),
        }
    }
}

/// The reactor is the main structure that will handle the connections and the events
///
/// All the attribute and objects will be powered by the reactor
///
#[derive(Clone)]
pub struct Reactor {
    ///
    ///
    structure: Structure,

    ///
    ///
    router: RouterHandler,
}

impl Reactor {
    ///
    ///
    pub fn new(structure: Structure, router: RouterHandler) -> Self {
        Self {
            structure: structure,
            router: router,
        }
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

        println!("found attribute: name = {}, meta topic = {:?}", n, meta.as_ref().map(|m| m.topic.clone()));

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

    // Register
    //
    pub fn register_listener<A: Into<String> + 'static>(
        &self,
        topic: A,
        channel_size: usize,
    ) -> impl std::future::Future<Output = Result<DataReceiver, String>> + '_ {
        self.router.register_listener(topic, channel_size)
    }

    ///
    ///
    pub fn register_publisher<A: Into<String> + 'static>(
        &self,
        topic: A,
        retain: bool,
    ) -> Result<Publisher, pubsub::Error> {
        self.router.register_publisher(topic.into(), retain)
    }
}

/// Start the reactor
///
/// This function initializes the reactor and waits for the structure to be initialized.
/// If the structure initialization times out after 3 seconds, it returns an error.
pub async fn new_reactor(options: ReactorOptions) -> Result<Reactor, String> {
    let router = new_router(options.pubsub_options).map_err(|e| e.to_string())?;

    let handler = router.start(None).unwrap();

    let structure_data_receiver = handler.register_listener("pza/_/structure/att", 5).await?;

    let structure = Structure::new(structure_data_receiver);
    let structure_initialized = structure.initialized_notifier();

    let timeout_duration = std::time::Duration::from_secs(15);
    let result = tokio::time::timeout(timeout_duration, structure_initialized.notified()).await;

    match result {
        Ok(_) => Ok(Reactor::new(structure, handler)),
        Err(_) => Err("Timeout while waiting for structure initialization".to_string()),
    }
}
