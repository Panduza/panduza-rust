use crate::PanduzaBufferBuilder;

use super::common::{generate_timestamp, BufferError};
use super::generic::PanduzaBuffer;
use super::panduza_generated::panduza::{
    Boolean, BooleanArgs, Header, HeaderArgs, Message, MessageArgs, Payload,
};
use bytes::Bytes;
use rand::Rng;
use std::fmt;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug)]
/// BooleanBuffer is a wrapper around a flatbuffer serialized Message with a Boolean payload.
/// It provides methods to create, access, and manipulate boolean data.
pub struct BooleanBuffer {
    ///
    value: Option<bool>,

    ///
    source: Option<u16>,

    ///
    sequence: Option<u16>,

    ///
    raw_data: Option<Bytes>,
}

/// Implementation of GenericBuffer for BooleanBuffer
///
impl PanduzaBuffer for BooleanBuffer {
    fn new() -> Self {
        Self {
            value: None,
            source: None,
            sequence: None,
            raw_data: None,
        }
    }

    fn with_value<T>(self, value: T) -> Self
    where
        T: Into<Self>,
    {
        Self {
            value: value.into().value,
            ..self
        }
    }

    fn with_source(self, source: u16) -> Self {
        Self {
            source: Some(source),
            ..self
        }
    }

    fn with_sequence(self, sequence: u16) -> Self {
        Self {
            sequence: Some(sequence),
            ..self
        }
    }

    fn with_random_sequence(self) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            sequence: Some(rng.gen::<u16>()),
            ..self
        }
    }

    fn with_sequence_as_a_reply_to<T>(self, command: T) -> Self
    where
        T: Into<Self>,
    {
        let command = command.into();
        Self {
            sequence: command.sequence,
            ..self
        }
    }

    fn build(self) -> Result<Self, String> {
        //
        let mut builder = flatbuffers::FlatBufferBuilder::new(); // Create the boolean payload
        let boolean_args = BooleanArgs {
            value: self.value.ok_or("value not provided".to_string())?,
        };
        let boolean = Boolean::create(&mut builder, &boolean_args);

        // Create header with timestamp
        let timestamp = generate_timestamp();
        let source = self.source.ok_or("source not provided".to_string())?;
        let sequence = self.sequence.ok_or("sequence not provided".to_string())?;

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

        Ok(Self {
            raw_data: Some(Bytes::from(builder.finished_data().to_vec())),
            value: self.value,
            source: self.source,
            sequence: self.sequence,
        })
    }

    fn build_from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        Self {
            raw_data: Some(bytes),
            value: None,
            source: None,
            sequence: None,
        }
    }

    fn is_builded(&self) -> bool {
        self.raw_data.is_some()
    }

    fn to_zbytes(self) -> ZBytes {
        ZBytes::from(
            self.raw_data
                .expect("Raw data must be set before converting to ZBytes"),
        )
    }
}

impl From<bool> for BooleanBuffer {
    fn from(value: bool) -> Self {
        let mut obj = Self::new();
        obj.value = Some(value);
        obj
    }
}

// impl BooleanBuffer {
//     /// Creates a new BooleanBuffer from existing raw serialized data
//     ///
//     /// # Arguments
//     /// * `raw_data` - The serialized flatbuffer data
//     pub fn from_raw_data(raw_data: Bytes) -> Self {
//         Self { raw_data: raw_data }
//     }

//     /// Get a reference to the underlying raw data
//     ///
//     /// # Returns
//     /// A reference to the serialized flatbuffer data
//     pub fn raw_data(&self) -> &Bytes {
//         &self.raw_data
//     }

//     /// Consumes the buffer and returns ownership of the raw data
//     ///
//     /// # Returns
//     /// The serialized flatbuffer data
//     pub fn take_data(self) -> Bytes {
//         self.raw_data
//     }

//     /// Consumes the buffer and returns its data as ZBytes
//     ///
//     /// # Returns
//     /// The serialized flatbuffer data as ZBytes
//     pub fn take_as_zbytes(self) -> ZBytes {
//         // Convert the Bytes to ZBytes and transfer ownership
//         ZBytes::from(self.raw_data)
//     }

//     /// Get the data as ZBytes without consuming the buffer
//     ///
//     /// # Returns
//     /// A ZBytes containing a copy of the serialized data
//     pub fn to_zbytes(&self) -> ZBytes {
//         // Create a ZBytes from a copy of raw_data
//         ZBytes::from(self.raw_data.clone())
//     }

//     /// Creates a BooleanBuffer from ZBytes
//     ///
//     /// # Arguments
//     /// * `zbytes` - The ZBytes containing serialized flatbuffer data
//     ///
//     /// # Returns
//     /// A new BooleanBuffer instance
//     pub fn from_zbytes(zbytes: ZBytes) -> Self {
//         let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
//         Self::from_raw_data(bytes)
//     }

//     /// Deserializes the raw data into a Message object
//     ///
//     /// # Returns
//     /// The deserialized Message object
//     pub fn message(&self) -> Message {
//         flatbuffers::root::<Message>(&self.raw_data).unwrap()
//     }
//     /// Deserializes the raw data into a Message object with error handling
//     ///
//     /// # Returns
//     /// The deserialized Message object or an error
//     pub fn try_message(&self) -> Result<Message> {
//         flatbuffers::root::<Message>(&self.raw_data).map_err(|_| BufferError::InvalidData)
//     }

//     /// Validates that the buffer contains valid data
//     ///
//     /// # Returns
//     /// True if the buffer contains valid flatbuffer data, false otherwise
//     pub fn is_valid(&self) -> bool {
//         self.try_message().is_ok()
//     }

//     /// Extracts the Boolean payload from the Message
//     ///
//     /// # Returns
//     /// The deserialized Boolean object, or None if the payload is not a Boolean
//     pub fn boolean(&self) -> Option<Boolean> {
//         self.message().payload_as_boolean()
//     }
//     /// Gets the boolean value from the payload
//     ///
//     /// # Returns
//     /// The boolean value, or false if the payload is not a valid Boolean
//     pub fn value(&self) -> bool {
//         self.boolean().map_or(false, |b| b.value())
//     }
//     /// Gets the boolean value with proper error handling
//     ///
//     /// # Returns
//     /// The boolean value or an error if the payload is invalid
//     pub fn try_value(&self) -> Result<bool> {
//         let message = self.try_message()?;
//         let boolean = message
//             .payload_as_boolean()
//             .ok_or(BufferError::MissingPayload)?;
//         Ok(boolean.value())
//     }

//     /// Gets the source identifier from the header
//     ///
//     /// # Returns
//     /// The source identifier, or 0 if no header is present
//     pub fn source(&self) -> u16 {
//         self.message().header().map_or(0, |h| h.source())
//     }

//     /// Gets the sequence number from the header
//     ///
//     /// # Returns
//     /// The sequence number, or 0 if no header is present
//     pub fn sequence(&self) -> u16 {
//         self.message().header().map_or(0, |h| h.sequence())
//     }
//     /// Gets the timestamp from the header
//     ///
//     /// # Returns
//     /// The timestamp as a formatted string, or None if no header/timestamp is present
//     pub fn timestamp(&self) -> Option<String> {
//         self.message()
//             .header()
//             .and_then(|h| h.timestamp())
//             .map(|ts| format!("{}:{:09}", ts.secs(), ts.nanos()))
//     }

//     /// Gets header information as a tuple (source, sequence, timestamp)
//     ///
//     /// # Returns
//     /// A tuple containing (source, sequence, timestamp)
//     pub fn header_info(&self) -> (u16, u16, Option<String>) {
//         let header = self.message().header();
//         match header {
//             Some(h) => (
//                 h.source(),
//                 h.sequence(),
//                 h.timestamp()
//                     .map(|ts| format!("{}:{:09}", ts.secs(), ts.nanos())),
//             ),
//             None => (0, 0, None),
//         }
//     }

//     /// Serializes the buffer to a byte vector
//     ///
//     /// # Returns
//     /// A vector containing the serialized flatbuffer data
//     pub fn to_vec(&self) -> Vec<u8> {
//         self.raw_data.to_vec()
//     }
//     /// Creates a BooleanBuffer from a byte slice
//     ///
//     /// # Arguments
//     /// * `data` - The byte slice containing serialized flatbuffer data
//     ///
//     /// # Returns
//     /// A new BooleanBuffer or an error if the data is invalid
//     pub fn from_slice(data: &[u8]) -> Result<Self> {
//         // Vérifier que les données ne sont pas vides
//         if data.is_empty() {
//             return Err(BufferError::InvalidData);
//         }

//         // Vérifier si la taille est suffisante pour un flatbuffer valide (minimum 16 octets)
//         if data.len() < 16 {
//             return Err(BufferError::InvalidData);
//         }

//         let bytes = Bytes::copy_from_slice(data);
//         let buffer = Self::from_raw_data(bytes);

//         // Validate the data
//         buffer.try_message()?;
//         Ok(buffer)
//     }

//     /// Gets the size of the serialized data
//     ///
//     /// # Returns
//     /// The size in bytes of the serialized flatbuffer data
//     pub fn size(&self) -> usize {
//         self.raw_data.len()
//     }

//     /// Creates a builder for constructing BooleanBuffer instances
//     ///
//     /// # Arguments
//     /// * `value` - The boolean value to serialize
//     pub fn builder(value: bool) -> BooleanBufferBuilder {
//         BooleanBufferBuilder::new(value)
//     }
// }

// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// // ----------------------------------------------------------------------------

// /// Builder pattern for creating BooleanBuffer instances with optional parameters
// pub struct BooleanBufferBuilder {
//     value: bool,
//     source: Option<u16>,
//     sequence: Option<u16>,
// }

// impl PanduzaBufferBuilder<BooleanBuffer> for BooleanBufferBuilder {
//     /// Creates a new instance of the builder
//     ///
//     /// # Returns
//     /// A new BooleanBufferBuilder with default values
//     fn new() -> Self {
//         Self {
//             value: false,
//             source: None,
//             sequence: None,
//         }
//     }

//     /// Sets the boolean value to serialize
//     ///
//     /// # Arguments
//     /// * `value` - The boolean value to serialize
//     fn with_value<T>(mut self, value: T) -> Self
//     where
//         T: Into<bool>,
//     {
//         self.value = value.into();
//         self
//     }

//     /// Sets the source identifier
//     fn with_source(mut self, source: u16) -> Self {
//         self.source = Some(source);
//         self
//     }

//     /// Sets the sequence number
//     fn with_sequence(mut self, sequence: u16) -> Self {
//         self.sequence = Some(sequence);
//         self
//     }

//     /// Sets the sequence number from a request message (reply pattern)
//     fn as_a_reply_to(mut self, request: Message) -> Self {
//         let sequence = request.header().map_or(0, |header| header.sequence());
//         self.sequence = Some(sequence);
//         self
//     }

//     /// Builds the BooleanBuffer
//     fn build(self) -> BooleanBuffer {
//         BooleanBufferBuilder::build(self)
//     }
// }

// impl BooleanBufferBuilder {
//     /// Creates a new builder for a BooleanBuffer
//     ///
//     /// # Arguments
//     /// * `value` - The boolean value to serialize
//     pub fn new(value: bool) -> Self {
//         Self {
//             value,
//             source: None,
//             sequence: None,
//         }
//     }

//     /// Creates a new BooleanBuffer builder with default source and sequence values
//     ///
//     /// # Arguments
//     /// * `value` - The boolean value to serialize
//     ///
//     /// # Returns
//     /// A new BooleanBufferBuilder with default values
//     pub fn with_default_args(value: bool) -> Self {
//         Self::new(value)
//     }

//     /// Creates a new BooleanBuffer builder with a random sequence number
//     ///
//     /// # Arguments
//     /// * `value` - The boolean value to serialize
//     /// * `source` - Source identifier
//     ///
//     /// # Returns
//     /// A new BooleanBufferBuilder with a random sequence
//     pub fn with_random_sequence(value: bool, source: u16) -> Self {
//         Self::new(value).source(source).random_sequence()
//     }

//     /// Creates a response message with a boolean value, matching the sequence of the original request.
//     /// This is typically used by servers to respond to client requests.
//     ///
//     /// # Arguments
//     /// * `value` - The boolean value to include in the response
//     /// * `request` - The original request Message to match the sequence number from
//     ///
//     /// # Returns
//     /// A new BooleanBuffer containing the response with matching sequence number
//     pub fn as_a_response_message_to(mut self, request: Message) -> Self {
//         let sequence = request.header().map_or(0, |header| header.sequence());
//         self.sequence = Some(sequence);
//         self
//     }

//     /// Sets the source identifier
//     ///
//     /// # Arguments
//     /// * `source` - The source identifier
//     pub fn source(mut self, source: u16) -> Self {
//         self.source = Some(source);
//         self
//     }

//     /// Sets the sequence number
//     ///
//     /// # Arguments
//     /// * `sequence` - The sequence number
//     pub fn sequence(mut self, sequence: u16) -> Self {
//         self.sequence = Some(sequence);
//         self
//     }

//     /// Sets a random sequence number
//     pub fn random_sequence(mut self) -> Self {
//         let mut rng = rand::thread_rng();
//         self.sequence = Some(rng.gen::<u16>());
//         self
//     }

//     /// Builds the BooleanBuffer
//     pub fn build(self) -> BooleanBuffer {
//         let mut builder = flatbuffers::FlatBufferBuilder::new(); // Create the boolean payload
//         let boolean_args = BooleanArgs { value: self.value };
//         let boolean = Boolean::create(&mut builder, &boolean_args);

//         // Create header with timestamp
//         let timestamp = generate_timestamp();
//         let source = self.source.unwrap_or(0);
//         let sequence = self.sequence.unwrap_or(0);

//         let header_args = HeaderArgs {
//             timestamp: Some(&timestamp),
//             source,
//             sequence,
//         };
//         let header = Header::create(&mut builder, &header_args);

//         // Create the message with the boolean payload
//         let message_args = MessageArgs {
//             header: Some(header),
//             payload_type: Payload::Boolean,
//             payload: Some(boolean.as_union_value()),
//         };
//         let message = Message::create(&mut builder, &message_args);

//         builder.finish(message, None);

//         let raw_data = Bytes::from(builder.finished_data().to_vec());

//         BooleanBuffer { raw_data }
//     }
// }
