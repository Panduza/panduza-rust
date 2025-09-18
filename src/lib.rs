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

// pub mod router;
pub mod session;

/// Configuration module for client settings
///
/// *Maturity Level*: good
///
/// This module provides configuration structures for platform and security settings.
/// It supports serialization/deserialization with serde for easy integration with
/// configuration files (JSON, TOML, etc.).
///
pub mod config;
pub use config::{Config, EndpointConfig, SecurityConfig};

/// Connection module for Zenoh session management
///
/// *Maturity Level*: good
///
/// This module provides easy functions to create Zenoh sessions for Panduza purpose.
/// It handles different connection types based on security configuration.
///
pub mod connection;
pub use connection::{create_client_connection, ConnectionError};

/// This module manage the Reactor
///
/// *Maturity Level*: good
///
/// The reactor is the main entry point for the Panduza client library.
/// Any user must create a reactor instance to be able to use the library.
///
/// The reactor create and manage a Zenoh session to communicate with Panduza Platform.
///
/// The reactor also provide a builder.
///
// ```rust
/// let reactor = Reactor::builder()
///     .with_platform_addr("127.0.0.1")
///     .with_platform_port(7447)
///     .disable_security()
///     .build();
/// ```
///
pub mod reactor;
pub use reactor::Reactor;
pub use reactor::ReactorBuilder;
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
pub use attribute::notification::NotificationAttribute;
pub use attribute::number::NumberAttribute;
pub use attribute::status::StatusAttribute;
pub use attribute::string::StringAttribute;
pub use attribute::structure::StructureAttribute;

///
///
pub mod task_monitor;
pub use task_monitor::TaskMonitor;

/// Module to manage topic helper
///
mod topic;
pub use topic::Topic;

/// This module provides handy functions to access to all standardized paths of Panduza on systems.
/// This module works for any OS (Windows, Linux, Mac)
///
pub mod path;

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

/// Module for AI model interface with Panduza Reactor
///
pub mod executor;
pub use executor::Executor;

///
///
pub mod security;
