use std::sync::Arc;

use crate::pubsub::{Operator, Publisher};
use crate::router::{new_router, RouterHandler};
use crate::structure::Structure;
use crate::{pubsub, pubsub::Options, AttributeBuilder};
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
    pub fn find_attribute<A: Into<String>>(&self, name: A) -> AttributeBuilder {
        let meta = self.structure.find_attribute(name);

        if meta.is_none() {
            println!(
                "not found attribute {:?}",
                self.structure.list_of_registered_topics()
            );
        }

        return AttributeBuilder::new(self.clone(), meta);
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
pub async fn new_reactor(options: ReactorOptions) -> Result<Reactor, String> {
    let router = new_router(options.pubsub_options).map_err(|e| e.to_string())?;

    let handler = router.start(None).unwrap();

    let structure_data_receiver = handler.register_listener("pza/_/structure/att", 5).await?;

    let structure = Structure::new(structure_data_receiver);
    let structure_initialized = structure.initialized_notifier();

    //stop ici
    structure_initialized.notified().await;
    Ok(Reactor::new(structure, handler))
}
