use crate::attribute::notification::NotificationAttribute;
use crate::attribute::status::StatusAttribute;
use crate::pubsub::Error;
use crate::structure::Structure;

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

#[derive(Debug, Clone)]
pub struct ReactorOptions {
    pubsub_options: Options,
}

impl ReactorOptions {
    pub fn new<T: Into<String>>(
        ip: T,
        port: u16,
        ca_certificate: T,
        connect_certificate: T,
        connect_private_key: T,
        namespace: Option<T>,
    ) -> Self {
        Self {
            pubsub_options: Options::new(
                ip,
                port,
                ca_certificate,
                connect_certificate,
                connect_private_key,
                namespace,
            ),
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

    pub namespace: Option<String>,
}

/// Very important to implement to ease Dioxus usage
impl PartialEq for Reactor {
    fn eq(&self, other: &Self) -> bool {
        self.session.zid() == other.session.zid()
    }
}

impl Reactor {
    ///
    ///
    pub fn new(session: Session, structure: Structure, namespace: Option<String>) -> Self {
        Self {
            session,
            structure,
            namespace,
        }
    }

    ///
    ///
    pub fn find_attribute<A: Into<String>>(&self, name: A) -> Option<AttributeBuilder> {
        let n: String = name.into();
        let meta = self.structure.find_attribute(&n);

        // println!("found metadata: {:?} for attribute '{}'", meta, &n);

        // self.structure
        //     .list_of_registered_topics()
        //     .iter()
        //     .for_each(|topic| println!("Registered topic: {}", topic));

        if meta.is_none() {
            return None;
        }

        Some(AttributeBuilder::new(self.clone(), meta))
    }

    ///
    ///
    pub fn build_instance_status_attribute<A: Into<String>>(&self, topic: A) -> AttributeBuilder {
        let meta = Some(AttributeMetadata::from_topic(
            format!(
                "{}{}",
                self.namespace
                    .clone()
                    .map_or("".to_string(), |ns| if ns.is_empty() {
                        "".to_string()
                    } else {
                        format!("{}/", ns)
                    }),
                topic.into()
            ),
            Some("json".to_string()),
            AttributeMode::ReadOnly,
        ));

        return AttributeBuilder::new(self.clone(), meta);
    }

    // fn format_topic(&self, topic: &str) -> String {
    //     if let Some(namespace) = &self.namespace {
    //         if topic.starts_with("pza/") {
    //             format!("{}/{}", namespace, topic)
    //         } else {
    //             topic.to_string()
    //         }
    //     } else {
    //         topic.to_string()
    //     }
    // }

    /// Attribute to get access to the status of the platform
    ///
    pub async fn new_status_attribute(&self) -> StatusAttribute {
        let meta = AttributeMetadata::from_topic(
            format!(
                "{}{}",
                self.namespace
                    .clone()
                    .map_or("".to_string(), |ns| if ns.is_empty() {
                        "".to_string()
                    } else {
                        format!("{}/", ns)
                    }),
                "pza/_/status"
            ),
            Some("status".to_string()),
            AttributeMode::ReadOnly,
        );

        let builder = AttributeBuilder::new(self.clone(), Some(meta));

        builder
            .try_into_status()
            .await
            .map_err(|e| e.to_string())
            .expect("Failed to create status attribute")
    }

    /// Attribute to get access to the streaming notification system of the platform
    ///
    pub async fn new_notification_attribute(&self) -> NotificationAttribute {
        let meta = AttributeMetadata::from_topic(
            format!(
                "{}{}",
                self.namespace
                    .clone()
                    .map_or("".to_string(), |ns| if ns.is_empty() {
                        "".to_string()
                    } else {
                        format!("{}/", ns)
                    }),
                "pza/_/notifications"
            ),
            Some("notification".to_string()),
            AttributeMode::ReadOnly,
        );

        let builder = AttributeBuilder::new(self.clone(), Some(meta));

        builder
            .try_into_notification()
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
    let session = match new_connection(options.pubsub_options.clone()).await {
        Ok(session) => session,
        Err(e) => return Err(format!("Connection failed: {}", e)),
    };

    let namespace = options.pubsub_options.namespace.clone();

    // println!(
    //     "{}",
    //     format!(
    //         "{}pza/_/structure/att",
    //         options
    //             .pubsub_options
    //             .namespace
    //             .clone()
    //             .map_or("".to_string(), |ns| format!("{}/", ns))
    //     )
    // );

    let struct_query = session
        .get(format!(
            "{}pza/_/structure/att",
            options.pubsub_options.namespace.clone().map_or(
                "".to_string(),
                |ns| if ns.is_empty() {
                    "".to_string()
                } else {
                    format!("{}/", ns)
                }
            )
        ))
        .await
        .expect("Failed to get structure attribute");

    let structure = Structure::new(struct_query).await;

    Ok(Reactor::new(session, structure, namespace))
}
