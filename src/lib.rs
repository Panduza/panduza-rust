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

pub mod boolean_attribute;
pub use boolean_attribute::BooleanAttribute;

pub mod json_attribute;
pub use json_attribute::JsonAttribute;

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
