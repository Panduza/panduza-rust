use panduza::{reactor::ReactorOptions, task_monitor::TaskMonitor};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let options = ReactorOptions::new();
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    let mut benchmark_string = reactor
        .find_attribute("string/rw")
        .expect_string()
        .await
        .unwrap();
    // println!("$$$$$$ {:?}", pp);

    let start = Instant::now();

    let total = 1000;
    let mut string = "Amet sunt cillum incididunt irure incididunt adipisicing. Dolore sint velit ipsum esse ea pariatur proident nisi qui proident adipisicing aliqua consectetur dolor. Quis veniam eu duis fugiat veniam dolor laborum ex ipsum. Sunt nostrud deserunt qui cillum cupidatat veniam sunt. Eu occaecat aliqua esse dolore nisi eu ea ad minim commodo irure sint anim. Nisi magna qui velit in anim sunt eu consectetur amet non. Duis incididunt reprehenderit ipsum ipsum.";
    let mut vvv = true;
    for i in 0..total {
        // println!("POK {:?}", i);
        vvv = if vvv { true } else { false };
        benchmark_string.set(string.to_string()).await;
    }

    // let mut benchmark_boolean = reactor
    //     .find_attribute("boolean/rw")
    //     .expect_boolean()
    //     .await
    //     .unwrap();
    // // println!("$$$$$$ {:?}", pp);

    // let start = Instant::now();

    // let total = 1000;
    // let mut vvv = true;
    // for i in 0..total {
    //     // println!("POK {:?}", i);
    //     vvv = if vvv { true } else { false };
    //     benchmark_boolean.set(vvv).await;
    // }

    // let to = tokio::spawn(async move {
    //     for i in 0..2 {
    //         tokio::time::sleep(Duration::from_millis(1000)).await;
    //         println!("oooo");
    //     }
    //     Ok(())
    // });

    // let (monitor, event_receiver) = TaskMonitor::new();

    // monitor.handle_sender().send(to).await.unwrap();

    // What if we create an other attribute on the same topic ?
    //      need to multiplexer

    // What if we delete the attribute ?
    //      need to cleanup

    // Is tokio spawn ok ? because it won't catch error ?
    //      this would force me to

    // Print the elapsed time
    println!("Perf : {:?}", start.elapsed() / total);
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
