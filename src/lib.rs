use bytes::Bytes;

pub mod asyncv;
mod common;
pub mod syncv;

// --- COMMON ---

pub type AttributeError = common::AttributeError;
pub type ReactorSettings = common::ReactorSettings;

/// Trait to manage an message attribute (MQTT)
/// Sync version
pub trait SyncMessageAttribute: Send + Sync {
    fn on_message(&self, data: &Bytes);
}

pub use common::BooleanMessage;
/// Trait for type that wan manage an attribute payload
///
pub trait AttributePayloadManager:
    Into<Vec<u8>> + From<Vec<u8>> + PartialEq + Copy + Sync + Send + 'static
{
}
