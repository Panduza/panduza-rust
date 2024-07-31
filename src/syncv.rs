/// This module manage the message attributes (MQTT/TCP)
pub mod msg;

/// This module manage the stream attributes (CUSTOM/QUIC)
pub mod stream;

/// This module manage the reactor
mod reactor;
pub type Reactor = reactor::Reactor;
pub type ReactorData = reactor::ReactorData;
