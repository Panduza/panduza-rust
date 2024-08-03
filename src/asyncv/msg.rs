use bytes::Bytes;

/// Members shared by all attributes
mod core_members;
pub type CoreMembers = core_members::CoreMembers;

///
pub mod att;
pub mod attribute_boolean;
pub use attribute_boolean::AttributeBoolean;

pub use super::ReactorData;
