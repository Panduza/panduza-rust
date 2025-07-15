use crate::fbs::panduza_generated;
use crate::fbs::structure_buffer::attribute_entry::AttributeEntryBuilder;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ClassEntryBuilder {
    pub name: Option<String>,
    pub tags: Vec<String>,
    pub attributes: Vec<AttributeEntryBuilder>,
    pub classes: Vec<ClassEntryBuilder>,
}

impl ClassEntryBuilder {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_attribute(mut self, attribute: AttributeEntryBuilder) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn with_attributes(mut self, attributes: Vec<AttributeEntryBuilder>) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn with_class(mut self, class: ClassEntryBuilder) -> Self {
        self.classes.push(class);
        self
    }

    pub fn with_classes(mut self, classes: Vec<ClassEntryBuilder>) -> Self {
        self.classes = classes;
        self
    }

    pub fn to_fbs_args<'a>(
        &self,
        builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> panduza_generated::panduza::ClassEntryArgs<'a> {
        let name = self.name.as_ref().map(|s| builder.create_string(s));

        // Convert tags Vec<String> to a FlatBuffer vector of string offsets
        let tags_vec: Vec<_> = self
            .tags
            .iter()
            .map(|tag| builder.create_string(tag))
            .collect();
        let tags = if !tags_vec.is_empty() {
            Some(builder.create_vector(&tags_vec))
        } else {
            None
        };

        // Attributs
        let attributes_vec = self
            .attributes
            .iter()
            .map(|a| {
                let args = a.to_fbs_args(builder);
                panduza_generated::panduza::AttributeEntry::create(builder, &args)
            })
            .collect::<Vec<_>>();
        let attributes = if !attributes_vec.is_empty() {
            Some(builder.create_vector(&attributes_vec))
        } else {
            None
        };

        // Classes
        let classes_vec = self
            .classes
            .iter()
            .map(|c| {
                let args = c.to_fbs_args(builder);
                panduza_generated::panduza::ClassEntry::create(builder, &args)
            })
            .collect::<Vec<_>>();
        let classes = if !classes_vec.is_empty() {
            Some(builder.create_vector(&classes_vec))
        } else {
            None
        };

        panduza_generated::panduza::ClassEntryArgs {
            name,
            tags,
            attributes,
            classes,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ClassEntry {}

impl ClassEntry {
    /// Creates a new builder for ClassEntry.
    pub fn builder() -> ClassEntryBuilder {
        ClassEntryBuilder::default()
    }
}
