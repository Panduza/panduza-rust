use bytes::Bytes;

mod router;
use router::{Router, RuleSender};

use crate::attribute_builder::AttributeBuilder;
use crate::structure::Structure;
use crate::ReactorSettings;

use rand::distributions::Alphanumeric;
use rand::Rng;
use rumqttc::AsyncClient;
use rumqttc::{MqttOptions, QoS};
use std::time::Duration;

use super::MessageClient;

/// Receiver of data payload
///
pub type DataReceiver = tokio::sync::mpsc::Receiver<Bytes>;

/// The reactor is the main structure that will handle the connections and the events
///
/// All the attribute and objects will be powered by the reactor
///
#[derive(Clone)]
pub struct Reactor {
    /// The mqtt client
    pub message_client: MessageClient,

    ///
    rules_sender: tokio::sync::mpsc::Sender<router::Rule>,

    structure: Structure,
}

impl Reactor {
    /// Start the reactor
    ///
    pub async fn start(settings: ReactorSettings) -> Result<Self, String> {
        println!("ReactorCore is running");
        let mut mqttoptions = MqttOptions::new(
            format!("panduza-rust-client-{}", Self::generate_random_string(10)),
            "127.0.0.1",
            1883,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, event_loop) = AsyncClient::new(mqttoptions, 100);

        client.subscribe("pza/_/#", QoS::AtLeastOnce).await.unwrap();
        let message_client = client;

        //
        // Create the rules channel
        let (rules_sender, rules_receiver) = tokio::sync::mpsc::channel(100);

        //
        // Create the structure
        let structure_data_receiver =
            Self::static_register_route(&rules_sender, "pza/_/structure/att", 5).await?;
        let structure = Structure::new(structure_data_receiver);

        let structure_initialized = structure.initialized_notifier();

        //
        // At the heart of the reactor, the router will dispatch the messages
        let mut router = Router::new(message_client.clone(), event_loop, rules_receiver);
        tokio::spawn(async move {
            router.run().await;
            println!("ReactorCore is not runiing !!!!!!!!!!!!!!!!!!!!!!");
        });

        structure_initialized.notified().await;

        //
        // Build the reactor
        Ok(Self {
            message_client: message_client,
            rules_sender: rules_sender,
            structure: structure,
        })
    }

    fn generate_random_string(length: usize) -> String {
        let rng = rand::thread_rng();
        rng.sample_iter(Alphanumeric)
            .take(length)
            .map(|c| c as char)
            .collect()
    }

    /// Register a new route
    ///
    pub fn register_route<A: Into<String> + 'static>(
        &self,
        topic: A,
        channel_size: usize,
    ) -> impl std::future::Future<Output = Result<DataReceiver, String>> + '_ {
        Self::static_register_route(&self.rules_sender, topic, channel_size)
    }

    /// Register a new route
    ///
    async fn static_register_route<A: Into<String>>(
        rules_sender: &RuleSender,
        topic: A,
        channel_size: usize,
    ) -> Result<DataReceiver, String> {
        //
        // Create the data channel
        let (data_sender, data_receiver) = tokio::sync::mpsc::channel::<Bytes>(channel_size);

        //
        // Create the rules on the router
        rules_sender
            .send(router::Rule {
                topic: topic.into(),
                sender: data_sender,
            })
            .await
            .map_err(|e| e.to_string())?;

        //
        // Return the receiver
        Ok(data_receiver)
    }

    /// find_attribute().try_into
    ///
    pub fn find_attribute<A: Into<String>>(&self, name: A) -> AttributeBuilder {
        return AttributeBuilder::new(self.clone(), self.structure.find_attribute(name));
    }

    // pub async fn scan_platforms(&self) {
    //     println!("publish");
    //     self.message_client
    //         .publish("pza", QoS::AtLeastOnce, true, "pok")
    //         .await
    //         .unwrap();
    // }
}
