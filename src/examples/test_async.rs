use std::sync::Arc;

use async_trait::async_trait;

use panduza::asyncv::attribute::message::boolean::attribute::AttributePayloadManager;
use panduza::asyncv::attribute::message::AttributeId;
use panduza::asyncv::attribute::message::OnBooleanMessage;
use panduza::asyncv::Reactor;
use panduza::ReactorSettings;
use tokio::time::sleep;
use tokio::time::Duration;

use tokio::sync::Mutex;

#[derive(Copy, Clone, PartialEq)]
struct BooleanPayload {
    value: bool,
}

impl Into<BooleanPayload> for bool {
    fn into(self) -> BooleanPayload {
        todo!()
    }
}

impl From<Vec<u8>> for BooleanPayload {
    fn from(value: Vec<u8>) -> Self {
        todo!()
    }
}
impl Into<Vec<u8>> for BooleanPayload {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}
impl AttributePayloadManager for BooleanPayload {}

#[tokio::main]
async fn main() {
    let settings = ReactorSettings::new("localhost", 1883);
    let mut reactor = Reactor::new(settings);

    reactor.start();

    // // wait for connection

    // // sleep(time::Duration::from_secs(5));

    // println!("-----------");

    // reactor.scan_platforms();

    let pp = reactor
        .create_new_attribute()
        .with_topic("test")
        // .control_config (exemple pour la suite)
        .build_with_payload_type::<BooleanPayload>();

    println!("send data");
    pp.set(true).await.unwrap();

    sleep(Duration::from_secs(60)).await;
}
