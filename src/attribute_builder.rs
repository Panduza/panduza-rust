use crate::{asyncv::reactor::Reactor, attribute_metadata::AttributeMetadata, BooleanAttribute};

#[derive(Clone)]
/// Metadata for an attribute
///
pub struct AttributeBuilder {
    ///
    ///
    reactor: Reactor,
    ///
    ///
    metadata: Option<AttributeMetadata>,
}

impl AttributeBuilder {
    ///
    ///
    pub fn new(reactor: Reactor, metadata: Option<AttributeMetadata>) -> Self {
        Self {
            reactor: reactor,
            metadata: metadata,
        }
    }

    pub async fn expect_boolean(&self) -> Result<BooleanAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);

        let pp = self.reactor.register_route(att_topic, 20).await?;

        Ok(BooleanAttribute::new(pp))
    }
}
