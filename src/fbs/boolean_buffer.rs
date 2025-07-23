use crate::fbs::generate_timestamp;
use crate::fbs::panduza_generated::panduza::Boolean;
use crate::fbs::panduza_generated::panduza::BooleanArgs;
use crate::fbs::panduza_generated::panduza::Header;
use crate::fbs::panduza_generated::panduza::HeaderArgs;
use crate::fbs::panduza_generated::panduza::Message;
use crate::fbs::panduza_generated::panduza::MessageArgs;
use crate::fbs::panduza_generated::panduza::Payload;
use crate::fbs::PzaBuffer;
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// A builder for creating a BooleanBuffer
///
#[derive(Default, Clone, Debug, PartialEq)]
pub struct BooleanBufferBuilder {
    /// The source of the message, typically a u16 identifier.
    ///
    source: Option<u16>,

    /// The sequence number of the message, typically a u16 identifier.
    ///
    sequence: Option<u16>,

    /// The value of the boolean message, if present.
    ///
    value: Option<bool>,
}

impl BooleanBufferBuilder {
    // ------------------------------------------------------------------------

    /// Prepare a buffer as an answer to another BooleanBuffer.
    ///
    pub fn as_answer_to(mut self, other: &BooleanBuffer) -> Self {
        if let Some(sequence) = other.sequence() {
            self = self.with_sequence(sequence);
        }
        self
    }

    // ------------------------------------------------------------------------

    /// Set the source of the message.
    ///
    pub fn with_source(mut self, source: u16) -> Self {
        self.source = Some(source);
        self
    }

    // ------------------------------------------------------------------------

    /// Set the sequence number of the message.
    ///
    pub fn with_sequence(mut self, sequence: u16) -> Self {
        self.sequence = Some(sequence);
        self
    }

    // ------------------------------------------------------------------------

    /// Set a random sequence number for the message.
    ///
    pub fn with_random_sequence(mut self) -> Self {
        let mut rng = rand::thread_rng();
        self.sequence = Some(rng.gen());
        self
    }

    // ------------------------------------------------------------------------

    /// Set the value of the boolean message.
    ///
    pub fn with_value(mut self, value: bool) -> Self {
        self.value = Some(value);
        self
    }

    // ------------------------------------------------------------------------

    /// Build the BooleanBuffer with the provided parameters.
    ///
    pub fn build(self) -> Result<BooleanBuffer, String> {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let timestamp = generate_timestamp();

        let value = self.value.ok_or("value not provided".to_string())?;

        let boolean_args = BooleanArgs { value };
        let boolean = Boolean::create(&mut builder, &boolean_args);

        let header_source = self
            .source
            .ok_or("header_source not provided".to_string())?;
        let sequence = self.sequence.ok_or("sequence not provided".to_string())?;

        let header_args = HeaderArgs {
            timestamp: Some(&timestamp),
            source: header_source,
            sequence,
        };
        let header = Header::create(&mut builder, &header_args);

        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::Boolean,
            payload: Some(boolean.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        Ok(BooleanBuffer {
            raw_data: Bytes::from(builder.finished_data().to_vec()),
        })
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// A buffer that contains a boolean value, along with metadata such as source and sequence.
///
#[derive(Default, Clone, Debug, PartialEq)]
pub struct BooleanBuffer {
    /// The raw data of the buffer, serialized as bytes.
    ///
    raw_data: Bytes,
}

impl PzaBuffer for BooleanBuffer {
    // ------------------------------------------------------------------------

    fn from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        BooleanBuffer { raw_data: bytes }
    }

    // ------------------------------------------------------------------------

    fn to_zbytes(self) -> ZBytes {
        ZBytes::from(self.raw_data)
    }

    // ------------------------------------------------------------------------

    fn size(&self) -> usize {
        self.raw_data.len()
    }

    // ------------------------------------------------------------------------

    fn source(&self) -> Option<u16> {
        let msg = self.as_message();
        msg.header().map(|h| h.source())
    }

    // ------------------------------------------------------------------------

    fn sequence(&self) -> Option<u16> {
        let msg = self.as_message();
        msg.header().map(|h| h.sequence())
    }

    // ------------------------------------------------------------------------

    fn as_message(&self) -> Message {
        flatbuffers::root::<Message>(&self.raw_data)
            .expect("BOOLEAN: Failed to deserialize Message from raw_data")
    }

    // ------------------------------------------------------------------------

    fn has_same_message_value<B: PzaBuffer>(&self, other_buffer: &B) -> bool {
        let self_msg = self.as_message();
        let other_msg = other_buffer.as_message();

        if self_msg.payload_type() != other_msg.payload_type() {
            return false;
        }

        if let (Some(self_bool), Some(other_bool)) = (
            self_msg.payload_as_boolean(),
            other_msg.payload_as_boolean(),
        ) {
            self_bool.value() == other_bool.value()
        } else {
            false
        }
    }

    // ------------------------------------------------------------------------
}

impl BooleanBuffer {
    // ------------------------------------------------------------------------

    /// Create a new BooleanBufferBuilder instance.
    ///
    pub fn builder() -> BooleanBufferBuilder {
        BooleanBufferBuilder::default()
    }

    // ------------------------------------------------------------------------

    /// Get the raw data of the buffer.
    ///
    pub fn value(&self) -> Option<bool> {
        self.as_message().payload_as_boolean().map(|b| b.value())
    }

    // ------------------------------------------------------------------------
}
