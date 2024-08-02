pub mod reactor_core;
pub mod reactor_data;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{AttributeError, ReactorSettings};

use reactor_core::ReactorCore;
pub use reactor_data::ReactorData;

use rumqttc::AsyncClient;
use rumqttc::{Client, MqttOptions, QoS};

use std::time::Duration;

use super::msg::att::Att;
use super::msg::OnMessageHandler;

/// The reactor is the main structure that will handle the connections and the events
///
/// All the attribute and objects will be powered by the reactor
///
#[derive(Clone)]
pub struct Reactor {
    core: Arc<Mutex<ReactorCore>>,
    data: Arc<Mutex<ReactorData>>,

    mqtt_client: AsyncClient,
}

impl Reactor {
    /// Create a new Reactor
    ///
    /// # Arguments
    ///
    /// * `core` - The core of the reactor
    ///
    pub fn new(settings: ReactorSettings) -> Self {
        println!("ReactorCore is running");
        let mut mqttoptions = MqttOptions::new("rumqtt-sync", "localhost", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, connection) = AsyncClient::new(mqttoptions, 100);

        let data = Arc::new(Mutex::new(ReactorData::new()));

        Reactor {
            core: Arc::new(Mutex::new(ReactorCore::new(data.clone(), connection))),
            data: data,
            mqtt_client: client,
        }
    }

    pub fn run_in_thread(&self) {
        let core = self.core.clone();
        tokio::spawn(async move {
            core.lock().await.run().await;
            println!("ReactorCore is not runiing !!!!!!!!!!!!!!!!!!!!!!");
        });
    }

    /// Create a new attribute from its topic
    ///
    pub async fn attribute_from_topic(&self, topic: &str) -> Result<Att, AttributeError> {
        Att::new(
            self.data.clone(),
            topic.to_string(),
            self.mqtt_client.clone(),
        )
        .init()
        .await
    }

    pub async fn scan_platforms(&self) {
        println!("publish");
        self.mqtt_client
            .publish("pza", QoS::AtLeastOnce, true, "pok")
            .await
            .unwrap();
    }
}
