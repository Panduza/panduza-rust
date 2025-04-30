use colored::*;
use panduza::{reactor::ReactorOptions, task_monitor::TaskMonitor};
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let options = ReactorOptions::new();
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    let mut pp = reactor
        .find_attribute("serial-stream/TX")
        .expect_string()
        .await
        .unwrap();
    // println!("$$$$$$ {:?}", pp);
    let mut listener = reactor
        .find_attribute("serial-stream/RX")
        .expect_string()
        .await
        .unwrap();

    // Tâche de réception avec pop()
    tokio::spawn(async move {
        let mut last_message_received = String::new();
        loop {
            if let Some(received) = listener.get() {
                if received != last_message_received {
                    println!(
                        "{} {}",
                        "[message received]".green().bold(),
                        format!("{:?}", received)
                    );
                }
                last_message_received = received;
            }
        }
    });

    //let start = Instant::now();
    let stdin: io::Stdin = io::stdin();
    let mut input: String = String::new();
    println!("Write your command ant press enter :");
    loop {
        input.clear(); // important pour ne pas accumuler les lignes précédentes
        stdin.read_line(&mut input).unwrap();

        if !input.is_empty() {
            pp.shoot(input.to_string()).await;
            println!(
                "{} {}",
                "[message send]".blue().bold(),
                format!("{:?}", input)
            );
        }
    }

    /*
    for i in 0..total {
        pp.shoot(format!("message_{}", i)).await;
        println!("{}", i);
    }
    */

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
    //println!("Perf : {:?}", start.elapsed() / total);
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
