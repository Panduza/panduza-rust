///
///
mod attribute_entry;
pub use attribute_entry::AttributeEntryBuffer;
pub use attribute_entry::AttributeEntryBufferBuilder;

use super::generate_timestamp;
use super::panduza_generated::panduza::Message;
use crate::fbs::panduza_generated::panduza::Header;
use crate::fbs::panduza_generated::panduza::HeaderArgs;
use crate::fbs::panduza_generated::panduza::MessageArgs;
use crate::fbs::panduza_generated::panduza::Payload;
use crate::fbs::panduza_generated::panduza::StructureNode;
use crate::fbs::PzaBuffer;
use crate::fbs::PzaBufferBuilder;
use bytes::Bytes;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StructureBufferBuilder {
    ///
    pub name: Option<String>,

    ///
    pub tags: Vec<String>,

    /// Attribute entries for this node
    ///
    pub attributes: Option<Vec<AttributeEntryBufferBuilder>>,

    ///
    pub classes: Option<Vec<StructureBufferBuilder>>,

    ///
    pub source: Option<u16>,

    ///
    pub sequence: Option<u16>,
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

        // Serialize name
        let name_offset = if let Some(name) = self.name {
            Some(builder.create_string(&name))
        } else {
            None
        };

        // Serialize tags
        let tags_offsets: Vec<_> = self
            .tags
            .iter()
            .map(|tag| builder.create_string(tag))
            .collect();
        let tags_vec = if !tags_offsets.is_empty() {
            Some(builder.create_vector(&tags_offsets))
        } else {
            None
        };

        // Serialize attributes (optionnel, à adapter selon ta structure FlatBuffers)
        // Ici, on suppose que AttributeEntryBufferBuilder a une méthode build_flatbuffer(&mut builder) qui retourne un WIPOffset
        let attributes_vec = if let Some(attributes) = &self.attributes {
            let attr_offsets: Vec<_> = attributes
                .iter()
                .map(|attr| attr.build_wip_offset(&mut builder))
                .collect();
            if !attr_offsets.is_empty() {
                Some(builder.create_vector(&attr_offsets))
            } else {
                None
            }
        } else {
            None
        };

        // Serialize classes (récursif, à adapter selon ta structure FlatBuffers)
        let classes_vec = if let Some(classes) = &self.classes {
            let class_offsets: Vec<_> = classes
                .iter()
                .map(|class| class.build_wip_offset(&mut builder))
                .collect();
            if !class_offsets.is_empty() {
                Some(builder.create_vector(&class_offsets))
            } else {
                None
            }
        } else {
            None
        };

        // Créer le node de structure (adapter selon le schéma FlatBuffers)
        // Supposons que StructureNodeArgs existe dans panduza_generated
        let structure_node_args = crate::fbs::panduza_generated::panduza::StructureNodeArgs {
            name: name_offset,
            tags: tags_vec,
            attributes: attributes_vec,
            children: classes_vec,
        };
        let structure_node_offset = crate::fbs::panduza_generated::panduza::StructureNode::create(
            &mut builder,
            &structure_node_args,
        );

        // Create the header
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

        // Create the message avec payload
        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::StructureNode,
            payload: Some(structure_node_offset.as_union_value()),
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

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    // ------------------------------------------------------------------------

    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    // ------------------------------------------------------------------------

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    // ------------------------------------------------------------------------

    pub fn build_wip_offset<'a>(
        &self,
        builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<StructureNode<'a>> {
        // Serialize tags
        let tags_offsets: Vec<_> = self
            .tags
            .iter()
            .map(|tag| builder.create_string(tag))
            .collect();
        let tags_vec = if !tags_offsets.is_empty() {
            Some(builder.create_vector(&tags_offsets))
        } else {
            None
        };

        //
        let args = crate::fbs::panduza_generated::panduza::StructureNodeArgs {
            name: self.name.as_ref().map(|s| builder.create_string(s)),
            tags: tags_vec,
            attributes: if let Some(attributes) = &self.attributes {
                let attr_offsets: Vec<_> = attributes
                    .iter()
                    .map(|attr| attr.build_wip_offset(builder))
                    .collect();
                Some(builder.create_vector(&attr_offsets))
            } else {
                None
            },
            children: if let Some(classes) = &self.classes {
                let class_offsets: Vec<_> = classes
                    .iter()
                    .map(|class| class.build_wip_offset(builder))
                    .collect();
                Some(builder.create_vector(&class_offsets))
            } else {
                None
            },
        };
        crate::fbs::panduza_generated::panduza::StructureNode::create(builder, &args)
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
