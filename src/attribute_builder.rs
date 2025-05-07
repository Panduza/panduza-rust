use crate::{
    attribute_metadata::AttributeMetadata,
    fbs::trigger_v0::TriggerBuffer,
    reactor::{self, Reactor},
    session, BooleanAttribute, BytesAttribute, NumberAttribute, StringAttribute,
};
use bytes::Bytes;
use zenoh::{
    handlers::FifoChannelHandler, matching::MatchingListener, matching::MatchingStatus, Session,
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

        let att_receiver = self.reactor.register_listener(att_topic).await;

        // let listener = ZenohListener::new(self.reactor.session.operator.session, &att_topic);

        let _cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(BooleanAttribute::new(
            self.reactor.session.clone(),
            att_receiver,
            cmd_topic,
        ))
    }

    pub async fn expect_string(&self) -> Result<StringAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic).await;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(StringAttribute::new(
            self.reactor.session.clone(),
            att_receiver,
            cmd_topic,
        ))
    }

    pub async fn expect_bytes(&self) -> Result<BytesAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic).await;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(BytesAttribute::new(
            self.reactor.session.clone(),
            att_receiver,
            cmd_topic,
        ))
    }

    pub async fn expect_number(&self) -> Result<NumberAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic).await;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(NumberAttribute::new(
            self.reactor.session.clone(),
            att_receiver,
            cmd_topic,
        ))
    }
}
