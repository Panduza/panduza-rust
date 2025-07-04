use super::panduza_generated::panduza::Message;
use zenoh::bytes::ZBytes;

/// Trait that defines the interface for generic buffer types that can be used with GenericAttribute
///
/// All buffer types that want to work with GenericAttribute must implement this trait.
/// This provides a common interface for serialization, deserialization, and conversion operations.
pub trait PanduzaBuffer: Clone + Default + Send + Sync + 'static {
    /// Create a new instance of the buffer builder
    /// It will use default values for the fields
    fn new() -> Self;

    ///
    ///
    fn with_value<T>(self, value: T) -> Self
    where
        T: Into<Self>;

    ///
    fn with_source(self, source: u16) -> Self;

    ///
    fn with_sequence(self, sequence: u16) -> Self;

    ///
    fn with_random_sequence(self) -> Self;

    ///
    fn build(self) -> Result<Self, String>;

    /// Create a buffer instance from ZBytes (Zenoh bytes)
    /// This is used when receiving data from Zenoh
    fn build_from_zbytes(zbytes: ZBytes) -> Self;

    ///
    fn is_builded(&self) -> bool;

    ///
    fn sequence(&self) -> u16;

    /// Convert the buffer to ZBytes for transmission over Zenoh
    fn to_zbytes(self) -> ZBytes;

    ///
    ///
    fn as_message(&self) -> Message;

    ///
    ///
    fn has_value_equal_to_message_value(&self, message: &Message) -> bool;
}
