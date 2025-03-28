use crate::{
    attribute_metadata::AttributeMetadata,
    bytes_attribute::BytesPublisher,
    pubsub::{Operator, Publisher},
    BooleanAttribute, BytesAttribute, Reactor, StringAttribute,
};
use bytes::Bytes;

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

    pub async fn expect_string(&self) -> Result<StringAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic, false)
            .map_err(|e| e.to_string())?;

        Ok(StringAttribute::new(cmd_publisher, att_receiver))
    }

    pub async fn expect_bytes(&self) -> Result<BytesAttribute, Bytes> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic, false)
            .map_err(|e| e.to_string())?;

        Ok(BytesAttribute::new(cmd_publisher, att_receiver))
    }

    pub async fn expect_bytes_publisher(&self) -> Result<BytesPublisher, String> {
        let md = self.metadata.as_ref().unwrap();
        let cmd_topic = format!("{}/cmd", md.topic);

        self.reactor
            .register_publisher(cmd_topic, false)
            .map(|publisher| BytesPublisher::new(publisher))
            .map_err(|e| e.to_string())
    }
}
