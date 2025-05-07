use crate::pubsub::Error;
use crate::structure::Structure;
use crate::{
    pubsub::{new_connection, Options},
    AttributeBuilder,
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
    pub fn find_attribute<A: Into<String>>(&self, name: A) -> AttributeBuilder {
        let meta = self.structure.find_attribute(name);

        println!("METADATAS : {:?}", meta);

        if meta.is_none() {
            println!(
                "not found attribute {:?}",
                self.structure.list_of_registered_topics()
            );
        }

        return AttributeBuilder::new(self.clone(), meta);
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
pub async fn new_reactor(options: ReactorOptions) -> Result<Reactor, String> {
    let session = new_connection(options.pubsub_options)
        .await
        .map_err(|e| e.to_string())?;

    let subscriber = session
        .declare_subscriber("pza/_/structure/att")
        .await
        .unwrap();

    let structure = Structure::new(subscriber);

    // let structure_initialized = structure.initialized_notifier();

    let operator = Reactor::new(session, structure);

    //stop ici
    // structure_initialized.notified().await;
    Ok(operator)
}
