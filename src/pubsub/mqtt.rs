use super::{
    PubSubError, PubSubEvent, PubSubListener, PubSubOperator, PubSubOptions, Publisher, Subscriber,
};
use async_trait::async_trait;
use bytes::Bytes;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rumqttc::{AsyncClient, EventLoop};
use rumqttc::{MqttOptions, QoS};
use std::time::Duration;

/// Generate a random string for connections IDs
///
pub fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(Alphanumeric)
        .take(length)
        .map(|c| c as char)
        .collect()
}

/// Convert options from PubSub API to mqtt options
///
fn pubsub_options_to_mqtt_options(options: PubSubOptions) -> MqttOptions {
    let mut mqttoptions = MqttOptions::new(
        format!("panduza-{}", generate_random_string(10)),
        "127.0.0.1",
        1883,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(3));
    mqttoptions
}

/// Start a MQTT connection and return object of the PubSub API
///
pub fn start_connection<O: PubSubOperator, L: PubSubListener>(
    options: PubSubOptions,
) -> Result<(O, L), PubSubError> {
    //
    let mqtt_options = pubsub_options_to_mqtt_options(options);

    //
    let (client, event_loop) = AsyncClient::new(mqtt_options, 100);
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct MqttListener {
    event_loop: EventLoop,
}

impl MqttListener {
    pub fn new(event_loop: EventLoop) -> Self {
        Self {
            event_loop: event_loop,
        }
    }
}

#[async_trait]
impl PubSubListener for MqttListener {
    ///
    ///
    async fn poll(&mut self) -> Result<PubSubEvent, PubSubError> {}
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct MqttOperator {
    client: AsyncClient,
}

impl MqttOperator {
    pub fn new(client: AsyncClient) -> Self {
        Self { client: client }
    }
}

impl PubSubOperator for MqttOperator {
    ///
    ///
    fn declare_publisher(&self) -> Result<impl Publisher, PubSubError> {}

    ///
    ///
    fn declare_subscriber(&self) -> Result<impl Subscriber, PubSubError> {
        Ok(MqttSubscriber::new(self.client.clone()))
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct MqttPublisher {
    client: AsyncClient,
}

impl MqttPublisher {
    pub fn new(client: AsyncClient) -> Self {
        Self { client: client }
    }
}

#[async_trait]
impl Publisher for MqttPublisher {
    ///
    ///
    async fn publish(&self, payload: Bytes) -> Result<(), PubSubError> {}
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct MqttSubscriber {
    client: AsyncClient,
}

impl MqttSubscriber {
    pub fn new(client: AsyncClient) -> Self {
        Self { client: client }
    }
}

#[async_trait]
impl Subscriber for MqttSubscriber {
    ///
    ///
    async fn subscribe<S: Into<String>>(&self, topic: S) -> Result<(), PubSubError> {}
}
