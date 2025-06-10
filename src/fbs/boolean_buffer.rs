use super::common::{generate_timestamp, BufferError};
use super::panduza_generated::panduza::{
    Boolean, BooleanArgs, Header, HeaderArgs, Message, MessageArgs, Payload,
};
use bytes::Bytes;
use rand::Rng;
use std::fmt;

type Result<T> = std::result::Result<T, BufferError>;

#[derive(Default, Clone, Debug)]
/// BooleanBuffer is a wrapper around a flatbuffer serialized Message with a Boolean payload.
/// It provides methods to create, access, and manipulate boolean data.
pub struct BooleanBuffer {
    /// Internal Raw Data that holds the serialized flatbuffer containing the Message
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

    /// Extracts the Boolean payload from the Message
    ///
    /// # Returns
    /// The deserialized Boolean object, or None if the payload is not a Boolean
    pub fn boolean(&self) -> Option<Boolean> {
        self.message().payload_as_boolean()
    }
    /// Gets the boolean value from the payload
    ///
    /// # Returns
    /// The boolean value, or false if the payload is not a valid Boolean
    pub fn value(&self) -> bool {
        self.boolean().map_or(false, |b| b.value())
    }
    /// Gets the boolean value with proper error handling
    ///
    /// # Returns
    /// The boolean value or an error if the payload is invalid
    pub fn try_value(&self) -> Result<bool> {
        let message = self.try_message()?;
        let boolean = message
            .payload_as_boolean()
            .ok_or(BufferError::MissingPayload)?;
        Ok(boolean.value())
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
    /// Creates a BooleanBuffer from a byte slice
    ///
    /// # Arguments
    /// * `data` - The byte slice containing serialized flatbuffer data
    ///
    /// # Returns
    /// A new BooleanBuffer or an error if the data is invalid
    pub fn from_slice(data: &[u8]) -> Result<Self> {
        // Vérifier que les données ne sont pas vides
        if data.is_empty() {
            return Err(BufferError::InvalidData);
        }

        // Vérifier si la taille est suffisante pour un flatbuffer valide (minimum 16 octets)
        if data.len() < 16 {
            return Err(BufferError::InvalidData);
        }

        let bytes = Bytes::copy_from_slice(data);
        let buffer = Self::from_raw_data(bytes);

        // Validate the data
        buffer.try_message()?;
        Ok(buffer)
    }

    /// Gets the size of the serialized data
    ///
    /// # Returns
    /// The size in bytes of the serialized flatbuffer data
    pub fn size(&self) -> usize {
        self.raw_data.len()
    }

    /// Creates a builder for constructing BooleanBuffer instances
    ///
    /// # Arguments
    /// * `value` - The boolean value to serialize
    pub fn builder(value: bool) -> BooleanBufferBuilder {
        BooleanBufferBuilder::new(value)
    }
}

/// Builder pattern for creating BooleanBuffer instances with optional parameters
pub struct BooleanBufferBuilder {
    value: bool,
    source: Option<u16>,
    sequence: Option<u16>,
}

impl BooleanBufferBuilder {
    /// Creates a new builder for a BooleanBuffer
    ///
    /// # Arguments
    /// * `value` - The boolean value to serialize
    pub fn new(value: bool) -> Self {
        Self {
            value,
            source: None,
            sequence: None,
        }
    }

    /// Creates a new BooleanBuffer builder with default source and sequence values
    ///
    /// # Arguments
    /// * `value` - The boolean value to serialize
    ///
    /// # Returns
    /// A new BooleanBufferBuilder with default values
    pub fn with_default_args(value: bool) -> Self {
        Self::new(value)
    }

    /// Creates a new BooleanBuffer builder with a random sequence number
    ///
    /// # Arguments
    /// * `value` - The boolean value to serialize
    /// * `source` - Source identifier
    ///
    /// # Returns
    /// A new BooleanBufferBuilder with a random sequence
    pub fn with_random_sequence(value: bool, source: u16) -> Self {
        Self::new(value).source(source).random_sequence()
    }

    /// Creates a response message with a boolean value, matching the sequence of the original request.
    /// This is typically used by servers to respond to client requests.
    ///
    /// # Arguments
    /// * `value` - The boolean value to include in the response
    /// * `request` - The original request Message to match the sequence number from
    ///
    /// # Returns
    /// A new BooleanBuffer containing the response with matching sequence number
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

    /// Builds the BooleanBuffer
    pub fn build(self) -> BooleanBuffer {
        let mut builder = flatbuffers::FlatBufferBuilder::new(); // Create the boolean payload
        let boolean_args = BooleanArgs { value: self.value };
        let boolean = Boolean::create(&mut builder, &boolean_args);

        // Create header with timestamp
        let timestamp = generate_timestamp();
        let source = self.source.unwrap_or(0);
        let sequence = self.sequence.unwrap_or(0);

        let header_args = HeaderArgs {
            timestamp: Some(&timestamp),
            source,
            sequence,
        };
        let header = Header::create(&mut builder, &header_args);

        // Create the message with the boolean payload
        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::Boolean,
            payload: Some(boolean.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        let raw_data = Bytes::from(builder.finished_data().to_vec());

        BooleanBuffer { raw_data }
    }
}

/// Implements the Display trait for better debugging and logging
impl fmt::Display for BooleanBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (source, sequence, timestamp) = self.header_info();
        write!(
            f,
            "BooleanBuffer {{ value: {}, source: {}, sequence: {}, timestamp: {:?} }}",
            self.value(),
            source,
            sequence,
            timestamp
        )
    }
}

/// Implements equality comparison between BooleanBuffers
impl PartialEq for BooleanBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for BooleanBuffer {}

/// Implements equality comparison between BooleanBuffer and bool
impl PartialEq<bool> for BooleanBuffer {
    fn eq(&self, other: &bool) -> bool {
        self.value() == *other
    }
}

/// Implements the conversion from BooleanBuffer to bool
impl From<BooleanBuffer> for bool {
    /// Converts a BooleanBuffer to a boolean value
    ///
    /// # Returns
    /// The boolean value contained in the buffer
    fn from(buffer: BooleanBuffer) -> Self {
        buffer.value()
    }
}

/// Implements the conversion from a reference to BooleanBuffer to bool
impl From<&BooleanBuffer> for bool {
    /// Converts a reference to BooleanBuffer to a boolean value
    ///
    /// # Returns
    /// The boolean value contained in the buffer
    fn from(buffer: &BooleanBuffer) -> Self {
        buffer.value()
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
        BooleanBufferBuilder::with_default_args(value).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_buffer_creation() {
        let buffer = BooleanBuffer::from(true);
        assert_eq!(buffer.value(), true);
        assert!(buffer.is_valid());
    }

    #[test]
    fn test_boolean_buffer_false() {
        let buffer = BooleanBuffer::from(false);
        assert_eq!(buffer.value(), false);
        assert!(buffer.is_valid());
    }

    #[test]
    fn test_builder_pattern() {
        let buffer = BooleanBuffer::builder(true)
            .source(123)
            .sequence(456)
            .build();

        assert_eq!(buffer.value(), true);
        assert_eq!(buffer.source(), 123);
        assert_eq!(buffer.sequence(), 456);
    }

    #[test]
    fn test_builder_with_random_sequence() {
        let buffer1 = BooleanBuffer::builder(true)
            .source(100)
            .random_sequence()
            .build();

        let buffer2 = BooleanBuffer::builder(true)
            .source(100)
            .random_sequence()
            .build();

        assert_eq!(buffer1.value(), true);
        assert_eq!(buffer2.value(), true);
        assert_eq!(buffer1.source(), 100);
        assert_eq!(buffer2.source(), 100);
        // Les séquences doivent être différentes (très probablement)
        assert_ne!(buffer1.sequence(), buffer2.sequence());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = BooleanBuffer::builder(true)
            .source(42)
            .sequence(789)
            .build();

        let data = original.to_vec();
        let restored = BooleanBuffer::from_slice(&data).unwrap();

        assert_eq!(original.value(), restored.value());
        assert_eq!(original.source(), restored.source());
        assert_eq!(original.sequence(), restored.sequence());
    }
    #[test]
    fn test_response_message() {
        let request = BooleanBuffer::builder(false).sequence(789).build();

        let response = BooleanBufferBuilder::new(true)
            .as_a_response_message_to(request.message())
            .build();
        assert_eq!(response.value(), true);
        assert_eq!(response.sequence(), 789);
        assert_eq!(response.source(), 0); // Default source for responses
    }

    #[test]
    fn test_header_info() {
        let buffer = BooleanBuffer::builder(true)
            .source(123)
            .sequence(456)
            .build();

        let (source, sequence, timestamp) = buffer.header_info();
        assert_eq!(source, 123);
        assert_eq!(sequence, 456);
        assert!(timestamp.is_some());
    }

    #[test]
    fn test_equality_comparison() {
        let buffer1 = BooleanBuffer::from(true);
        let buffer2 = BooleanBuffer::from(true);
        let buffer3 = BooleanBuffer::from(false);

        assert_eq!(buffer1, buffer2);
        assert_ne!(buffer1, buffer3);
        assert_eq!(buffer1, true);
        assert_eq!(buffer3, false);
    }

    #[test]
    fn test_display_trait() {
        let buffer = BooleanBuffer::builder(true)
            .source(42)
            .sequence(123)
            .build();

        let display_string = format!("{}", buffer);
        assert!(display_string.contains("value: true"));
        assert!(display_string.contains("source: 42"));
        assert!(display_string.contains("sequence: 123"));
    }

    #[test]
    fn test_try_value_success() {
        let buffer = BooleanBuffer::from(true);
        let result = buffer.try_value();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
    #[test]
    fn test_invalid_data_handling() {
        // Créer des données vraiment corrompues qui ne peuvent pas être un flatbuffer valide
        let invalid_data = vec![1, 2, 3, 4, 5]; // Trop court et sans structure flatbuffer
        let result = BooleanBuffer::from_slice(&invalid_data);
        assert!(result.is_err());

        match result {
            Err(BufferError::InvalidData) => {}
            _ => panic!("Expected InvalidData error"),
        }

        // Test avec un vecteur vide
        let empty_data = vec![];
        let result = BooleanBuffer::from_slice(&empty_data);
        assert!(result.is_err());
    }
    #[test]
    fn test_with_default_args() {
        let buffer = BooleanBufferBuilder::with_default_args(true).build();
        assert_eq!(buffer.value(), true);
        assert_eq!(buffer.source(), 0);
        assert_eq!(buffer.sequence(), 0);
    }

    #[test]
    fn test_with_random_sequence() {
        let buffer = BooleanBufferBuilder::with_random_sequence(false, 100).build();
        assert_eq!(buffer.value(), false);
        assert_eq!(buffer.source(), 100);
        // La séquence doit être différente de 0 (très probablement)
        assert_ne!(buffer.sequence(), 0);
    }

    #[test]
    fn test_buffer_size() {
        let buffer = BooleanBuffer::from(true);
        assert!(buffer.size() > 0);
    }

    #[test]
    fn test_raw_data_access() {
        let buffer = BooleanBuffer::from(true);
        let raw_data_ref = buffer.raw_data();
        assert!(!raw_data_ref.is_empty());

        let raw_data_owned = buffer.take_data();
        assert!(!raw_data_owned.is_empty());
    }
}
