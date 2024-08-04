use panduza::asyncv::Reactor;
use panduza::BooleanMessage;
use panduza::ReactorSettings;
use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let settings = ReactorSettings::new("localhost", 1883);
    let mut reactor = Reactor::new(settings);

    reactor.start();

    let pp = reactor
        .create_new_attribute()
        .with_topic("test")
        // .control_config (exemple pour la suite)
        .build_with_message_type::<BooleanMessage>();

    println!("send data");
    pp.set(true).await.unwrap();

    let pp2 = pp.clone();
    pp.when_change(async move {
        println!("cooucou");
        let _dat = pp2.get().await.unwrap();
        println!("cooucou");
    });

    sleep(Duration::from_secs(60)).await;
}
