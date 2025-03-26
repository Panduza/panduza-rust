// use super::{Error, IncomingUpdate, Options, PubSubEvent};
// use bytes::Bytes;
// use rand::distributions::Alphanumeric;
// use rand::Rng;
// use rumqttc::{AsyncClient, EventLoop, Publish};
// use rumqttc::{MqttOptions, QoS};
// use std::time::Duration;

// /// Generate a random string for connections IDs
// ///
// pub fn generate_random_string(length: usize) -> String {
//     let rng = rand::thread_rng();
//     rng.sample_iter(Alphanumeric)
//         .take(length)
//         .map(|c| c as char)
//         .collect()
// }

// /// Convert options from PubSub API to mqtt options
// ///
// fn pubsub_options_to_mqtt_options(options: Options) -> MqttOptions {
//     let mut mqttoptions = MqttOptions::new(
//         format!("panduza-{}", generate_random_string(10)),
//         options.ip,
//         options.port,
//     );

//     mqttoptions.set_keep_alive(Duration::from_secs(3));
//     mqttoptions.set_max_packet_size(1024 * 1000, 1024 * 1000);

//     mqttoptions
// }

// /// Start a MQTT connection and return object of the PubSub API
// ///
// pub fn new_connection(options: Options) -> Result<(Operator, Listener), Error> {
//     //
//     let mqtt_options = pubsub_options_to_mqtt_options(options);

//     //
//     let (client, event_loop) = AsyncClient::new(mqtt_options, 100);

//     //
//     Ok((Operator::new(client), Listener::new(event_loop)))
// }

// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------

// pub struct Listener {
//     event_loop: EventLoop,
// }

// impl Listener {
//     ///
//     ///
//     pub fn new(event_loop: EventLoop) -> Self {
//         Self {
//             event_loop: event_loop,
//         }
//     }

//     ///
//     ///
//     pub fn process_incoming_publish(packet: Publish) -> PubSubEvent {
//         PubSubEvent::IncomingUpdate(IncomingUpdate {
//             topic: packet.topic,
//             payload: packet.payload,
//         })
//     }
// }

// impl Listener {
//     ///
//     ///
//     pub async fn poll(&mut self) -> Result<PubSubEvent, Error> {
//         loop {
//             match self.event_loop.poll().await {
//                 Ok(event) => {
//                     // println!("*** Notification = {:?}", event);
//                     match event {
//                         rumqttc::Event::Incoming(incoming) => match incoming {
//                             rumqttc::Packet::Publish(packet) => {
//                                 return Ok(Self::process_incoming_publish(packet));
//                             }
//                             _ => {}
//                         },
//                         _ => {}
//                     }
//                 }
//                 Err(e) => {
//                     return Err(Error::ListenError {
//                         cause: e.to_string(),
//                     })
//                 }
//             }
//         }
//     }
// }

// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------

// #[derive(Clone)]
// ///
// ///
// pub struct Operator {
//     client: AsyncClient,
// }

// impl Operator {
//     pub fn new(client: AsyncClient) -> Self {
//         Self { client: client }
//     }

//     ///
//     ///
//     pub fn declare_publisher(&self, topic: String, retain: bool) -> Result<Publisher, Error> {
//         Ok(Publisher::new(self.client.clone(), topic, retain))
//     }

//     ///
//     ///
//     pub fn declare_subscriber(&self) -> Result<Subscriber, Error> {
//         Ok(Subscriber::new(self.client.clone()))
//     }
// }

// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------

// #[derive(Clone, Debug)]
// ///
// ///
// pub struct Publisher {
//     client: AsyncClient,
//     topic: String,
//     retain: bool,
// }

// impl Publisher {
//     pub fn new(client: AsyncClient, topic: String, retain: bool) -> Self {
//         Self {
//             client: client,
//             topic: topic,
//             retain: retain,
//         }
//     }

//     /// Publish the payload
//     ///
//     pub async fn publish(&self, payload: Bytes) -> Result<(), Error> {
//         self.client
//             .publish_bytes(&self.topic, QoS::AtMostOnce, self.retain, payload)
//             .await
//             .map_err(|e| Error::PublishError {
//                 topic: self.topic.clone(),
//                 cause: e.to_string(),
//             })
//     }
// }

// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------

// ///
// ///
// pub struct Subscriber {
//     client: AsyncClient,
// }

// impl Subscriber {
//     pub fn new(client: AsyncClient) -> Self {
//         Self { client: client }
//     }

//     ///
//     ///
//     pub async fn subscribe(&self, topic: String) -> Result<(), Error> {
//         self.client
//             .subscribe(&topic, QoS::AtMostOnce)
//             .await
//             .map_err(|e| Error::SubscribeError {
//                 topic: topic,
//                 cause: e.to_string(),
//             })
//     }
// }
