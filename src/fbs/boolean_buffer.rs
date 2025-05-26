use super::common::generate_timestamp;
use super::panduza_generated::panduza::{
    Boolean, BooleanArgs, Header, HeaderArgs, Message, MessageArgs, Payload,
};
use bytes::Bytes;
use rand::Rng;

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

    /// Creates a new BooleanBuffer from a boolean value
    ///
    /// # Arguments
    /// * `value` - The boolean value to serialize
    /// * `source` - Optional source identifier, defaults to 0 if not specified
    /// * `sequence` - Optional sequence number, defaults to 0 if not specified
    ///
    /// # Returns
    /// A new BooleanBuffer containing the serialized value wrapped in a Message
    pub fn from_value(value: bool, source: u16, sequence: u16) -> Self {
        let mut builder = flatbuffers::FlatBufferBuilder::new(); // Create the boolean payload
        let boolean_args = BooleanArgs { value: value };
        let boolean = Boolean::create(&mut builder, &boolean_args);

        // Create header with timestamp
        let timestamp = generate_timestamp();
        let header_args = HeaderArgs {
            timestamp: Some(&timestamp),
            source: source,
            sequence: sequence,
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

        Self { raw_data: raw_data }
    }

    /// Deserializes the raw data into a Message object
    ///
    /// # Returns
    /// The deserialized Message object
    pub fn message(&self) -> Message {
        flatbuffers::root::<Message>(&self.raw_data).unwrap()
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

    /// Creates a new BooleanBuffer from a boolean value with default source and sequence values
    ///
    /// # Arguments
    /// * `value` - The boolean value to serialize
    ///
    /// # Returns
    /// A new BooleanBuffer containing the serialized value wrapped in a Message
    pub fn with_default_args(value: bool) -> Self {
        Self::from_value(value, 0, 0)
    }

    /// Creates a new BooleanBuffer from a boolean value with a random sequence number
    /// Used by client to generate unique messages
    ///
    /// # Arguments
    /// * `value` - The boolean value to serialize
    /// * `source` - Source identifier
    ///
    /// # Returns
    /// A new BooleanBuffer containing the serialized value wrapped in a Message with a random sequence number
    pub fn with_random_sequence(value: bool, source: u16) -> Self {
        let mut rng = rand::thread_rng();
        let sequence = rng.gen::<u16>();
        Self::from_value(value, source, sequence)
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
    pub fn as_response_message(value: bool, request: Message) -> Self {
        let sequence = request.header().map_or(0, |header| header.sequence());
        Self::from_value(value, 0, sequence)
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
        BooleanBuffer::from_value(value, 0, 0)
    }
}
