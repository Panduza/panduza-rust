#![deny(
    while_true,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    bad_style,
    dead_code,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens
)]

//
pub mod pubsub;

// pub mod router;
pub mod session;

/// This module manage the reactor
pub mod reactor;
pub use reactor::new_reactor;
pub use reactor::Reactor;
pub mod structure;

///
pub mod attribute_mode;
pub use attribute_mode::AttributeMode;

///
pub mod attribute_builder;
pub use attribute_builder::AttributeBuilder;

///
pub mod attribute_metadata;
pub use attribute_metadata::AttributeMetadata;

/// This module provides attribute objects
///
pub mod attribute;
pub use attribute::boolean::BooleanAttribute;
pub use attribute::bytes::BytesAttribute;
pub use attribute::json::JsonAttribute;
pub use attribute::notification::NotificationAttribute;
pub use attribute::number::NumberAttribute;
pub use attribute::status::StatusAttribute;
pub use attribute::string::StringAttribute;

///
///
pub mod task_monitor;
pub use task_monitor::TaskMonitor;

///
///
pub mod topic;
pub use topic::Topic;

/// FlatBuffers: Serialization and Deserialization
///
/// Define and manage all the network payload for Panduza.
///
/// - *panduza.fbs*: contains flatbuffer definitions
/// - The other source files are buffers helper to handle payloads.
///
pub mod fbs;
pub use fbs::BooleanBuffer;
pub use fbs::BooleanBufferBuilder;
pub use fbs::PzaBuffer;

///
///
pub mod instance_state;
pub use instance_state::InstanceState;

pub mod benchmark_config;
pub use benchmark_config::BenchmarkConfig;
