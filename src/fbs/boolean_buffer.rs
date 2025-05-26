use super::panduza_generated::panduza::{Boolean, BooleanArgs};
use bytes::Bytes;

#[derive(Debug)]
/// BooleanBuffer is a wrapper around a flatbuffer serialized Boolean value.
/// It provides methods to create, access, and manipulate boolean data.
pub struct BooleanBuffer {
    /// Internal Raw Data that holds the serialized flatbuffer
    raw_data: Bytes,
}

impl BooleanBuffer {
    /// Creates a new BooleanBuffer from existing raw serialized data
    ///
    /// # Arguments
    /// * `raw_data` - The serialized flatbuffer data
    pub fn from_raw_data(raw_data: Bytes) -> Self {
        Self { raw_data: raw_data }
    }

    /// Get a reference to the underlying raw data
    ///
    /// # Returns
    /// A reference to the serialized flatbuffer data
    pub fn raw_data(&self) -> &Bytes {
        &self.raw_data
    }

    /// Consumes the buffer and returns ownership of the raw data
    ///
    /// # Returns
    /// The serialized flatbuffer data
    pub fn take_data(self) -> Bytes {
        self.raw_data
    }

    /// Creates a new BooleanBuffer from a boolean value
    ///
    /// # Arguments
    /// * `value` - The boolean value to serialize
    ///
    /// # Returns
    /// A new BooleanBuffer containing the serialized value
    pub fn from_value(value: bool) -> Self {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let object = Boolean::create(&mut builder, &BooleanArgs { value: value });

        builder.finish(object, None);

        let raw_data = Bytes::from(builder.finished_data().to_vec());

        // Here we copy into the buffer
        Self { raw_data: raw_data }
    }
    /// Deserializes the raw data into a Boolean object
    ///
    /// # Returns
    /// The deserialized Boolean object
    pub fn object(&self) -> Boolean {
        flatbuffers::root::<Boolean>(&self.raw_data).unwrap()
    }
}

/// Implements the conversion from BooleanBuffer to bool
impl From<BooleanBuffer> for bool {
    /// Converts a BooleanBuffer to a boolean value
    ///
    /// # Returns
    /// The boolean value contained in the buffer
    fn from(buffer: BooleanBuffer) -> Self {
        buffer.object().value()
    }
}

/// Implements the conversion from a reference to BooleanBuffer to bool
impl From<&BooleanBuffer> for bool {
    /// Converts a reference to BooleanBuffer to a boolean value
    ///
    /// # Returns
    /// The boolean value contained in the buffer
    fn from(buffer: &BooleanBuffer) -> Self {
        buffer.object().value()
    }
}

/// Implements the conversion from bool to BooleanBuffer
impl From<bool> for BooleanBuffer {
    /// Creates a new BooleanBuffer from a boolean value
    ///
    /// # Arguments
    /// * `value` - The boolean value to serialize
    ///
    /// # Returns
    /// A new BooleanBuffer containing the serialized value
    fn from(value: bool) -> Self {
        BooleanBuffer::from_value(value)
    }
}
