///
///
mod attribute_entry;
pub use attribute_entry::AttributeEntry;
pub use attribute_entry::AttributeEntryBuilder;

///
///
mod class_entry;
pub use class_entry::ClassEntry;
pub use class_entry::ClassEntryBuilder;

use super::generate_timestamp;
use super::panduza_generated::panduza::Header;
use super::panduza_generated::panduza::HeaderArgs;
use super::panduza_generated::panduza::Message;
use super::panduza_generated::panduza::MessageArgs;
use super::panduza_generated::panduza::Payload;
use super::panduza_generated::panduza::Structure as FbStructure;
use super::panduza_generated::panduza::StructureArgs;
use crate::fbs::PzaBuffer;
use crate::fbs::PzaBufferBuilder;
use bytes::Bytes;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StructureBufferBuilder {
    ///
    attributes: Option<Vec<AttributeEntryBuilder>>,

    ///
    classes: Option<Vec<ClassEntryBuilder>>,

    ///
    source: Option<u16>,

    ///
    sequence: Option<u16>,
}

impl PzaBufferBuilder<StructureBuffer> for StructureBufferBuilder {
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
        self.sequence = Some(rand::random());
        self
    }

    // ------------------------------------------------------------------------

    fn build(self) -> Result<StructureBuffer, String> {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let timestamp = generate_timestamp();

        let structure_args = StructureArgs {
            attributes: None,
            classes: None,
        };
        let structure = FbStructure::create(&mut builder, &structure_args);

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
            payload_type: Payload::Structure,
            payload: Some(structure.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        Ok(StructureBuffer {
            raw_data: Bytes::from(builder.finished_data().to_vec()),
        })
    }

    // ------------------------------------------------------------------------
}

impl StructureBufferBuilder {
    // ------------------------------------------------------------------------

    pub fn with_attributes(mut self, attributes: Vec<AttributeEntryBuilder>) -> Self {
        self.attributes = Some(attributes);
        self
    }

    // ------------------------------------------------------------------------

    pub fn with_classes(mut self, classes: Vec<ClassEntryBuilder>) -> Self {
        self.classes = Some(classes);
        self
    }

    // ------------------------------------------------------------------------
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StructureBuffer {
    raw_data: Bytes,
}

impl PzaBuffer for StructureBuffer {
    fn from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        StructureBuffer { raw_data: bytes }
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

    fn has_same_message_value<B: PzaBuffer>(&self, _other_buffer: &B) -> bool {
        true
    }
}

impl StructureBuffer {
    pub fn builder() -> StructureBufferBuilder {
        StructureBufferBuilder::default()
    }
}
