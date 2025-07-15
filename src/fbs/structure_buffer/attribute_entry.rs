use crate::fbs::panduza_generated;

// ----------------------------------------------------------------------------

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AttributeEntryBuilder {
    pub name: Option<String>,
    pub type_: Option<String>,
    pub mode: Option<String>,
}

impl AttributeEntryBuilder {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_type(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn with_mode(mut self, mode: String) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn to_fbs_args<'a>(
        &self,
        _builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> panduza_generated::panduza::AttributeEntryArgs<'a> {
        // Remplacez par la logique réelle pour convertir en arguments FlatBuffers
        panduza_generated::panduza::AttributeEntryArgs {
            name: self.name.as_ref().map(|s| _builder.create_string(s)),
            type_: self.type_.as_ref().map(|s| _builder.create_string(s)),
            mode: self.mode.as_ref().map(|s| _builder.create_string(s)),
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AttributeEntry {}

impl AttributeEntry {
    /// Creates a new builder for AttributeEntry.
    ///
    pub fn builder() -> AttributeEntryBuilder {
        AttributeEntryBuilder::default()
    }
}
