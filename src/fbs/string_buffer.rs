use crate::fbs::generate_timestamp;
use crate::fbs::{
    panduza_generated::panduza::{
        Header, HeaderArgs, Message, MessageArgs, Payload, String as FbString, StringArgs,
    },
    PzaBuffer,
};
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StringBufferBuilder {
    value: Option<String>,
    source: Option<u16>,
    sequence: Option<u16>,
}

impl StringBufferBuilder {
    pub fn with_value<S: Into<String>>(mut self, value: S) -> Self {
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

    pub fn build(self) -> Result<StringBuffer, String> {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let timestamp = generate_timestamp();

        let value = self
            .value
            .as_deref()
            .ok_or("value not provided".to_string())?;
        let value_fb = builder.create_string(value);

        // whitelist est optionnel, on ne le gère pas ici (None)
        let string_args = StringArgs {
            value: Some(value_fb),
            whitelist: None,
        };
        let fb_string = FbString::create(&mut builder, &string_args);

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
            payload_type: Payload::String,
            payload: Some(fb_string.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        Ok(StringBuffer {
            raw_data: Bytes::from(builder.finished_data().to_vec()),
        })
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StringBuffer {
    raw_data: Bytes,
}

impl PzaBuffer for StringBuffer {
    fn from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        StringBuffer { raw_data: bytes }
    }

    fn to_zbytes(self) -> ZBytes {
        ZBytes::from(self.raw_data)
    }

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
            .expect("Failed to deserialize Message from raw_data")
    }

    fn has_same_message_value<B: PzaBuffer>(&self, other_buffer: &B) -> bool {
        let self_msg = self.as_message();
        let other_msg = other_buffer.as_message();

        if self_msg.payload_type() != other_msg.payload_type() {
            return false;
        }

        if let (Some(self_str), Some(other_str)) =
            (self_msg.payload_as_string(), other_msg.payload_as_string())
        {
            self_str.value() == other_str.value()
        } else {
            false
        }
    }
}

impl StringBuffer {
    pub fn builder() -> StringBufferBuilder {
        StringBufferBuilder::default()
    }

    /// Retourne la valeur String du buffer, si présente.
    pub fn value(&self) -> Option<&str> {
        self.as_message()
            .payload_as_string()
            .and_then(|s| s.value())
    }
}
