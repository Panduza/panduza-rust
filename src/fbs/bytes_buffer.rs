use super::common::generate_timestamp;
use super::generic::PanduzaBuffer;
use super::panduza_generated::panduza::{
    Bytes as FbsBytes, BytesArgs, Header, HeaderArgs, Message, MessageArgs, Payload,
};
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
/// BytesBuffer is a wrapper around a flatbuffer serialized Message with a Bytes payload.
/// It provides methods to create, access, and manipulate byte array data.
pub struct BytesBuffer {
    ///
    value: Option<Bytes>,

    ///
    source: Option<u16>,

    ///
    sequence: Option<u16>,

    ///
    raw_data: Option<Bytes>,
}

/// Implementation of GenericBuffer for BytesBuffer
impl PanduzaBuffer for BytesBuffer {
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
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        // Create the bytes payload
        let data_vec = self.value.as_ref().ok_or("value not provided".to_string())?;
        let data = builder.create_vector(data_vec);
        let bytes_args = BytesArgs { data: Some(data) };
        let bytes = FbsBytes::create(&mut builder, &bytes_args);

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

        // Create the message with the bytes payload
        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::Bytes,
            payload: Some(bytes.as_union_value()),
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

    fn has_value_equal_to_message_value(&self, message: &Message) -> bool {
        if let Some(payload) = message.payload_as_bytes() {
            if let Some(ref value) = self.value {
                return payload.data().map_or(false, |d| d.bytes() == &value[..]);
            }
        }
        false
    }
}

impl From<Bytes> for BytesBuffer {
    fn from(value: Bytes) -> Self {
        let mut obj = Self::new();
        obj.value = Some(value);
        obj
    }
}

impl From<BytesBuffer> for Bytes {
    fn from(buffer: BytesBuffer) -> Self {
        match buffer.value {
            Some(v) => v,
            None => buffer.value(),
        }
    }
}

impl BytesBuffer {
    /// Extracts the Bytes payload from the Message
    pub fn bytes(&self) -> Option<FbsBytes> {
        self.as_message().payload_as_bytes()
    }

    /// Gets the byte vector from the payload
    pub fn value(&self) -> Bytes {
        self.bytes()
            .and_then(|b| b.data())
            .map(|data| Bytes::copy_from_slice(data.bytes()))
            .unwrap_or_else(|| self.value.clone().unwrap_or_default())
    }
}
