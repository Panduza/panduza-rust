pub mod attribute;

/// This module manage the message attributes (MQTT/TCP)
// pub mod msg;
pub type MessageClient = rumqttc::AsyncClient;

/// This module manage the stream attributes (CUSTOM/QUIC)
// pub mod stream;

/// This module manage the reactor
pub mod reactor;
pub type Reactor = reactor::Reactor;
