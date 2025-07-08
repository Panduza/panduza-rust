#[allow(warnings)]
pub mod panduza_generated;
use panduza_generated::panduza::Message;
use panduza_generated::panduza::Timestamp;

pub mod common;

///
mod boolean_buffer;
pub use boolean_buffer::BooleanBuffer;

///
mod string_buffer;
pub use string_buffer::StringBuffer;

///
mod bytes_buffer;
pub use bytes_buffer::BytesBuffer;

mod number_buffer;
pub use number_buffer::NumberBuffer;

mod notification_buffer;
pub use notification_buffer::NotificationBuffer;
pub use notification_buffer::NotificationType;

pub mod status_buffer;
pub use status_buffer::InstanceStatusBuffer;
pub use status_buffer::StatusBuffer;

mod generic;
pub use generic::PanduzaBuffer;

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
    /// Sets the value for the buffer
    /// Can be anyting that can be converted into the buffer builder
    ///
    fn with_value<T>(self, value: T) -> Self
    where
        T: Into<Self>;

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

/// Trait that defines the interface for generic buffer types that can be used with GenericAttribute
///
pub trait PzaBuffer: Clone + Default + Send + Sync + 'static {
    /// Create a buffer instance from ZBytes (Zenoh bytes)
    /// This is used when receiving data from Zenoh
    /// PzaBuffer implementations is based on bytes::Bytes (to work with fbs)
    fn from_zbytes(zbytes: ZBytes) -> Self;

    /// Convert the buffer to ZBytes for transmission over Zenoh
    fn to_zbytes(self) -> ZBytes;

    /// Returns the source of the buffer
    ///
    fn source(&self) -> u16;

    /// Returns the sequence number of the buffer
    ///
    fn sequence(&self) -> u16;

    ///
    ///
    fn as_message(&self) -> Message;

    ///
    ///
    fn has_value_equal_to_message_value(&self, message: &Message) -> bool;
}
