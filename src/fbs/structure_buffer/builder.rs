use super::super::generate_timestamp;
use crate::fbs::panduza_generated::panduza::Header;
use crate::fbs::panduza_generated::panduza::HeaderArgs;
use crate::fbs::panduza_generated::panduza::Message;
use crate::fbs::panduza_generated::panduza::MessageArgs;
use crate::fbs::panduza_generated::panduza::Node;
use crate::fbs::panduza_generated::panduza::Payload;
use crate::fbs::panduza_generated::panduza::Structure;
use crate::fbs::panduza_generated::panduza::StructureArgs;
use crate::fbs::PzaBufferBuilder;
use crate::fbs::StructureBuffer;
use bytes::Bytes;
use flatbuffers::FlatBufferBuilder;
use flatbuffers::WIPOffset;
use rand::random;
use std::fmt::Debug;

#[cfg(test)]
mod tests;

/// Builder for StructureBuffer
#[derive(Default, Clone, PartialEq, Debug)]
pub struct StructureBufferBuilder {
    /// Name of this node
    pub name: Option<String>,
    /// Node type
    pub node: Option<String>,
    /// Child nodes, because the structure represent a tree
    pub children: Option<Vec<StructureBufferBuilder>>,
    /// Node tags
    pub tags: Vec<String>,
    /// Attribute type
    pub r#type: Option<String>,
    /// Attribute mode (read/write)
    pub mode: Option<String>,
    /// Message source
    pub source: Option<u16>,
    /// Message sequence
    pub sequence: Option<u16>,
}

// ------------------------------------------------------------------------------
// Implementation of PzaBufferBuilder for StructureBufferBuilder
// ------------------------------------------------------------------------------
impl PzaBufferBuilder<StructureBuffer> for StructureBufferBuilder {
    // -------------------------------------------------------------------------------

    fn with_source(mut self, source: u16) -> Self {
        self.source = Some(source);
        self
    }

    // -------------------------------------------------------------------------------

    fn with_sequence(mut self, sequence: u16) -> Self {
        self.sequence = Some(sequence);
        self
    }

    // -------------------------------------------------------------------------------

    fn with_random_sequence(mut self) -> Self {
        self.sequence = Some(random());
        self
    }

    // -------------------------------------------------------------------------------

    fn build(self) -> Result<StructureBuffer, String> {
        let mut builder = FlatBufferBuilder::new();
        let timestamp = generate_timestamp();

        // Serialize name
        let name_offset = self
            .name
            .as_ref()
            .map(|n| builder.create_string(n.as_str()));

        // Serialize tags
        let tags_offsets: Vec<_> = self.tags.iter().map(|t| builder.create_string(t)).collect();
        let tags_vec = if !tags_offsets.is_empty() {
            Some(builder.create_vector(&tags_offsets))
        } else {
            None
        };

        // Serialize children recursively
        let children_vec = if let Some(children) = &self.children {
            let children_offsets: Vec<_> = children
                .iter()
                .map(|c| c.build_wip_offset(&mut builder))
                .collect();
            if !children_offsets.is_empty() {
                Some(builder.create_vector(&children_offsets))
            } else {
                None
            }
        } else {
            None
        };

        // Serialize type and mode
        let type_offset = self.r#type.as_ref().map(|t| builder.create_string(t));
        let mode_offset = self.mode.as_ref().map(|m| builder.create_string(m));

        // Node type (as enum)
        let node_enum = match &self.node {
            Some(n) if n == "Instance" => Node::Instance,
            Some(n) if n == "Class" => Node::Class,
            Some(n) if n == "Attribute" => Node::Attribute,
            _ => Node::Undefined,
        };

        let structure_args = StructureArgs {
            name: name_offset,
            node: node_enum,
            children: children_vec,
            tags: tags_vec,
            type_: type_offset,
            mode: mode_offset,
        };
        let structure_offset = Structure::create(&mut builder, &structure_args);

        // Header
        let header_args = HeaderArgs {
            timestamp: Some(&timestamp),
            source: self.source.unwrap_or(0),
            sequence: self.sequence.unwrap_or(0),
        };
        let header_offset = Header::create(&mut builder, &header_args);

        // Message
        let mut msg_args = MessageArgs::default();
        msg_args.header = Some(header_offset);
        msg_args.payload_type = Payload::Structure;
        msg_args.payload = Some(flatbuffers::WIPOffset::new(structure_offset.value()));
        let msg_offset = Message::create(&mut builder, &msg_args);

        builder.finish(msg_offset, None);
        let data = builder.finished_data();
        Ok(StructureBuffer {
            raw_data: Bytes::copy_from_slice(data),
        })
    }
}

// ------------------------------------------------------------------------------
// Builder-specific methods for StructureBufferBuilder
// ------------------------------------------------------------------------------
impl StructureBufferBuilder {
    /// Set the name of the node
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    // -------------------------------------------------------------------------------

    /// Set the node type
    pub fn with_node(mut self, node: String) -> Self {
        self.node = Some(node);
        self
    }

    // -------------------------------------------------------------------------------

    /// Set the children nodes
    pub fn with_children(mut self, children: Vec<StructureBufferBuilder>) -> Self {
        self.children = Some(children);
        self
    }

    // -------------------------------------------------------------------------------

    /// Set the attribute type
    pub fn with_type(mut self, r#type: String) -> Self {
        self.r#type = Some(r#type);
        self
    }

    // -------------------------------------------------------------------------------

    /// Set the attribute mode
    pub fn with_mode(mut self, mode: String) -> Self {
        self.mode = Some(mode);
        self
    }

    // -------------------------------------------------------------------------------

    /// Add a tag to the node
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    // -------------------------------------------------------------------------------

    /// Replace all tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    // -------------------------------------------------------------------------------

    /// Build the FlatBuffer WIPOffset for this node
    pub fn build_wip_offset<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<Structure<'a>> {
        let name_offset = self
            .name
            .as_ref()
            .map(|n| builder.create_string(n.as_str()));
        let tags_offsets: Vec<_> = self.tags.iter().map(|t| builder.create_string(t)).collect();
        let tags_vec = if !tags_offsets.is_empty() {
            Some(builder.create_vector(&tags_offsets))
        } else {
            None
        };
        let children_vec = if let Some(children) = &self.children {
            let children_offsets: Vec<_> = children
                .iter()
                .map(|c| c.build_wip_offset(builder))
                .collect();
            if !children_offsets.is_empty() {
                Some(builder.create_vector(&children_offsets))
            } else {
                None
            }
        } else {
            None
        };
        let type_offset = self.r#type.as_ref().map(|t| builder.create_string(t));
        let mode_offset = self.mode.as_ref().map(|m| builder.create_string(m));
        let node_enum = match &self.node {
            Some(n) if n == "Instance" => Node::Instance,
            Some(n) if n == "Class" => Node::Class,
            Some(n) if n == "Attribute" => Node::Attribute,
            _ => Node::Undefined,
        };
        let structure_args = StructureArgs {
            name: name_offset,
            node: node_enum,
            children: children_vec,
            tags: tags_vec,
            type_: type_offset,
            mode: mode_offset,
        };
        Structure::create(builder, &structure_args)
    }

    // -------------------------------------------------------------------------------

    /// Insert a child node
    pub fn insert_child(&mut self, child: StructureBufferBuilder) {
        if self.children.is_none() {
            self.children = Some(vec![]);
        }
        self.children.as_mut().unwrap().push(child);
    }

    // -------------------------------------------------------------------------------

    /// Check if a child with the given name exists
    pub fn is_children_exists_with_name(&self, name: &str) -> bool {
        if let Some(children) = &self.children {
            children.iter().any(|c| c.name.as_deref() == Some(name))
        } else {
            false
        }
    }

    // -------------------------------------------------------------------------------

    /// Recursively insert a node at the given path
    pub fn insert_node(&mut self, mut path: Vec<String>, node: StructureBufferBuilder) {
        if path.is_empty() {
            self.insert_child(node);
            return;
        }
        let next = path.remove(0);
        if let Some(children) = &mut self.children {
            if let Some(child) = children
                .iter_mut()
                .find(|c| c.name.as_deref() == Some(&next))
            {
                child.insert_node(path, node);
                return;
            }
        }
        let mut new_child = StructureBufferBuilder::default().with_name(next.clone());
        new_child.insert_node(path, node);
        self.insert_child(new_child);
    }
}
