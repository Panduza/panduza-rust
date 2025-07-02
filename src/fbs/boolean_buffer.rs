use super::common::generate_timestamp;
use super::generic::PanduzaBuffer;
use super::panduza_generated::panduza::{
    Boolean, BooleanArgs, Header, HeaderArgs, Message, MessageArgs, Payload,
};
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
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

    ///
    fn is_builded(&self) -> bool {
        self.raw_data.is_some()
    }

    ///
    fn sequence(&self) -> u16 {
        self.sequence
            .expect("Sequence must be set before accessing it")
    }

    fn to_zbytes(self) -> ZBytes {
        ZBytes::from(
            self.raw_data
                .expect("Raw data must be set before converting to ZBytes"),
        )
    }

    fn as_message(&self) -> Message {
        let data = self
            .raw_data
            .as_ref()
            .expect("Buffer must be built to access the message");
        flatbuffers::root::<Message>(data).expect("Failed to deserialize Message from raw_data")
    }

    ///
    ///
    fn has_value_equal_to_message_value(&self, message: &Message) -> bool {
        if let Some(payload) = message.payload_as_boolean() {
            if let Some(value) = self.value {
                return payload.value() == value;
            }
        }
        false
    }
}

impl From<bool> for BooleanBuffer {
    fn from(value: bool) -> Self {
        let mut obj = Self::new();
        obj.value = Some(value);
        obj
    }
}

impl From<BooleanBuffer> for bool {
    fn from(buffer: BooleanBuffer) -> Self {
        match buffer.value {
            Some(v) => v,
            None => buffer.value(),
        }
    }
}

impl BooleanBuffer {
    /// Extracts the Boolean payload from the Message
    ///
    /// # Returns
    /// The deserialized Boolean object, or None if the payload is not a Boolean
    pub fn boolean(&self) -> Option<Boolean> {
        self.as_message().payload_as_boolean()
    }

    /// Gets the boolean value from the payload
    ///
    /// # Returns
    /// The boolean value, or false if the payload is not a valid Boolean
    pub fn value(&self) -> bool {
        self.boolean().map_or(false, |b| b.value())
    }
}
