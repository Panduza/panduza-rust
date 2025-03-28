use panduza::{reactor::ReactorOptions, task_monitor::TaskMonitor};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let options = ReactorOptions::new();
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    let number_of_classes = 1;

    // Create multiple benchmark byte attributes
    let mut benchmark_bytes_list = vec![];
    for i in 1..=number_of_classes {
        let bytes = reactor
            .find_attribute(&format!("bytes_{}/wo", i))
            .expect_bytes_publisher()
            .await
            .unwrap();
        benchmark_bytes_list.push(bytes);
    }

    let start = Instant::now();

    let total = 1000;

    let kB = 1024;
    let bytes = 100;
    let size = bytes * kB;
    let mut data = vec![0; size];

    for i in 0..total {
        // println!("Iteration {:?}", i);
        // Send data to all bytes attributes concurrently
        let futures: Vec<_> = benchmark_bytes_list
            .iter_mut()
            .map(|bytes| bytes.shoot(data.clone().into()))
            .collect();
        futures::future::join_all(futures).await;
    }

    let elapsed = start.elapsed();

    // Print the average time
    println!("Average speed : {:.2?}", elapsed / total);

    // Print the efficiency combining all atributes
    let total_bytes = size * number_of_classes;
    let bytes_per_sec = total_bytes as f32 * total as f32 / elapsed.as_secs_f32();
    let (efficiency, unit) = if bytes_per_sec > 1_000_000.0 {
        (bytes_per_sec / 1_000_000.0, "MB/s")
    } else if bytes_per_sec > 1_000.0 {
        (bytes_per_sec / 1_000.0, "kB/s")
    } else {
        (bytes_per_sec, "B/s")
    };
    println!("Efficiency : {:.2} {}", efficiency, unit);

    sleep(Duration::from_secs(60)).await;
}
