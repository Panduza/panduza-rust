mod message_engine;
use message_engine::MessageEngine;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{AttributeError, ReactorSettings};

use rand::distributions::Alphanumeric;
use rand::Rng;
use rumqttc::AsyncClient;
use rumqttc::{Client, MqttOptions, QoS};
use std::time::Duration;

use super::attribute::message::MessageDispatcher;

use super::builder::AttributeBuilder;
use super::MessageClient;

/// The reactor is the main structure that will handle the connections and the events
///
/// All the attribute and objects will be powered by the reactor
///
#[derive(Clone)]
pub struct Reactor {
    /// The mqtt client
    message_client: Option<MessageClient>,

    ///
    message_dispatcher: Arc<Mutex<MessageDispatcher>>,
}

impl Reactor {
    /// Create a new Reactor
    ///
    /// # Arguments
    ///
    /// * `core` - The core of the reactor
    ///
    pub fn new(settings: ReactorSettings) -> Self {
        // let data = ;

        Reactor {
            message_client: None,
            message_dispatcher: Arc::new(Mutex::new(MessageDispatcher::new())),
        }
    }

    pub async fn start(&mut self) {
        println!("ReactorCore is running");
        let mut mqttoptions = MqttOptions::new(
            format!("rumqtt-sync-{}", Self::generate_random_string(5)),
            "127.0.0.1",
            1883,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, event_loop) = AsyncClient::new(mqttoptions, 100);

        client.subscribe("pza/_/#", QoS::AtLeastOnce).await.unwrap();
        self.message_client = Some(client);

        let mut message_engine = MessageEngine::new(self.message_dispatcher.clone(), event_loop);
        tokio::spawn(async move {
            message_engine.run().await;
            println!("ReactorCore is not runiing !!!!!!!!!!!!!!!!!!!!!!");
        });
    }

    fn generate_random_string(length: usize) -> String {
        let rng = rand::thread_rng();
        rng.sample_iter(Alphanumeric)
            .take(length)
            .map(|c| c as char)
            .collect()
    }
    pub fn create_new_attribute(&self) -> AttributeBuilder {
        AttributeBuilder::new(
            self.message_client.as_ref().unwrap().clone(),
            Arc::downgrade(&self.message_dispatcher),
        )
    }

    // pub async fn scan_platforms(&self) {
    //     println!("publish");
    //     self.message_client
    //         .publish("pza", QoS::AtLeastOnce, true, "pok")
    //         .await
    //         .unwrap();
    // }
}
