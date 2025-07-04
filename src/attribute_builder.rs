use crate::attribute::bytes::BytesAttribute;
use crate::attribute::json::JsonAttribute;
use crate::attribute::notification::NotificationAttribute;
use crate::attribute::number::NumberAttribute;
use crate::attribute::status::StatusAttribute;
use crate::attribute_metadata::AttributeMetadata;
use crate::reactor::Reactor;
use crate::BooleanAttribute;
use crate::StringAttribute;

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

    // ------------------------------------------------------------------------

    /// BOOLEAN
    ///
    pub async fn expect_boolean(self) -> Result<BooleanAttribute, String> {
        let metadata = self
            .metadata
            .ok_or_else(|| "Metadata is required".to_string())?;
        Ok(BooleanAttribute::new(self.reactor.session, metadata).await)
    }

    // ------------------------------------------------------------------------

    /// NUMBER
    ///
    pub async fn expect_number(self) -> Result<NumberAttribute, String> {
        let metadata = self
            .metadata
            .ok_or_else(|| "Metadata is required".to_string())?;
        Ok(NumberAttribute::new(self.reactor.session, metadata).await)
    }

    // ------------------------------------------------------------------------

    /// STRING
    ///
    pub async fn expect_string(self) -> Result<StringAttribute, String> {
        let metadata = self
            .metadata
            .ok_or_else(|| "Metadata is required".to_string())?;
        Ok(StringAttribute::new(self.reactor.session, metadata).await)
    }

    // ------------------------------------------------------------------------

    /// BYTES
    ///
    pub async fn expect_bytes(self) -> Result<BytesAttribute, String> {
        let metadata = self
            .metadata
            .ok_or_else(|| "Metadata is required".to_string())?;
        Ok(BytesAttribute::new(self.reactor.session, metadata).await)
    }

    pub async fn expect_json(&self) -> Result<JsonAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!(
            "{}{}/att",
            self.reactor
                .namespace
                .clone()
                .map_or("".to_string(), |ns| format!("{}/", ns)),
            md.topic
        );
        let cmd_topic = format!(
            "{}{}/cmd",
            self.reactor
                .namespace
                .clone()
                .map_or("".to_string(), |ns| format!("{}/", ns)),
            md.topic
        );

        let att_receiver = self.reactor.register_listener(att_topic.clone()).await;

        // let cmd_publisher = self
        //     .reactor
        //     .register_publisher(cmd_topic.clone())
        //     .await
        //     .map_err(|e| e.to_string())?;

        Ok(JsonAttribute::new(
            self.reactor.session.clone(),
            cmd_topic,
            att_topic,
            md.mode.clone(),
            att_receiver,
        )
        .await)
    }

    // ------------------------------------------------------------------------

    pub async fn expect_status(&self) -> Result<StatusAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic.clone()).await;

        Ok(StatusAttribute::new(self.reactor.session.clone(), att_topic, att_receiver).await)
    }

    // ------------------------------------------------------------------------

    pub async fn expect_notification(&self) -> Result<NotificationAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        // let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic).await;

        // let cmd_publisher = self
        //     .reactor
        //     .register_publisher(cmd_topic.clone())
        //     .await
        //     .map_err(|e| e.to_string())?;

        Ok(NotificationAttribute::new(
            self.reactor.session.clone(),
            md.topic.clone(),
            md.mode.clone(),
            att_receiver,
        )
        .await)
    }
}
