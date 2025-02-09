use rumqttc::QoS;

use crate::{
    attribute_metadata::AttributeMetadata,
    pubsub::{PubSubOperator, Publisher},
    BooleanAttribute, Reactor,
};

#[derive(Clone)]
/// Metadata for an attribute
///
pub struct AttributeBuilder<O: PubSubOperator> {
    ///
    ///
    reactor: Reactor<O>,
    ///
    ///
    metadata: Option<AttributeMetadata>,
}

impl<O: PubSubOperator> AttributeBuilder<O> {
    ///
    ///
    pub fn new(reactor: Reactor<O>, metadata: Option<AttributeMetadata>) -> Self {
        Self {
            reactor: reactor,
            metadata: metadata,
        }
    }

    pub async fn expect_boolean<P: Publisher>(&self) -> Result<BooleanAttribute<P>, String> {
        let md = self.metadata.as_ref().unwrap();
        let att_topic = format!("{}/att", md.topic);
        let cmd_topic = format!("{}/cmd", md.topic);

        // self.reactor
        //     .message_client
        //     .subscribe(att_topic.clone(), QoS::AtLeastOnce)
        //     .await
        //     .unwrap();

        let att_receiver = self.reactor.register_listener(att_topic, 20).await?;

        let cmd_publisher = self
            .reactor
            .register_publisher(cmd_topic, false)
            .map_err(|e| e.to_string())?;

        Ok(BooleanAttribute::new(cmd_publisher, att_receiver))
    }
}
