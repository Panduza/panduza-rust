use bytes::Bytes;
use colored::*;
use panduza::{reactor::ReactorOptions, NotificationAttribute, StatusAttribute};
use std::io;

// --- TEST PARAMETERS ---
const PLAFORM_LOCALHOST: &str = "localhost";
const PLAFORM_PORT: u16 = 1883;
// -----------------------

#[tokio::main]
async fn main() {
    let options = ReactorOptions::new(PLAFORM_LOCALHOST, PLAFORM_PORT);
    let reactor = panduza::new_reactor(options).await.unwrap();

    let mut platform_status = Some(reactor.new_status_attribute().await);

    let mut platform_notifications = Some(reactor.new_notification_attribute().await);

    let attribute_builder_tx: panduza::AttributeBuilder = reactor
        .find_attribute("serial-stream/TX")
        .expect("Attribute not found");
    let mut serial_stream_tx = attribute_builder_tx.expect_bytes().await.unwrap();

    let attribute_builder_rx: panduza::AttributeBuilder = reactor
        .find_attribute("serial-stream/RX")
        .expect("Attribute not found");
    let serial_stream_rx = attribute_builder_rx.expect_bytes().await.unwrap();

    // Tâche de réception avec pop()
    tokio::spawn(async move {
        let mut last_message_received = Bytes::new();
        loop {
            if let Some(received) = serial_stream_rx.get() {
                if received != last_message_received {
                    println!(
                        "{} {}",
                        "[message received]".green().bold(),
                        format!("{:?}", String::from_utf8_lossy(&received))
                    );
                }
                last_message_received = received;
            }
        }
    });

    // Tâche de réception des alertes
    tokio::spawn(async move {
        loop {
            if platform_notifications
                .as_mut()
                .unwrap()
                .pop_all()
                .has_alert()
                == true
            {
                println!(
                    "{} {}",
                    "[notification alert]".yellow().bold(),
                    " a notification alert has been detected"
                );
            }
        }
    });

    // Tâche de réception des erreurs
    tokio::spawn(async move {
        loop {
            if platform_status
                .as_mut()
                .unwrap()
                .at_least_one_instance_is_not_running()
                .expect("Error while checking if at least one instance is not running")
                == true
            {
                println!(
                    "{} {}",
                    "[status error]".red().bold(),
                    " at least one instance is not running"
                );
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
            let bytes = bytes::Bytes::from(input.as_bytes().to_vec());
            serial_stream_tx.set(bytes).await.unwrap();
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

    //sleep(Duration::from_secs(60)).await;
}
