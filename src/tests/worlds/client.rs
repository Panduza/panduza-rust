use cucumber::{given, then, when, World};
use panduza::{reactor::ReactorOptions, Reactor, StringAttribute};
use std::fmt::Debug;
use tokio::time::{sleep, Duration};

#[derive(Default, World)]
pub struct ClientWorld {
    r: Option<Reactor>,
    response: Option<String>,
    att_rx: Option<StringAttribute>,
    att_tx: Option<StringAttribute>,
    topic_rx: Option<String>,
    topic_tx: Option<String>,
}

impl Debug for ClientWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientWorld")
            // .field("r", &self.r)
            .finish()
    }
}

#[given(expr = "the client is connected to {string} on port {int}")]
async fn given_a_connected_client(world: &mut ClientWorld, hostname: String, port: u16) {
    let options = ReactorOptions::new();
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    world.r = Some(reactor);
}

#[given(expr = "the attribute rx {string}")]
async fn given_the_attribute_rx(world: &mut ClientWorld, attribute_name: String) {
    world.topic_rx = Some(attribute_name.clone());

    let attribute: panduza::StringAttribute = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect_string()
        .await
        .unwrap();

    world.att_rx = Some(attribute);
}

#[given(expr = "the attribute tx {string}")]
async fn given_the_attribute_tx(world: &mut ClientWorld, attribute_name: String) {
    world.topic_tx = Some(attribute_name.clone());

    let attribute: panduza::StringAttribute = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect_string()
        .await
        .unwrap();

    world.att_tx = Some(attribute);
}

#[when(expr = "the client sends {string}")]
async fn send_message(world: &mut ClientWorld, message: String) {
    world
        .att_tx
        .as_mut()
        .unwrap()
        .shoot(message.to_string())
        .await;
    //sleep(Duration::from_secs(5)).await
}

#[then(expr = "the client should receive {string}")]
async fn check_response(world: &mut ClientWorld, expected: String) {
    sleep(Duration::from_secs(5)).await;

    let read_value = world.att_rx.as_mut().unwrap().get().unwrap();
    assert_eq!(
        read_value,
        expected.to_string(),
        "read '{:?}' != expected '{:?}'",
        read_value,
        expected.to_string()
    );
}
