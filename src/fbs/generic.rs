use zenoh::bytes::ZBytes;

/// Trait that defines the interface for generic buffer types that can be used with GenericAttribute
///
/// All buffer types that want to work with GenericAttribute must implement this trait.
/// This provides a common interface for serialization, deserialization, and conversion operations.
pub trait GenericBuffer: Clone + Send + Sync + 'static {
    /// Create a buffer instance from ZBytes (Zenoh bytes)
    /// This is used when receiving data from Zenoh
    fn from_zbytes(zbytes: ZBytes) -> Self;

    /// Convert the buffer to ZBytes for transmission over Zenoh
    fn to_zbytes(&self) -> ZBytes;

    /// Create a buffer from a primitive value
    /// For example, BooleanBuffer::from(true) or NumberBuffer::from(42.5)
    fn from_value<T>(value: T) -> Self
    where
        T: Into<Self>;
}
