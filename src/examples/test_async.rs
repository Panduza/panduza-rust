use panduza::asyncv::Reactor;
use panduza::ReactorSettings;
use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let settings = ReactorSettings::new("localhost", 1883);
    let reactor = Reactor::new(settings);

    // reactor.run_in_thread();

    // // wait for connection

    // // sleep(time::Duration::from_secs(5));

    // println!("-----------");

    // reactor.scan_platforms();

    let pp = reactor
        .create_new_attribute()
        .with_topic("test")
        .with_type_boolean()
        .finish();

    // pp.set(true).await.unwrap();

    sleep(Duration::from_secs(60)).await;
}
