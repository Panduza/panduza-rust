use std::sync::Arc;

use async_trait::async_trait;

use panduza::asyncv::attribute::message::attribute::AttributePayloadManager;

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
        return BooleanPayload { value: true };
    }
}

impl From<Vec<u8>> for BooleanPayload {
    fn from(value: Vec<u8>) -> Self {
        return BooleanPayload { value: true };
    }
}
impl Into<Vec<u8>> for BooleanPayload {
    fn into(self) -> Vec<u8> {
        return vec![1];
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

    pp.when_change(async move {
        println!("cooucou");
    });

    sleep(Duration::from_secs(60)).await;
}
