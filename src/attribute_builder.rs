use crate::{
    attribute::{
        bytes::BytesAttribute, json::JsonAttribute, notification::NotificationAttribute,
        number::NumberAttribute, status::StatusAttribute,
    },
    attribute_metadata::AttributeMetadata,
    reactor::{self, Reactor},
    session, BooleanAttribute, SiAttribute, StringAttribute,
};
use bytes::Bytes;
use zenoh::{
    handlers::FifoChannelHandler, Session,
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

    /// BOOLEAN
    /// 
    pub async fn expect_boolean(self) -> Result<BooleanAttribute, String> {
        let metadata = self.metadata.ok_or_else(|| "Metadata is required".to_string())?;
        Ok(BooleanAttribute::new(
            self.reactor.session,
            metadata,
        )
        .await)
    }

    pub async fn expect_string(&self) -> Result<StringAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!(
            "{}{}/att",
            self.reactor
                .namespace
                .clone()
                .map_or("".to_string(), |ns| if ns.is_empty() {
                    "".to_string()
                } else {
                    format!("{}/", ns)
                }),
            md.topic
        );
        let cmd_topic = format!(
            "{}{}/cmd",
            self.reactor
                .namespace
                .clone()
                .map_or("".to_string(), |ns| if ns.is_empty() {
                    "".to_string()
                } else {
                    format!("{}/", ns)
                }),
            md.topic
        );

        let att_receiver = self.reactor.register_listener(att_topic.clone()).await;

        // let struct_query = self.reactor.session.get(att_topic).await.unwrap();

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(StringAttribute::new(
            self.reactor.session.clone(),
            cmd_topic,
            att_topic,
            md.mode.clone(),
            att_receiver,
        )
        .await)
    }

    pub async fn expect_bytes(&self) -> Result<BytesAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!(
            "{}{}/att",
            self.reactor
                .namespace
                .clone()
                .map_or("".to_string(), |ns| if ns.is_empty() {
                    "".to_string()
                } else {
                    format!("{}/", ns)
                }),
            md.topic
        );
        let cmd_topic = format!(
            "{}{}/cmd",
            self.reactor
                .namespace
                .clone()
                .map_or("".to_string(), |ns| if ns.is_empty() {
                    "".to_string()
                } else {
                    format!("{}/", ns)
                }),
            md.topic
        );

        let att_receiver = self.reactor.register_listener(att_topic.clone()).await;

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
            att_topic,
        )
        .await)
    }

    pub async fn expect_number(&self) -> Result<NumberAttribute, String> {
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
                .map_or("".to_string(), |ns| if ns.is_empty() {
                    "".to_string()
                } else {
                    format!("{}/", ns)
                }),
            md.topic
        );

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

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(JsonAttribute::new(
            self.reactor.session.clone(),
            cmd_topic,
            att_topic,
            md.mode.clone(),
            att_receiver,
        )
        .await)
    }

    pub async fn expect_si(&self) -> Result<SiAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!(
            "{}{}/att",
            self.reactor
                .namespace
                .clone()
                .map_or("".to_string(), |ns| if ns.is_empty() {
                    "".to_string()
                } else {
                    format!("{}/", ns)
                }),
            md.topic
        );
        let cmd_topic = format!(
            "{}{}/cmd",
            self.reactor
                .namespace
                .clone()
                .map_or("".to_string(), |ns| if ns.is_empty() {
                    "".to_string()
                } else {
                    format!("{}/", ns)
                }),
            md.topic
        );

        println!("att_topic: {}", att_topic);

        let att_receiver = self.reactor.register_listener(att_topic.clone()).await;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic.clone())
            .await
            .map_err(|e| {
                println!("Failed to register publisher: {}", e);
                e.to_string()
            })?;

        Ok(SiAttribute::new(
            self.reactor.session.clone(),
            cmd_topic,
            att_topic,
            md.mode.clone(),
            att_receiver,
        )
        .await)
    }

    pub async fn expect_enum(&self) -> Result<StringAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!(
            "{}{}/att",
            self.reactor
                .namespace
                .clone()
                .map_or("".to_string(), |ns| if ns.is_empty() {
                    "".to_string()
                } else {
                    format!("{}/", ns)
                }),
            md.topic
        );
        let cmd_topic = format!(
            "{}{}/cmd",
            self.reactor
                .namespace
                .clone()
                .map_or("".to_string(), |ns| if ns.is_empty() {
                    "".to_string()
                } else {
                    format!("{}/", ns)
                }),
            md.topic
        );

        let att_receiver = self.reactor.register_listener(att_topic.clone()).await;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(StringAttribute::new(
            self.reactor.session.clone(),
            cmd_topic,
            att_topic,
            md.mode.clone(),
            att_receiver,
        )
        .await)
    }

    pub async fn expect_status(&self) -> Result<StatusAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic.clone()).await;

        Ok(StatusAttribute::new(self.reactor.session.clone(), att_topic, att_receiver).await)
    }

    pub async fn expect_notification(&self) -> Result<NotificationAttribute, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        let att_receiver = self.reactor.register_listener(att_topic).await;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(NotificationAttribute::new(
            self.reactor.session.clone(),
            md.topic.clone(),
            md.mode.clone(),
            att_receiver,
        )
        .await)
    }
}
