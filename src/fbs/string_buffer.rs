use super::common::{generate_timestamp, BufferError};
use super::panduza_generated::panduza::{
    Header, HeaderArgs, Message, MessageArgs, Payload, String, StringArgs,
};
use bytes::Bytes;
use rand::Rng;
use std::fmt;

type Result<T> = std::result::Result<T, BufferError>;

#[derive(Default, Clone, Debug)]
/// StringBuffer is a wrapper around a flatbuffer serialized Message with a String payload.
/// It provides methods to create, access, and manipulate string data.
pub struct StringBuffer {
    /// Internal Raw Data that holds the serialized flatbuffer containing the Message
    raw_data: Bytes,
}

impl StringBuffer {
    /// Creates a new StringBuffer from existing raw serialized data
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

    /// Extracts the String payload from the Message
    ///
    /// # Returns
    /// The deserialized String object, or None if the payload is not a String
    pub fn string(&self) -> Option<String> {
        self.message().payload_as_string()
    }

    /// Gets the string value from the payload
    ///
    /// # Returns
    /// The string value, or an empty string if the payload is not a valid String
    pub fn value(&self) -> &str {
        self.string().and_then(|s| s.value()).unwrap_or("")
    }

    /// Gets the string value with proper error handling
    ///
    /// # Returns
    /// The string value or an error if the payload is invalid
    pub fn try_value(&self) -> Result<&str> {
        let message = self.try_message()?;
        let string = message
            .payload_as_string()
            .ok_or(BufferError::MissingPayload)?;
        string.value().ok_or(BufferError::InvalidData)
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
    pub fn timestamp(&self) -> Option<std::string::String> {
        self.message()
            .header()
            .and_then(|h| h.timestamp())
            .map(|ts| format!("{}:{:09}", ts.secs(), ts.nanos()))
    }

    /// Gets header information as a tuple (source, sequence, timestamp)
    ///
    /// # Returns
    /// A tuple containing (source, sequence, timestamp)
    pub fn header_info(&self) -> (u16, u16, Option<std::string::String>) {
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

    /// Creates a StringBuffer from a byte slice
    ///
    /// # Arguments
    /// * `data` - The byte slice containing serialized flatbuffer data
    ///
    /// # Returns
    /// A new StringBuffer or an error if the data is invalid
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

    /// Vérifie si la charge utile de la chaîne contient une liste blanche (whitelist)
    ///
    /// # Returns
    /// `true` si une whitelist est présente, `false` sinon
    pub fn has_whitelist(&self) -> bool {
        self.string().and_then(|s| s.whitelist()).is_some()
    }

    /// Récupère la liste des valeurs autorisées (whitelist) de la charge utile
    ///
    /// # Returns
    /// Un vecteur de chaînes représentant les valeurs autorisées, ou None s'il n'y a pas de whitelist
    pub fn whitelist_values(&self) -> Option<Vec<std::string::String>> {
        self.string().and_then(|s| s.whitelist()).map(|whitelist| {
            (0..whitelist.len())
                .filter_map(|i| Some(whitelist.get(i)))
                .map(|s| s.to_string())
                .collect()
        })
    }

    /// Creates a builder for constructing StringBuffer instances
    ///
    /// # Arguments
    /// * `value` - The string value to serialize
    pub fn builder<S: AsRef<str>>(value: S) -> StringBufferBuilder {
        StringBufferBuilder::new(value)
    }
}

/// Builder pattern for creating StringBuffer instances with optional parameters
pub struct StringBufferBuilder {
    value: std::string::String,
    source: Option<u16>,
    sequence: Option<u16>,
    whitelist: Option<Vec<std::string::String>>,
}

impl StringBufferBuilder {
    /// Creates a new builder for a StringBuffer
    ///
    /// # Arguments
    /// * `value` - The string value to serialize
    pub fn new<S: AsRef<str>>(value: S) -> Self {
        Self {
            value: value.as_ref().to_string(),
            source: None,
            sequence: None,
            whitelist: None,
        }
    }

    /// Creates a new StringBuffer builder with default source and sequence values
    ///
    /// # Arguments
    /// * `value` - The string value to serialize
    ///
    /// # Returns
    /// A new StringBufferBuilder with default values
    pub fn with_default_args<S: AsRef<str>>(value: S) -> Self {
        Self::new(value)
    }

    /// Creates a new StringBuffer builder with a random sequence number
    ///
    /// # Arguments
    /// * `value` - The string value to serialize
    /// * `source` - Source identifier
    ///
    /// # Returns
    /// A new StringBufferBuilder with a random sequence
    pub fn with_random_sequence<S: AsRef<str>>(value: S, source: u16) -> Self {
        Self::new(value).source(source).random_sequence()
    }

    /// Sets a whitelist of allowed values
    ///
    /// # Arguments
    /// * `whitelist` - A vector of strings representing allowed values
    pub fn whitelist<S: AsRef<str>>(mut self, whitelist: Vec<S>) -> Self {
        self.whitelist = Some(whitelist.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    /// Creates a response message with a string value, matching the sequence of the original request.
    /// This is typically used by servers to respond to client requests.
    ///
    /// # Arguments
    /// * `value` - The string value to include in the response
    /// * `request` - The original request Message to match the sequence number from
    ///
    /// # Returns
    /// A new StringBuffer containing the response with matching sequence number
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

    /// Builds the StringBuffer
    pub fn build(self) -> StringBuffer {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        // Create the string payload
        let value_offset = builder.create_string(&self.value);

        // Create whitelist if provided
        let whitelist_offset = match self.whitelist {
            Some(list) => {
                let items: Vec<_> = list.iter().map(|s| builder.create_string(s)).collect();
                Some(builder.create_vector(&items))
            }
            None => None,
        };

        // Create the string args
        let string_args = StringArgs {
            value: Some(value_offset),
            whitelist: whitelist_offset,
        };

        // Create the string object
        let string = String::create(&mut builder, &string_args);

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

        // Create the message with the string payload
        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::String,
            payload: Some(string.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        let raw_data = Bytes::from(builder.finished_data().to_vec());

        StringBuffer { raw_data }
    }
}

/// Implements the Display trait for better debugging and logging
impl fmt::Display for StringBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (source, sequence, timestamp) = self.header_info();
        write!(
            f,
            "StringBuffer {{ value: \"{}\", source: {}, sequence: {}, timestamp: {:?} }}",
            self.value(),
            source,
            sequence,
            timestamp
        )
    }
}

/// Implements equality comparison between StringBuffers
impl PartialEq for StringBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for StringBuffer {}

/// Implements equality comparison between StringBuffer and &str
impl<T: AsRef<str>> PartialEq<T> for StringBuffer {
    fn eq(&self, other: &T) -> bool {
        self.value() == other.as_ref()
    }
}

/// Implements the conversion from StringBuffer to std::string::String
impl From<StringBuffer> for std::string::String {
    /// Converts a StringBuffer to a std::string::String value
    ///
    /// # Returns
    /// The std::string::String value contained in the buffer
    fn from(buffer: StringBuffer) -> Self {
        buffer.value().to_string()
    }
}

/// Implements the conversion from &StringBuffer to std::string::String
impl From<&StringBuffer> for std::string::String {
    /// Converts a reference to StringBuffer to a std::string::String value
    ///
    /// # Returns
    /// The std::string::String value contained in the buffer
    fn from(buffer: &StringBuffer) -> Self {
        buffer.value().to_string()
    }
}

/// Implements the conversion from String/&str to StringBuffer
impl<T: AsRef<str>> From<T> for StringBuffer {
    /// Creates a new StringBuffer from a string value
    ///
    /// # Arguments
    /// * `value` - The string value to serialize
    ///
    /// # Returns
    /// A new StringBuffer containing the serialized value
    fn from(value: T) -> Self {
        StringBufferBuilder::with_default_args(value).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_buffer_creation() {
        let buffer = StringBuffer::from("Hello world");
        assert_eq!(buffer.value(), "Hello world");
        assert!(buffer.is_valid());
    }

    #[test]
    fn test_string_buffer_empty() {
        let buffer = StringBuffer::from("");
        assert_eq!(buffer.value(), "");
        assert!(buffer.is_valid());
    }

    #[test]
    fn test_builder_pattern() {
        let buffer = StringBuffer::builder("Hello world")
            .source(123)
            .sequence(456)
            .build();

        assert_eq!(buffer.value(), "Hello world");
        assert_eq!(buffer.source(), 123);
        assert_eq!(buffer.sequence(), 456);
    }

    #[test]
    fn test_builder_with_random_sequence() {
        let buffer1 = StringBuffer::builder("Test string")
            .source(100)
            .random_sequence()
            .build();

        let buffer2 = StringBuffer::builder("Test string")
            .source(100)
            .random_sequence()
            .build();

        assert_eq!(buffer1.value(), "Test string");
        assert_eq!(buffer2.value(), "Test string");
        assert_eq!(buffer1.source(), 100);
        assert_eq!(buffer2.source(), 100);
        // Les séquences doivent être différentes (très probablement)
        assert_ne!(buffer1.sequence(), buffer2.sequence());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = StringBuffer::builder("Serialization test")
            .source(42)
            .sequence(789)
            .build();

        let data = original.to_vec();
        let restored = StringBuffer::from_slice(&data).unwrap();

        assert_eq!(original.value(), restored.value());
        assert_eq!(original.source(), restored.source());
        assert_eq!(original.sequence(), restored.sequence());
    }

    #[test]
    fn test_response_message() {
        let request = StringBuffer::builder("Request").sequence(789).build();

        let response = StringBufferBuilder::new("Response")
            .as_a_response_message_to(request.message())
            .build();

        assert_eq!(response.value(), "Response");
        assert_eq!(response.sequence(), 789);
        assert_eq!(response.source(), 0); // Default source for responses
    }

    #[test]
    fn test_header_info() {
        let buffer = StringBuffer::builder("Header test")
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
        let buffer1 = StringBuffer::from("Equal test");
        let buffer2 = StringBuffer::from("Equal test");
        let buffer3 = StringBuffer::from("Not equal");

        assert_eq!(buffer1, buffer2);
        assert_ne!(buffer1, buffer3);
        assert_eq!(buffer1, "Equal test");
        assert_ne!(buffer1, "Not equal");
    }

    #[test]
    fn test_display_trait() {
        let buffer = StringBuffer::builder("Display test")
            .source(42)
            .sequence(123)
            .build();

        let display_string = format!("{}", buffer);
        assert!(display_string.contains("value: \"Display test\""));
        assert!(display_string.contains("source: 42"));
        assert!(display_string.contains("sequence: 123"));
    }

    #[test]
    fn test_try_value_success() {
        let buffer = StringBuffer::from("Success test");
        let result = buffer.try_value();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success test");
    }

    #[test]
    fn test_invalid_data_handling() {
        // Créer des données vraiment corrompues qui ne peuvent pas être un flatbuffer valide
        let invalid_data = vec![1, 2, 3, 4, 5]; // Trop court et sans structure flatbuffer
        let result = StringBuffer::from_slice(&invalid_data);
        assert!(result.is_err());

        match result {
            Err(BufferError::InvalidData) => {}
            _ => panic!("Expected InvalidData error"),
        }

        // Test avec un vecteur vide
        let empty_data = vec![];
        let result = StringBuffer::from_slice(&empty_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_with_default_args() {
        let buffer = StringBufferBuilder::with_default_args("Default test").build();
        assert_eq!(buffer.value(), "Default test");
        assert_eq!(buffer.source(), 0);
        assert_eq!(buffer.sequence(), 0);
    }

    #[test]
    fn test_with_random_sequence() {
        let buffer = StringBufferBuilder::with_random_sequence("Random sequence", 100).build();
        assert_eq!(buffer.value(), "Random sequence");
        assert_eq!(buffer.source(), 100);
        // La séquence doit être différente de 0 (très probablement)
        assert_ne!(buffer.sequence(), 0);
    }

    #[test]
    fn test_buffer_size() {
        let buffer = StringBuffer::from("Size test");
        assert!(buffer.size() > 0);
    }

    #[test]
    fn test_raw_data_access() {
        let buffer = StringBuffer::from("Raw data test");
        let raw_data_ref = buffer.raw_data();
        assert!(!raw_data_ref.is_empty());

        let raw_data_owned = buffer.take_data();
        assert!(!raw_data_owned.is_empty());
    }

    #[test]
    fn test_whitelist() {
        let allowed_values = vec!["option1", "option2", "option3"];
        let buffer = StringBuffer::builder("option1")
            .whitelist(allowed_values)
            .build();

        assert_eq!(buffer.value(), "option1");
        assert!(buffer.is_valid());
    }
}
