mod instance_status_buffer;
use super::common::generate_timestamp;
use super::panduza_generated::panduza::Header;
use super::panduza_generated::panduza::HeaderArgs;
use super::panduza_generated::panduza::InstanceStatus as FbInstanceStatus;
use super::panduza_generated::panduza::Message;
use super::panduza_generated::panduza::MessageArgs;
use super::panduza_generated::panduza::Payload;
use super::panduza_generated::panduza::Status as FbStatus;
use super::panduza_generated::panduza::StatusArgs;
use crate::fbs::PzaBuffer;
use crate::fbs::PzaBufferBuilder;
use crate::InstanceState;
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
    // ------------------------------------------------------------------------

    fn with_source(mut self, source: u16) -> Self {
        self.source = Some(source);
        self
    }

    // ------------------------------------------------------------------------

    fn with_sequence(mut self, sequence: u16) -> Self {
        self.sequence = Some(sequence);
        self
    }

    // ------------------------------------------------------------------------

    fn with_random_sequence(mut self) -> Self {
        let mut rng = rand::thread_rng();
        self.sequence = Some(rng.gen::<u16>());
        self
    }

    // ------------------------------------------------------------------------

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
    // ------------------------------------------------------------------------

    fn from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        StatusBuffer { raw_data: bytes }
    }

    // ------------------------------------------------------------------------

    fn to_zbytes(self) -> ZBytes {
        ZBytes::from(self.raw_data)
    }

    // ------------------------------------------------------------------------

    fn source(&self) -> u16 {
        let msg = self.as_message();
        msg.header().map(|h| h.source()).unwrap_or(0)
    }

    // ------------------------------------------------------------------------

    fn sequence(&self) -> u16 {
        let msg = self.as_message();
        msg.header().map(|h| h.sequence()).unwrap_or(0)
    }

    // ------------------------------------------------------------------------

    fn as_message(&self) -> Message {
        flatbuffers::root::<Message>(&self.raw_data)
            .expect("Failed to deserialize Message from raw_data")
    }

    // ------------------------------------------------------------------------

    fn has_same_message_value<B: PzaBuffer>(&self, other_buffer: &B) -> bool {
        let self_msg = self.as_message();
        let other_msg = other_buffer.as_message();

        // Compare payload types
        if self_msg.payload_type() != other_msg.payload_type() {
            return false;
        }

        // Compare payloads for Status type
        if let (Some(self_status), Some(other_status)) =
            (self_msg.payload_as_status(), other_msg.payload_as_status())
        {
            let self_instances = self_status.instances();
            let other_instances = other_status.instances();

            let self_len = self_instances.map(|v| v.len()).unwrap_or(0);
            let other_len = other_instances.map(|v| v.len()).unwrap_or(0);
            if self_len != other_len {
                return false;
            }

            match (self_instances, other_instances) {
                (Some(self_vec), Some(other_vec)) => {
                    for (a, b) in self_vec.iter().zip(other_vec.iter()) {
                        if a.state() != b.state() {
                            return false;
                        }
                    }
                    true
                }
                (None, None) => true,
                _ => false,
            }
        } else {
            false
        }
    }

    // ------------------------------------------------------------------------
}

impl StatusBuffer {
    // ------------------------------------------------------------------------

    /// Returns true if all instances in the status buffer are in the "running" state.
    ///
    pub fn all_instances_are_running(&self) -> bool {
        let msg = self.as_message();
        if let Some(status) = msg.payload_as_status() {
            if let Some(instances) = status.instances() {
                for inst in instances {
                    if inst.state() != InstanceState::Running as u16 {
                        return false;
                    }
                }
                return true;
            }
        }
        false
    }

    // ------------------------------------------------------------------------
}
