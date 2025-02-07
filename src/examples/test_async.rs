use panduza::asyncv::Reactor;
use panduza::BooleanMessage;
use panduza::ReactorSettings;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let settings = ReactorSettings::new("127.0.0.1", 1883);
    let mut reactor = Reactor::start(settings).await.unwrap();

    let mut pp = reactor
        .find_attribute("truc_1")
        .expect_boolean()
        .await
        .unwrap();
    // println!("$$$$$$ {:?}", pp);

    let mut vvv = true;
    for _ in 0..1000 {
        vvv = if vvv { true } else { false };
        pp.set(vvv).await;
    }

    // Print the elapsed time
    println!("Time elapsed: {:?}", start.elapsed() / 1000);
    // let ro_bool = reactor
    //     .create_new_attribute()
    //     .with_topic("test")
    //     .with_ro_access()
    //     .finish_with_message_type::<BooleanMessage>()
    //     .await;

    // let rw_bool = reactor
    //     .create_new_attribute()
    //     .with_topic("o")
    //     .with_rw_access()
    //     .finish_with_message_type::<BooleanMessage>()
    //     .await;

    // rw_bool.set(true).await.unwrap();

    // // Wait then execute the function once
    // let ro_bool_bis = ro_bool.clone();
    // ro_bool
    //     .wait_change_then(async move {
    //         println!("cooucou");
    //         let _dat = ro_bool_bis.get().await.unwrap();
    //         println!("cooucou {} ", _dat);
    //     })
    //     .await;

    // // Task that run an action every time the value of the attribute change
    // tokio::spawn(async move {
    //     loop {
    //         let ro_bool_bis = ro_bool.clone();
    //         ro_bool
    //             .wait_change_then(async move {
    //                 println!("cooucou");
    //                 let _dat = ro_bool_bis.get().await.unwrap();
    //                 println!("cooucou {} ", _dat);
    //             })
    //             .await;
    //     }
    // });

    // // Task that run an action every time the value of the attribute change
    // tokio::spawn(async move {
    //     loop {
    //         rw_bool
    //             .wait_change_then(async move {
    //                 println!("cooucou depuis o");
    //             })
    //             .await;
    //     }
    // });

    sleep(Duration::from_secs(60)).await;
}
