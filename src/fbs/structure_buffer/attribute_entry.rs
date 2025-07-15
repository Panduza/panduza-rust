use crate::fbs::panduza_generated::panduza::{AttributeEntry, AttributeEntryArgs};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AttributeEntryBufferBuilder {
    /// Name of the attribute
    ///
    pub name: Option<String>,

    /// Type of the attribute
    ///
    pub type_: Option<String>,

    /// Mode of the attribute (e.g., read, write)
    ///
    pub mode: Option<String>,
}

impl AttributeEntryBufferBuilder {
    // ------------------------------------------------------------------------

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    // ------------------------------------------------------------------------

    pub fn with_type(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }

    // ------------------------------------------------------------------------

    pub fn with_mode(mut self, mode: String) -> Self {
        self.mode = Some(mode);
        self
    }

    // ------------------------------------------------------------------------

    pub fn build_wip_offset<'a>(
        &self,
        builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<AttributeEntry<'a>> {
        let args = AttributeEntryArgs {
            name: self.name.as_ref().map(|s| builder.create_string(s)),
            type_: self.type_.as_ref().map(|s| builder.create_string(s)),
            mode: self.mode.as_ref().map(|s| builder.create_string(s)),
        };
        AttributeEntry::create(builder, &args)
    }

    // ------------------------------------------------------------------------
}

// ----------------------------------------------------------------------------

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AttributeEntryBuffer {}

impl AttributeEntryBuffer {
    /// Creates a new builder for AttributeEntry.
    ///
    pub fn builder() -> AttributeEntryBufferBuilder {
        AttributeEntryBufferBuilder::default()
    }
}
