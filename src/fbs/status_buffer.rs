///
mod instance_status_buffer;
pub use instance_status_buffer::InstanceStatusBuffer;

use crate::fbs::PzaBufferBuilder;

use super::common::generate_timestamp;
use super::panduza_generated::panduza::{
    Header, HeaderArgs, InstanceStatus as FbInstanceStatus, InstanceStatusArgs, Message,
    MessageArgs, Payload, Status as FbStatus, StatusArgs,
};
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

use crate::fbs::PzaBuffer;
use crate::fbs::PzaBufferError;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StatusBufferBuilder {
    instances: Option<Vec<InstanceStatusBuffer>>,
    source: Option<u16>,
    sequence: Option<u16>,
}

impl PzaBufferBuilder<StatusBuffer> for StatusBufferBuilder {
    fn with_value<T>(self, value: T) -> Self
    where
        T: Into<Self>,
    {
        todo!()
    }

    fn with_source(mut self, source: u16) -> Self {
        self.source = Some(source);
        self
    }

    fn with_sequence(mut self, sequence: u16) -> Self {
        self.sequence = Some(sequence);
        self
    }

    fn with_random_sequence(mut self) -> Self {
        let mut rng = rand::thread_rng();
        self.sequence = Some(rng.gen::<u16>());
        self
    }

    fn build(self) -> Result<StatusBuffer, String> {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let timestamp = generate_timestamp();

        let instances = self.instances.unwrap_or_default();
        let instance_statuses: Vec<_> = instances
            .into_iter()
            .map(|instance| {
                let instance_args = instance.to_fbs_args(&mut builder);
                FbInstanceStatus::create(&mut builder, &instance_args)
            })
            .collect();

        let instances_vector = builder.create_vector(&instance_statuses);

        let status_args = StatusArgs {
            instances: Some(instances_vector),
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

        Ok(StatusBuffer {
            raw_data: Bytes::from(builder.finished_data().to_vec()),
        })
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StatusBuffer {
    raw_data: Bytes,
}

impl PzaBuffer for StatusBuffer {
    fn from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        StatusBuffer { raw_data: bytes }
    }

    fn to_zbytes(self) -> ZBytes {
        ZBytes::from(self.raw_data)
    }

    fn source(&self) -> u16 {
        let msg = self.as_message();
        msg.header().map(|h| h.source()).unwrap_or(0)
    }

    fn sequence(&self) -> u16 {
        let msg = self.as_message();
        msg.header().map(|h| h.sequence()).unwrap_or(0)
    }

    fn as_message(&self) -> Message {
        flatbuffers::root::<Message>(&self.raw_data)
            .expect("Failed to deserialize Message from raw_data")
    }

    fn has_value_equal_to_message_value(&self, message: &Message) -> bool {
        // À adapter selon la logique métier, ici on compare les instances si possible
        // let self_status = self.as_message().payload_as_status();
        // let other_status = message.payload_as_status();
        // match (self_status, other_status) {
        //     (Some(s1), Some(s2)) => s1.instances() == s2.instances(),
        //     (None, None) => true,
        //     _ => false,
        // }
        false
    }
}

// #[derive(Default, Clone, Debug, PartialEq)]
// /// StatusBuffer is a wrapper around a flatbuffer serialized Message with a Status payload.
// /// It provides methods to create, access, and manipulate status data.
// pub struct StatusBuffer {
//     ///
//     value: Option<Vec<InstanceStatusBuffer>>,

//     source: Option<u16>,
//     sequence: Option<u16>,
//     raw_data: Option<Bytes>,
// }

// impl PanduzaBuffer for StatusBuffer {
//     fn new() -> Self {
//         Self {
//             value: None,
//             source: None,
//             sequence: None,
//             raw_data: None,
//         }
//     }

//     fn with_value<T>(self, value: T) -> Self
//     where
//         T: Into<Self>,
//     {
//         Self {
//             value: value.into().value,
//             ..self
//         }
//     }

//     fn with_source(self, source: u16) -> Self {
//         Self {
//             source: Some(source),
//             ..self
//         }
//     }

//     fn with_sequence(self, sequence: u16) -> Self {
//         Self {
//             sequence: Some(sequence),
//             ..self
//         }
//     }

//     fn with_random_sequence(self) -> Self {
//         let mut rng = rand::thread_rng();
//         Self {
//             sequence: Some(rng.gen::<u16>()),
//             ..self
//         }
//     }

//     fn build(self) -> Result<Self, String> {
//         let mut builder = flatbuffers::FlatBufferBuilder::new();
//         let value = self.value.clone().ok_or("value not provided".to_string())?;
//         let timestamp = generate_timestamp();
//         let status_args = StatusArgs {
//             instances: None,
//             timestamp: Some(&timestamp),
//         };
//         let status = FbStatus::create(&mut builder, &status_args);

//         let source = self.source.ok_or("source not provided".to_string())?;
//         let sequence = self.sequence.ok_or("sequence not provided".to_string())?;

//         let header_args = HeaderArgs {
//             timestamp: Some(&timestamp),
//             source,
//             sequence,
//         };
//         let header = Header::create(&mut builder, &header_args);

//         let message_args = MessageArgs {
//             header: Some(header),
//             payload_type: Payload::Status,
//             payload: Some(status.as_union_value()),
//         };
//         let message = Message::create(&mut builder, &message_args);

//         builder.finish(message, None);

//         Ok(Self {
//             raw_data: Some(Bytes::from(builder.finished_data().to_vec())),
//             value: self.value,
//             source: self.source,
//             sequence: self.sequence,
//         })
//     }

//     fn build_from_zbytes(zbytes: ZBytes) -> Self {
//         let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
//         Self {
//             raw_data: Some(bytes),
//             value: None,
//             source: None,
//             sequence: None,
//         }
//     }

//     fn is_builded(&self) -> bool {
//         self.raw_data.is_some()
//     }

//     fn sequence(&self) -> u16 {
//         self.sequence
//             .expect("Sequence must be set before accessing it")
//     }

//     fn to_zbytes(self) -> ZBytes {
//         ZBytes::from(
//             self.raw_data
//                 .expect("Raw data must be set before converting to ZBytes"),
//         )
//     }

//     fn as_message(&self) -> Message {
//         let data = self
//             .raw_data
//             .as_ref()
//             .expect("Buffer must be built to access the message");
//         flatbuffers::root::<Message>(data).expect("Failed to deserialize Message from raw_data")
//     }

//     fn has_value_equal_to_message_value(&self, message: &Message) -> bool {
//         // if let Some(payload) = message.payload_as_status() {
//         //     if let Some(value) = self.value {
//         //         return payload.value() == value;
//         //     }
//         // }
//         false
//     }
// }

// impl StatusBuffer {
//     pub fn status(&self) -> Option<FbStatus> {
//         self.as_message().payload_as_status()
//     }

//     pub fn with_instance_status_list(mut self, instances: Vec<InstanceStatusBuffer>) -> Self {
//         self.value = Some(instances);
//         self
//     }
// }
