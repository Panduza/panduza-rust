use std::sync::Arc;

use async_trait::async_trait;

use panduza::asyncv::attribute::message::AttributeId;
use panduza::asyncv::attribute::message::OnBooleanMessage;
use panduza::asyncv::Reactor;
use panduza::ReactorSettings;
use tokio::time::sleep;
use tokio::time::Duration;

use tokio::sync::Mutex;

struct TestBehaviour {}

impl TestBehaviour {
    pub fn to_arc_mutex(self) -> Arc<Mutex<TestBehaviour>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl OnBooleanMessage for TestBehaviour {
    async fn on_message_boolean(&mut self, id: AttributeId, data: bool) {}
}

#[tokio::main]
async fn main() {
    let settings = ReactorSettings::new("localhost", 1883);
    let mut reactor = Reactor::new(settings);

    reactor.start();

    // // wait for connection

    // // sleep(time::Duration::from_secs(5));

    // println!("-----------");

    // reactor.scan_platforms();

    let lll = TestBehaviour {}.to_arc_mutex();

    let pp = reactor
        .create_new_attribute()
        .with_topic("test")
        .with_type_boolean()
        .finish()
        .with_boolean_message_handler(lll);

    println!("send data");
    pp.set(true).await.unwrap();

    sleep(Duration::from_secs(60)).await;
}
