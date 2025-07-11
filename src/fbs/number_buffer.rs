use crate::fbs::generate_timestamp;
use crate::fbs::{
    panduza_generated::panduza::{
        Header, HeaderArgs, Message, MessageArgs, Number as FbNumber, NumberArgs, Payload,
    },
    PzaBuffer,
};
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct NumberBufferBuilder {
    value: Option<f64>,
    source: Option<u16>,
    sequence: Option<u16>,
}

impl NumberBufferBuilder {
    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
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

    pub fn build(self) -> Result<NumberBuffer, String> {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let timestamp = generate_timestamp();

        let value = self.value.ok_or("value not provided".to_string())?;
        let number_args = NumberArgs {
            value,
            unit: None,
            decimals: 0,
            range: None,
            whitelist: None,
        };
        let fb_number = FbNumber::create(&mut builder, &number_args);

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
            payload_type: Payload::Number,
            payload: Some(fb_number.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        Ok(NumberBuffer {
            raw_data: Bytes::from(builder.finished_data().to_vec()),
        })
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct NumberBuffer {
    raw_data: Bytes,
}

impl PzaBuffer for NumberBuffer {
    fn from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        NumberBuffer { raw_data: bytes }
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

        if let (Some(self_number), Some(other_number)) =
            (self_msg.payload_as_number(), other_msg.payload_as_number())
        {
            self_number.value() == other_number.value()
        } else {
            false
        }
    }
}

impl NumberBuffer {
    pub fn builder() -> NumberBufferBuilder {
        NumberBufferBuilder::default()
    }

    /// Retourne la valeur du buffer sous forme de f64, si présente.
    pub fn value(&self) -> Option<f64> {
        self.as_message().payload_as_number().map(|n| n.value())
    }
}
