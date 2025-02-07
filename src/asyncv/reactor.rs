mod message_engine;
use bytes::Bytes;
use message_engine::MessageEngine;

mod router;
use router::Router;
use serde_json::json;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::structure::Structure;
use crate::{AttributeError, ReactorSettings};

use rand::distributions::Alphanumeric;
use rand::Rng;
use rumqttc::AsyncClient;
use rumqttc::{Client, MqttOptions, QoS};
use std::time::Duration;

use super::attribute::message::MessageDispatcher;

use super::builder::AttributeBuilder;
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
    message_client: MessageClient,

    ///
    rules_sender: tokio::sync::mpsc::Sender<router::Rule>,
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
        // At the heart of the reactor, the router will dispatch the messages
        let mut router = Router::new(event_loop, rules_receiver);
        tokio::spawn(async move {
            router.run().await;
            println!("ReactorCore is not runiing !!!!!!!!!!!!!!!!!!!!!!");
        });

        let obj = Self {
            message_client: message_client,
            rules_sender: rules_sender,
        };

        let r = obj.register_route("pza/_/structure").await?;
        let mut stru = Structure::new(r);

        // stru.update(json!({}));

        //
        // Build the reactor
        Ok(obj)
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
    pub async fn register_route<A: Into<String>>(&self, topic: A) -> Result<DataReceiver, String> {
        //
        // Create the data channel
        let (data_sender, data_receiver) = tokio::sync::mpsc::channel::<Bytes>(100);

        //
        // Create the rules on the router
        let rule = router::Rule {
            topic: topic.into(),
            sender: data_sender,
        };
        self.rules_sender
            .send(rule)
            .await
            .map_err(|e| e.to_string())?;

        //
        // Return the receiver
        Ok(data_receiver)
    }

    // pub fn create_new_attribute(&self) -> AttributeBuilder {
    //     AttributeBuilder::new(
    //         self.message_client.as_ref().unwrap().clone(),
    //         Arc::downgrade(&self.message_dispatcher),
    //     )
    // }

    // pub async fn scan_platforms(&self) {
    //     println!("publish");
    //     self.message_client
    //         .publish("pza", QoS::AtLeastOnce, true, "pok")
    //         .await
    //         .unwrap();
    // }
}
