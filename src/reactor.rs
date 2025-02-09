use crate::pubsub::{PubSubOperator, Publisher};
use crate::router::{create_mqtt_router, RouterHandler};
use crate::structure::Structure;
use crate::{AttributeBuilder, PubSubError, PubSubOptions};
use bytes::Bytes;

/// Receiver of data payload
///
pub type DataReceiver = tokio::sync::mpsc::Receiver<Bytes>;

pub struct ReactorOptions {
    pubsub_options: PubSubOptions,
}

impl ReactorOptions {
    pub fn new() -> Self {
        Self {
            pubsub_options: PubSubOptions::default(),
        }
    }
}

/// The reactor is the main structure that will handle the connections and the events
///
/// All the attribute and objects will be powered by the reactor
///
#[derive(Clone)]
pub struct Reactor<O: PubSubOperator> {
    ///
    ///
    structure: Structure,

    ///
    ///
    router: RouterHandler<O>,
}

impl<O: PubSubOperator> Reactor<O> {
    ///
    ///
    pub fn new(structure: Structure, router: RouterHandler<O>) -> Self {
        Self {
            structure: structure,
            router: router,
        }
    }

    ///
    ///
    pub fn find_attribute<A: Into<String>>(&self, name: A) -> AttributeBuilder<O> {
        return AttributeBuilder::new(self.clone(), self.structure.find_attribute(name));
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
    ) -> Result<impl Publisher + '_, PubSubError> {
        self.router.register_publisher(topic.into(), retain)
    }
}

/// Start the reactor
///
pub async fn create_reactor(
    options: ReactorOptions,
) -> Result<Reactor<impl PubSubOperator>, String> {
    let mut router = create_mqtt_router(options.pubsub_options).map_err(|e| e.to_string())?;

    let handler = router.handler();

    tokio::spawn(async move {
        router.run().await;
        println!("ReactorCore is not runiing !!!!!!!!!!!!!!!!!!!!!!");
    });

    let structure_data_receiver = handler.register_listener("pza/_/structure/att", 5).await?;

    let structure = Structure::new(structure_data_receiver);
    let structure_initialized = structure.initialized_notifier();

    structure_initialized.notified().await;

    Ok(Reactor::new(structure, handler))
}
