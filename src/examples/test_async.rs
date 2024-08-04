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

    let ro_bool = reactor
        .create_new_attribute()
        .with_topic("test")
        .with_ro_access()
        .finish_with_message_type::<BooleanMessage>()
        .await;

    // Wait then execute the function once
    let ro_bool_bis = ro_bool.clone();
    ro_bool
        .wait_change_then(async move {
            println!("cooucou");
            let _dat = ro_bool_bis.get().await.unwrap();
            println!("cooucou {} ", _dat);
        })
        .await;

    // Task that run an action every time the value of the attribute change
    tokio::spawn(async move {
        loop {
            let ro_bool_bis = ro_bool.clone();
            ro_bool
                .wait_change_then(async move {
                    println!("cooucou");
                    let _dat = ro_bool_bis.get().await.unwrap();
                    println!("cooucou {} ", _dat);
                })
                .await;
        }
    });

    sleep(Duration::from_secs(60)).await;
}
