use crate::{
    attribute::{notification::NotificationAttribute, status::StatusAttribute},
    attribute_metadata::AttributeMetadata,
    fbs::trigger_v0::TriggerBuffer,
    reactor::{self, Reactor},
    session, BooleanAttribute, BytesAttribute, JsonAttribute, NumberAttribute, SiAttribute,
    StringAttribute,
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
            md.mode.clone(),
            att_receiver,
            cmd_topic,
        )
        .await)
    }

    pub async fn expect_string(&self) -> Result<StringAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic.clone()).await;

        // let struct_query = self.reactor.session.get(att_topic).await.unwrap();

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(StringAttribute::new(
            self.reactor.session.clone(),
            md.mode.clone(),
            att_receiver,
            cmd_topic,
        )
        .await)
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
            md.mode.clone(),
            att_receiver,
            cmd_topic,
        )
        .await)
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
            md.mode.clone(),
            att_receiver,
            cmd_topic,
        )
        .await)
    }
    pub async fn expect_json(&self) -> Result<JsonAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic, false)
            .map_err(|e| e.to_string())?;

        Ok(JsonAttribute::new(md.topic.clone(), cmd_publisher, att_receiver).await)
    }

    pub async fn expect_si(&self) -> Result<SiAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic, false)
            .map_err(|e| e.to_string())?;

        Ok(SiAttribute::new(
            md.topic.clone(),
            md.mode.clone(),
            cmd_publisher,
            att_receiver,
        )
        .await)
    }

    pub async fn expect_enum(&self) -> Result<StringAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic, false)
            .map_err(|e| e.to_string())?;

        Ok(StringAttribute::new(
            md.topic.clone(),
            md.mode.clone(),
            cmd_publisher,
            att_receiver,
        )
        .await)
    }

    pub async fn expect_status(&self) -> Result<StatusAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        Ok(StatusAttribute::new(md.topic.clone(), att_receiver).await)
    }

    pub async fn expect_notification(&self) -> Result<NotificationAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic, false)
            .map_err(|e| e.to_string())?;

        Ok(NotificationAttribute::new(
            md.topic.clone(),
            md.mode.clone(),
            cmd_publisher,
            att_receiver,
        )
        .await)
    }
}
