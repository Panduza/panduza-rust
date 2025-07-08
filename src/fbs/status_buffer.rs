mod instance_status_buffer;
use super::common::generate_timestamp;
use super::panduza_generated::panduza::{
    Header, HeaderArgs, InstanceStatus as FbInstanceStatus, Message, MessageArgs, Payload,
    Status as FbStatus, StatusArgs,
};
use crate::fbs::PzaBuffer;
use crate::fbs::PzaBufferBuilder;
use bytes::Bytes;
pub use instance_status_buffer::InstanceStatusBuffer;
use rand::Rng;
use zenoh::bytes::ZBytes;

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

impl StatusBufferBuilder {
    /// Attaches a list of instance statuses to the status buffer builder.
    ///
    pub fn with_instance_status_list(mut self, instances: Vec<InstanceStatusBuffer>) -> Self {
        self.instances = Some(instances);
        self
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
