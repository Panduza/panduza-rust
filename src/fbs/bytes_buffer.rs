use super::common::{generate_timestamp, BufferError};
use super::panduza_generated::panduza::{
    Bytes, BytesArgs, Header, HeaderArgs, Message, MessageArgs, Payload,
};
use bytes::Bytes as BytesType;
use rand::Rng;
use std::fmt;

type Result<T> = std::result::Result<T, BufferError>;

#[derive(Default, Clone, Debug)]
/// BytesBuffer is a wrapper around a flatbuffer serialized Message with a Bytes payload.
/// It provides methods to create, access, and manipulate binary data.
pub struct BytesBuffer {
    /// Internal Raw Data that holds the serialized flatbuffer containing the Message
    raw_data: BytesType,
}

impl BytesBuffer {
    /// Creates a new BytesBuffer from existing raw serialized data
    ///
    /// # Arguments
    /// * `raw_data` - The serialized flatbuffer data
    pub fn from_raw_data(raw_data: BytesType) -> Self {
        Self { raw_data }
    }

    /// Get a reference to the underlying raw data
    ///
    /// # Returns
    /// A reference to the serialized flatbuffer data
    pub fn raw_data(&self) -> &BytesType {
        &self.raw_data
    }

    /// Consumes the buffer and returns ownership of the raw data
    ///
    /// # Returns
    /// The serialized flatbuffer data
    pub fn take_data(self) -> BytesType {
        self.raw_data
    }

    /// Deserializes the raw data into a Message object
    ///
    /// # Returns
    /// The deserialized Message object
    pub fn message(&self) -> Message {
        flatbuffers::root::<Message>(&self.raw_data).unwrap()
    }

    /// Deserializes the raw data into a Message object with error handling
    ///
    /// # Returns
    /// The deserialized Message object or an error
    pub fn try_message(&self) -> Result<Message> {
        flatbuffers::root::<Message>(&self.raw_data).map_err(|_| BufferError::InvalidData)
    }

    /// Validates that the buffer contains valid data
    ///
    /// # Returns
    /// True if the buffer contains valid flatbuffer data, false otherwise
    pub fn is_valid(&self) -> bool {
        self.try_message().is_ok()
    }

    /// Extracts the Bytes payload from the Message
    ///
    /// # Returns
    /// The deserialized Bytes object, or None if the payload is not a Bytes
    pub fn bytes(&self) -> Option<Bytes> {
        self.message().payload_as_bytes()
    }

    /// Gets the binary data from the payload
    ///
    /// # Returns
    /// The binary data as a slice of u8, or an empty slice if the payload is not valid Bytes
    pub fn value(&self) -> &[u8] {
        self.bytes()
            .and_then(|b| b.data())
            .map_or(&[], |data| data.bytes())
    }
    /// Gets the binary data with proper error handling
    ///
    /// # Returns
    /// The binary data or an error if the payload is invalid
    pub fn try_value(&self) -> Result<&[u8]> {
        let message = self.try_message()?;
        let bytes = message
            .payload_as_bytes()
            .ok_or(BufferError::MissingPayload)?;
        bytes
            .data()
            .map(|data| data.bytes())
            .ok_or(BufferError::InvalidData)
    }

    /// Gets the source identifier from the header
    ///
    /// # Returns
    /// The source identifier, or 0 if no header is present
    pub fn source(&self) -> u16 {
        self.message().header().map_or(0, |h| h.source())
    }

    /// Gets the sequence number from the header
    ///
    /// # Returns
    /// The sequence number, or 0 if no header is present
    pub fn sequence(&self) -> u16 {
        self.message().header().map_or(0, |h| h.sequence())
    }
    /// Gets the timestamp from the header
    ///
    /// # Returns
    /// The timestamp as a formatted string, or None if no header/timestamp is present
    pub fn timestamp(&self) -> Option<String> {
        self.message()
            .header()
            .and_then(|h| h.timestamp())
            .map(|ts| format!("{}:{:09}", ts.secs(), ts.nanos()))
    }
    /// Gets header information as a tuple (source, sequence, timestamp)
    ///
    /// # Returns
    /// A tuple containing (source, sequence, timestamp)
    pub fn header_info(&self) -> (u16, u16, Option<String>) {
        let header = self.message().header();
        match header {
            Some(h) => (
                h.source(),
                h.sequence(),
                h.timestamp()
                    .map(|ts| format!("{}:{:09}", ts.secs(), ts.nanos())),
            ),
            None => (0, 0, None),
        }
    }

    /// Serializes the buffer to a byte vector
    ///
    /// # Returns
    /// A vector containing the serialized flatbuffer data
    pub fn to_vec(&self) -> Vec<u8> {
        self.raw_data.to_vec()
    }

    /// Gets the size of the serialized data
    ///
    /// # Returns
    /// The size in bytes of the serialized flatbuffer data
    pub fn size(&self) -> usize {
        self.raw_data.len()
    }

    /// Creates a builder for constructing BytesBuffer instances
    ///
    /// # Arguments
    /// * `value` - The binary data to serialize
    pub fn builder(value: &[u8]) -> BytesBufferBuilder {
        BytesBufferBuilder::new(value)
    }

    /// Creates a BytesBuffer from a byte slice
    ///
    /// # Arguments
    /// * `data` - The byte slice containing serialized flatbuffer data
    ///
    /// # Returns
    /// A new BytesBuffer or an error if the data is invalid
    pub fn from_slice(data: &[u8]) -> Result<Self> {
        // Vérifier que les données ne sont pas vides
        if data.is_empty() {
            return Err(BufferError::InvalidData);
        }

        // Vérifier si la taille est suffisante pour un flatbuffer valide (minimum 16 octets)
        if data.len() < 16 {
            return Err(BufferError::InvalidData);
        }

        let bytes = BytesType::copy_from_slice(data);
        let buffer = Self::from_raw_data(bytes);

        // Validate the data
        buffer.try_message()?;
        Ok(buffer)
    }
}

/// Builder pattern for creating BytesBuffer instances with optional parameters
pub struct BytesBufferBuilder {
    value: Vec<u8>,
    source: Option<u16>,
    sequence: Option<u16>,
}

impl BytesBufferBuilder {
    /// Creates a new builder for a BytesBuffer
    ///
    /// # Arguments
    /// * `value` - The binary data to serialize
    pub fn new(value: &[u8]) -> Self {
        Self {
            value: value.to_vec(),
            source: None,
            sequence: None,
        }
    }

    /// Creates a new BytesBuffer builder with default source and sequence values
    ///
    /// # Arguments
    /// * `value` - The binary data to serialize
    ///
    /// # Returns
    /// A new BytesBufferBuilder with default values
    pub fn with_default_args(value: &[u8]) -> Self {
        Self::new(value)
    }

    /// Creates a new BytesBuffer builder with a random sequence number
    ///
    /// # Arguments
    /// * `value` - The binary data to serialize
    /// * `source` - Source identifier
    ///
    /// # Returns
    /// A new BytesBufferBuilder with a random sequence
    pub fn with_random_sequence(value: &[u8], source: u16) -> Self {
        Self::new(value).source(source).random_sequence()
    }

    /// Creates a response message with binary data, matching the sequence of the original request.
    /// This is typically used by servers to respond to client requests.
    ///
    /// # Arguments
    /// * `value` - The binary data to include in the response
    /// * `request` - The original request Message to match the sequence number from
    ///
    /// # Returns
    /// A new BytesBuffer containing the response with matching sequence number
    pub fn as_a_response_message_to(mut self, request: Message) -> Self {
        let sequence = request.header().map_or(0, |header| header.sequence());
        self.sequence = Some(sequence);
        self
    }

    /// Sets the source identifier
    ///
    /// # Arguments
    /// * `source` - The source identifier
    pub fn source(mut self, source: u16) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the sequence number
    ///
    /// # Arguments
    /// * `sequence` - The sequence number
    pub fn sequence(mut self, sequence: u16) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// Sets a random sequence number
    pub fn random_sequence(mut self) -> Self {
        let mut rng = rand::thread_rng();
        self.sequence = Some(rng.gen::<u16>());
        self
    }

    /// Builds the BytesBuffer
    pub fn build(self) -> BytesBuffer {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        // Create the bytes payload
        let data_vec = self.value;
        let data = builder.create_vector(&data_vec);
        let bytes_args = BytesArgs { data: Some(data) };
        let bytes = Bytes::create(&mut builder, &bytes_args); // Create header with timestamp
        let timestamp = generate_timestamp();
        let source = self.source.unwrap_or(0);
        let sequence = self.sequence.unwrap_or(0);

        let header_args = HeaderArgs {
            timestamp: Some(&timestamp),
            source,
            sequence,
        };
        let header = Header::create(&mut builder, &header_args);

        // Create the message with the bytes payload
        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::Bytes,
            payload: Some(bytes.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        let raw_data = BytesType::from(builder.finished_data().to_vec());

        BytesBuffer { raw_data }
    }
}

/// Implements the Display trait for better debugging and logging
impl fmt::Display for BytesBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (source, sequence, timestamp) = self.header_info();
        let data_len = self.value().len();
        write!(
            f,
            "BytesBuffer {{ source: {}, sequence: {}, timestamp: {:?}, data_length: {} bytes }}",
            source, sequence, timestamp, data_len
        )
    }
}

/// Implements equality comparison between BytesBuffers
impl PartialEq for BytesBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for BytesBuffer {}

/// Implements equality comparison between BytesBuffer and &[u8]
impl PartialEq<[u8]> for BytesBuffer {
    fn eq(&self, other: &[u8]) -> bool {
        self.value() == other
    }
}

/// Implements the conversion from BytesBuffer to Vec<u8>
impl From<BytesBuffer> for Vec<u8> {
    /// Converts a BytesBuffer to a Vec<u8> value
    ///
    /// # Returns
    /// The Vec<u8> value contained in the buffer
    fn from(buffer: BytesBuffer) -> Self {
        buffer.value().to_vec()
    }
}

/// Implements the conversion from &BytesBuffer to Vec<u8>
impl From<&BytesBuffer> for Vec<u8> {
    /// Converts a reference to BytesBuffer to a Vec<u8> value
    ///
    /// # Returns
    /// The Vec<u8> value contained in the buffer
    fn from(buffer: &BytesBuffer) -> Self {
        buffer.value().to_vec()
    }
}

/// Implements the conversion from Vec<u8>/&[u8] to BytesBuffer
impl From<Vec<u8>> for BytesBuffer {
    /// Creates a new BytesBuffer from a Vec<u8> value
    ///
    /// # Arguments
    /// * `value` - The Vec<u8> value to serialize
    ///
    /// # Returns
    /// A new BytesBuffer containing the serialized value
    fn from(value: Vec<u8>) -> Self {
        BytesBufferBuilder::with_default_args(&value).build()
    }
}

impl From<&[u8]> for BytesBuffer {
    /// Creates a new BytesBuffer from a &[u8] value
    ///
    /// # Arguments
    /// * `value` - The &[u8] value to serialize
    ///
    /// # Returns
    /// A new BytesBuffer containing the serialized value
    fn from(value: &[u8]) -> Self {
        BytesBufferBuilder::with_default_args(value).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_buffer_creation() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = BytesBuffer::from(data.clone());
        assert_eq!(buffer.value(), data.as_slice());
        assert!(buffer.is_valid());
    }    #[test]
    fn test_bytes_buffer_empty() {
        let data: Vec<u8> = vec![];
        let buffer = BytesBuffer::from(data);
        assert!(buffer.value().is_empty());
        assert!(buffer.is_valid());
    }

    #[test]
    fn test_builder_pattern() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = BytesBuffer::builder(&data)
            .source(123)
            .sequence(456)
            .build();

        assert_eq!(buffer.value(), data.as_slice());
        assert_eq!(buffer.source(), 123);
        assert_eq!(buffer.sequence(), 456);
    }

    #[test]
    fn test_builder_with_random_sequence() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer1 = BytesBuffer::builder(&data)
            .source(100)
            .random_sequence()
            .build();

        let buffer2 = BytesBuffer::builder(&data)
            .source(100)
            .random_sequence()
            .build();

        assert_eq!(buffer1.value(), data.as_slice());
        assert_eq!(buffer2.value(), data.as_slice());
        assert_eq!(buffer1.source(), 100);
        assert_eq!(buffer2.source(), 100);
        // Les séquences doivent être différentes (très probablement)
        assert_ne!(buffer1.sequence(), buffer2.sequence());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let data = vec![1, 2, 3, 4, 5];
        let original = BytesBuffer::builder(&data)
            .source(123)
            .sequence(456)
            .build();

        let serialized = original.to_vec();
        let restored = BytesBuffer::from_slice(&serialized).unwrap();

        assert_eq!(restored.value(), data.as_slice());
        assert_eq!(restored.source(), 123);
        assert_eq!(restored.sequence(), 456);
    }

    #[test]
    fn test_response_message() {
        let request_data = vec![10, 20, 30];
        let request = BytesBuffer::builder(&request_data).sequence(789).build();

        let response_data = vec![40, 50, 60];
        let response = BytesBufferBuilder::new(&response_data)
            .as_a_response_message_to(request.message())
            .build();

        assert_eq!(response.value(), response_data.as_slice());
        assert_eq!(response.sequence(), 789);
        assert_eq!(response.source(), 0); // Default source for responses
    }

    #[test]
    fn test_header_info() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = BytesBuffer::builder(&data)
            .source(123)
            .sequence(456)
            .build();

        let (source, sequence, timestamp) = buffer.header_info();
        assert_eq!(source, 123);
        assert_eq!(sequence, 456);
        assert!(timestamp.is_some());
    }    #[test]
    fn test_equality_comparison() {
        let data1 = vec![1, 2, 3, 4, 5];
        let data2 = vec![1, 2, 3, 4, 5]; // Same data
        let data3 = vec![5, 4, 3, 2, 1]; // Different data

        let buffer1 = BytesBuffer::from(&data1[..]);
        let buffer2 = BytesBuffer::from(&data2[..]);
        let buffer3 = BytesBuffer::from(&data3[..]);

        assert_eq!(buffer1, buffer2);
        assert_ne!(buffer1, buffer3);
        assert_eq!(buffer1.value(), data1.as_slice());
    }

    #[test]
    fn test_display_trait() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = BytesBuffer::from(&data[..]);

        let display_output = format!("{}", buffer);
        assert!(display_output.contains("data_length: 5 bytes"));
    }

    #[test]
    fn test_try_value_success() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = BytesBuffer::from(&data[..]);

        let result = buffer.try_value();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data.as_slice());
    }

    #[test]
    fn test_invalid_data_handling() {
        // Create invalid data by just using a few random bytes
        let invalid_data = vec![1, 2, 3, 4];

        let result = BytesBuffer::from_slice(&invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_with_default_args() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = BytesBufferBuilder::with_default_args(&data).build();

        assert_eq!(buffer.value(), data.as_slice());
        assert_eq!(buffer.source(), 0); // Default source
        assert_eq!(buffer.sequence(), 0); // Default sequence
    }

    #[test]
    fn test_with_random_sequence() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = BytesBufferBuilder::with_random_sequence(&data, 42).build();

        assert_eq!(buffer.value(), data.as_slice());
        assert_eq!(buffer.source(), 42);
        assert_ne!(buffer.sequence(), 0); // Random sequence should not be 0
    }

    #[test]
    fn test_buffer_size() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = BytesBuffer::from(&data[..]);

        // The size will be larger than 5 due to the flatbuffer format overhead
        assert!(buffer.size() > 5);
    }

    #[test]
    fn test_raw_data_access() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = BytesBuffer::from(&data[..]);

        let raw_data = buffer.raw_data();
        assert!(!raw_data.is_empty());

        let taken_data = buffer.clone().take_data();
        assert_eq!(taken_data, *raw_data);
    }
}
