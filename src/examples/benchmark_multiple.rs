use futures::future::join_all;
use panduza::{reactor::ReactorOptions, task_monitor::TaskMonitor};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let ip = "192.168.140.13";
    let port = 1883;

    let options = ReactorOptions::new(ip, port);
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    /// change mount number in plugin-vi/tester/device.rs
    let number_of_classes = 5;

    // Create multiple benchmark byte attributes
    let mut benchmark_bytes_list = vec![];
    for i in 1..=number_of_classes {
        let bytes = reactor
            .find_attribute(&format!("bytes_{}/wo", i))
            .expect_bytes()
            .await
            .unwrap();
        benchmark_bytes_list.push(bytes);
    }

    let start = Instant::now();

    let total = 10;

    let megaB = 1;
    let kB = 1;
    let bytes = 1;
    let size = bytes * kB * megaB;
    let mut data = vec![0; size];

    for i in 0..total {
        println!("Iteration {:?}", i);
        // Send data to all bytes attributes concurrently
        let futures: Vec<_> = benchmark_bytes_list
            .iter_mut()
            .map(|bytes| bytes.shoot(data.clone().into()))
            .collect();
        join_all(futures).await;
    }

    let elapsed = start.elapsed();

    // Print the number of messages sent
    println!("Number of messages : {:?}", total);
    // Print the size of a message
    let (msg_size, msg_unit) = if size >= 1_048_576 {
        (size as f32 / 1_048_576.0, "MB")
    } else if size >= 1_024 {
        (size as f32 / 1_024.0, "kB")
    } else {
        (size as f32, "B")
    };
    println!("Size of a message : {:.2} {}", msg_size, msg_unit);
    // Print the time elapsed
    println!("Time elapsed : {:.3?}", elapsed);
    // Print the efficiency combining all atributes
    let total_bytes: f32 = size as f32 * total as f32;
    let bytes_per_sec = total_bytes as f32 / elapsed.as_secs_f32();
    let (efficiency, unit) = if bytes_per_sec >= 1_048_576.0 {
        (bytes_per_sec / 1_048_576.0, "MB/s")
    } else if bytes_per_sec >= 1_024.0 {
        (bytes_per_sec / 1_024.0, "kB/s")
    } else {
        (bytes_per_sec, "B/s")
    };
    println!("Efficiency : {:.2} {}", efficiency, unit);

    sleep(Duration::from_secs(60)).await;
}
