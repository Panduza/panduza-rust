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
