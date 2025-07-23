use crate::fbs::generate_timestamp;
use crate::fbs::{
    panduza_generated::panduza::{
        Bytes as FbBytes, BytesArgs, Header, HeaderArgs, Message, MessageArgs, Payload,
    },
    PzaBuffer,
};
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct BytesBufferBuilder {
    value: Option<Bytes>,
    source: Option<u16>,
    sequence: Option<u16>,
}

impl BytesBufferBuilder {
    pub fn with_value<T: Into<Bytes>>(mut self, value: T) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_source(mut self, source: u16) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_sequence(mut self, sequence: u16) -> Self {
        self.sequence = Some(sequence);
        self
    }

    pub fn with_random_sequence(mut self) -> Self {
        let mut rng = rand::thread_rng();
        self.sequence = Some(rng.gen());
        self
    }

    pub fn build(self) -> Result<BytesBuffer, String> {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let timestamp = generate_timestamp();

        let value = self.value.ok_or("value not provided".to_string())?;
        let data_vec = builder.create_vector(&value);
        let bytes_args = BytesArgs {
            data: Some(data_vec),
        };
        let fb_bytes = FbBytes::create(&mut builder, &bytes_args);

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
            payload_type: Payload::Bytes,
            payload: Some(fb_bytes.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        Ok(BytesBuffer {
            raw_data: Bytes::from(builder.finished_data().to_vec()),
        })
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct BytesBuffer {
    raw_data: Bytes,
}

impl PzaBuffer for BytesBuffer {
    fn from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        BytesBuffer { raw_data: bytes }
    }

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

    fn sequence(&self) -> Option<u16> {
        let msg = self.as_message();
        msg.header().map(|h| h.sequence())
    }

    fn as_message(&self) -> Message {
        flatbuffers::root::<Message>(&self.raw_data)
            .expect("BYTES: Failed to deserialize Message from raw_data")
    }

    fn has_same_message_value<B: PzaBuffer>(&self, other_buffer: &B) -> bool {
        let self_msg = self.as_message();
        let other_msg = other_buffer.as_message();

        if self_msg.payload_type() != other_msg.payload_type() {
            return false;
        }

        if let (Some(self_bytes), Some(other_bytes)) =
            (self_msg.payload_as_bytes(), other_msg.payload_as_bytes())
        {
            match (self_bytes.data(), other_bytes.data()) {
                (Some(self_data), Some(other_data)) => self_data.bytes() == other_data.bytes(),
                (None, None) => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

impl BytesBuffer {
    pub fn builder() -> BytesBufferBuilder {
        BytesBufferBuilder::default()
    }

    /// Retourne la valeur du buffer sous forme de slice d'octets, si présente.
    pub fn value(&self) -> Option<Bytes> {
        self.as_message()
            .payload_as_bytes()
            .and_then(|b| b.data().map(|v| Bytes::copy_from_slice(v.bytes())))
    }
}
