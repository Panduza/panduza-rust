use super::common::generate_timestamp;
use super::generic::PanduzaBuffer;
use super::panduza_generated::panduza::{
    Header, HeaderArgs, InstanceStatus as FbInstanceStatus, InstanceStatusArgs, Message,
    MessageArgs, Payload, Status as FbStatus, StatusArgs,
};
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
/// InstanceStatusBuffer is a wrapper around a flatbuffer serialized Message with an InstanceStatus payload.
/// It provides methods to create, access, and manipulate instance status data.
pub struct InstanceStatusBuffer {
    instance: Option<String>,
    state: Option<u16>,
    error_string: Option<String>,

    source: Option<u16>,
    sequence: Option<u16>,
    raw_data: Option<Bytes>,
}

impl PanduzaBuffer for InstanceStatusBuffer {
    fn new() -> Self {
        Self {
            instance: None,
            state: None,
            error_string: None,
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
            value: Some(value.into().value),
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
        let value = self.value.ok_or("value not provided".to_string())?;
        let instance_status_args = InstanceStatusArgs { value };
        let instance_status = FbInstanceStatus::create(&mut builder, &instance_status_args);

        let timestamp = generate_timestamp();
        let source = self.source.ok_or("source not provided".to_string())?;
        let sequence = self.sequence.ok_or("sequence not provided".to_string())?;

        let header_args = HeaderArgs {
            timestamp: Some(&timestamp),
            source,
            sequence,
        };
        let header = Header::create(&mut builder, &header_args);

        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::InstanceStatus,
            payload: Some(instance_status.as_union_value()),
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
        if let Some(payload) = message.payload_as_instance_status() {
            if let Some(value) = self.value {
                return payload.value() == value;
            }
        }
        false
    }
}

impl From<u8> for InstanceStatusBuffer {
    fn from(value: u8) -> Self {
        let mut obj = Self::new();
        obj.value = Some(value);
        obj
    }
}

impl From<InstanceStatusBuffer> for u8 {
    fn from(buffer: InstanceStatusBuffer) -> Self {
        match buffer.value {
            Some(v) => v,
            None => buffer.value(),
        }
    }
}

impl InstanceStatusBuffer {
    pub fn instance_status(&self) -> Option<FbInstanceStatus> {
        self.as_message().payload_as_instance_status()
    }

    pub fn value(&self) -> u8 {
        self.instance_status().map_or(0, |s| s.value())
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
/// StatusBuffer is a wrapper around a flatbuffer serialized Message with a Status payload.
/// It provides methods to create, access, and manipulate status data.
pub struct StatusBuffer {
    value: Option<u8>,
    source: Option<u16>,
    sequence: Option<u16>,
    raw_data: Option<Bytes>,
}

impl PanduzaBuffer for StatusBuffer {
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
        let value = self.value.ok_or("value not provided".to_string())?;
        let timestamp = generate_timestamp();
        let status_args = StatusArgs {
            instances: None,
            timestamp: Some(&timestamp),
        };
        let status = FbStatus::create(&mut builder, &status_args);

        let source = self.source.ok_or("source not provided".to_string())?;
        let sequence = self.sequence.ok_or("sequence not provided".to_string())?;

        let header_args = HeaderArgs {
            timestamp: Some(&timestamp),
            source,
            sequence,
        };
        let header = Header::create(&mut builder, &header_args);

        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::Status,
            payload: Some(status.as_union_value()),
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
        if let Some(payload) = message.payload_as_status() {
            if let Some(value) = self.value {
                return payload.value() == value;
            }
        }
        false
    }
}

impl From<u8> for StatusBuffer {
    fn from(value: u8) -> Self {
        let mut obj = Self::new();
        obj.value = Some(value);
        obj
    }
}

impl From<StatusBuffer> for u8 {
    fn from(buffer: StatusBuffer) -> Self {
        match buffer.value {
            Some(v) => v,
            None => buffer.value(),
        }
    }
}

impl StatusBuffer {
    pub fn status(&self) -> Option<FbStatus> {
        self.as_message().payload_as_status()
    }

    pub fn value(&self) -> u8 {
        self.status().map_or(0, |s| s.value())
    }
}
