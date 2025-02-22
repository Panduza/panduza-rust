use crate::{
    attribute_metadata::AttributeMetadata,
    pubsub::{Operator, Publisher},
    BooleanAttribute, Reactor,
};

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
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic, false)
            .map_err(|e| e.to_string())?;

        Ok(BooleanAttribute::new(cmd_publisher, att_receiver))
    }
}
