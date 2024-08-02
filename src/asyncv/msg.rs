use bytes::Bytes;

/// Members shared by all attributes
mod core_members;
pub type CoreMembers = core_members::CoreMembers;

///
pub mod att;
pub mod att_bool;

pub use super::ReactorData;

/// Trait to manage an message attribute (MQTT)
/// Sync version
pub trait OnMessageHandler: Send + Sync {
    fn on_message(&mut self, data: &Bytes);
}
