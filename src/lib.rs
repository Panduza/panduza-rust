//
pub mod pubsub;
pub use pubsub::PubSubError;
pub use pubsub::PubSubOptions;

pub mod router;

/// This module manage the reactor
pub mod reactor;
pub use reactor::create_reactor;
pub use reactor::Reactor;

// use bytes::Bytes;

// pub mod asyncv;
// mod common;
pub mod structure;

pub mod attribute_builder;
pub use attribute_builder::AttributeBuilder;

pub mod attribute_metadata;
pub use attribute_metadata::AttributeMetadata;

pub mod boolean_attribute;
pub use boolean_attribute::BooleanAttribute;

// // --- COMMON ---

// pub type AttributeError = common::AttributeError;
// pub type ReactorSettings = common::ReactorSettings;

// /// Trait to manage an message attribute (MQTT)
// /// Sync version
// pub trait SyncMessageAttribute: Send + Sync {
//     fn on_message(&self, data: &Bytes);
// }

// pub use common::BooleanMessage;
// /// Trait for type that wan manage an attribute payload
// ///
// pub trait MessagePayloadManager:
//     Into<Vec<u8>> + From<Vec<u8>> + PartialEq + Copy + Sync + Send + 'static
// {
// }
