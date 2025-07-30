#[allow(warnings)]
pub mod panduza_generated;
use panduza_generated::panduza::Message;
use panduza_generated::panduza::Timestamp;

/// Structure buffer
///
mod structure_buffer;
pub use structure_buffer::AttributeEntryBuffer;
pub use structure_buffer::AttributeEntryBufferBuilder;
pub use structure_buffer::StructureBuffer;
pub use structure_buffer::StructureBufferBuilder;

///
mod boolean_buffer;
pub use boolean_buffer::BooleanBuffer;
pub use boolean_buffer::BooleanBufferBuilder;

///
mod string_buffer;
pub use string_buffer::StringBuffer;

///
mod bytes_buffer;
pub use bytes_buffer::BytesBuffer;

///
mod number_buffer;
pub use number_buffer::NumberBuffer;

///
mod notification_buffer;
pub use notification_buffer::NotificationBuffer;
pub use notification_buffer::NotificationBufferBuilder;
pub use notification_buffer::NotificationType;

pub mod status_buffer;
pub use status_buffer::InstanceStatusBuffer;
pub use status_buffer::StatusBuffer;
pub use status_buffer::StatusBufferBuilder;

use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use zenoh::bytes::ZBytes;

/// Generates a timestamp for message headers using the current system time
///
/// # Returns
/// A Timestamp object with current time in seconds and nanoseconds
pub fn generate_timestamp() -> Timestamp {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let seconds = since_the_epoch.as_secs();
    let nanoseconds = since_the_epoch.subsec_nanos();
    Timestamp::new(seconds, nanoseconds)
}

/// Error type for PzaBuffer operations
///
#[derive(Error, Debug)]
pub enum PzaBufferError {
    #[error("Invalid flatbuffer data")]
    InvalidData,
    #[error("Missing payload")]
    MissingPayload,
    #[error("Serialization failed")]
    SerializationError,
}

/// Trait that defines the interface for buffer builders
///
pub trait PzaBufferBuilder<B: PzaBuffer>: Clone + Default + Send + Sync + 'static {
    /// Sets the source sender for the buffer
    ///
    fn with_source(self, source: u16) -> Self;

    /// Sets the sequence number for the buffer
    ///
    fn with_sequence(self, sequence: u16) -> Self;

    /// Generates a random sequence number for the buffer
    ///
    fn with_random_sequence(self) -> Self;

    /// Transforms the builder into a buffer with the specified value
    ///
    fn build(self) -> Result<B, String>;
}

/// Trait that defines the interface for generic buffer types that can be used with
/// [StdObjAttribute](crate::attribute::std_obj::StdObjAttribute) and
/// [RoStreamAttribute](crate::attribute::ro_stream::RoStreamAttribute)
///
pub trait PzaBuffer: Clone + Default + Send + Sync + 'static {
    /// Create a buffer instance from ZBytes (Zenoh bytes)
    /// This is used when receiving data from Zenoh
    /// PzaBuffer implementations is based on bytes::Bytes (to work with fbs)
    fn from_zbytes(zbytes: ZBytes) -> Self;

    /// Convert the buffer to ZBytes for transmission over Zenoh
    fn to_zbytes(self) -> ZBytes;

    /// Returns the size in bytes
    ///
    fn size(&self) -> usize;

    /// Returns the source of the buffer
    ///
    fn source(&self) -> Option<u16>;

    /// Returns the sequence number of the buffer
    ///
    fn sequence(&self) -> Option<u16>;

    ///
    ///
    fn as_message(&self) -> Message;

    ///
    ///
    fn has_same_message_value<B: PzaBuffer>(&self, other_buffer: &B) -> bool;
}
