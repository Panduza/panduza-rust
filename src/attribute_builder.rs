use crate::{
    attribute::{notification::NotificationAttribute, status::StatusAttribute},
    attribute_metadata::AttributeMetadata,
    BooleanAttribute, BytesAttribute, JsonAttribute, Reactor, SiAttribute, StringAttribute,
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
        let att_topic: String = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic, false)
            .map_err(|e| e.to_string())?;

        Ok(BooleanAttribute::new(
            md.topic.clone(),
            md.mode.clone(),
            cmd_publisher,
            att_receiver,
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

    pub async fn expect_string(&self) -> Result<StringAttribute, String> {
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

    pub async fn expect_bytes(&self) -> Result<BytesAttribute, String> {
        let md = self
            .metadata
            .as_ref()
            .expect("Metadata is required but was not provided.");
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic, false)
            .map_err(|e| e.to_string())?;

        Ok(BytesAttribute::new(
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
