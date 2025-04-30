//
pub mod pubsub;

pub mod router;

/// This module manage the reactor
pub mod reactor;
pub use reactor::new_reactor;
pub use reactor::Reactor;

pub mod structure;

pub mod attribute_builder;
pub use attribute_builder::AttributeBuilder;

pub mod attribute_metadata;
pub use attribute_metadata::AttributeMetadata;

/// This module provides attribute objects
/// 
pub mod attribute;
pub use attribute::boolean::BooleanAttribute;
pub use attribute::json::JsonAttribute;

///
///
pub mod task_monitor;
use serde::Deserialize;
use serde::Serialize;
pub use task_monitor::TaskMonitor;

//
pub mod fbs;


pub mod topic;
pub use topic::Topic;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttributeMode {
    #[serde(rename = "RO")]
    ReadOnly,
    #[serde(rename = "WO")]
    WriteOnly,
    #[serde(rename = "RW")]
    ReadWrite,
}
