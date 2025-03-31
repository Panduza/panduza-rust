use panduza::{reactor::ReactorOptions, task_monitor::TaskMonitor};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let options = ReactorOptions::new();
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    let mut benchmark_bytes = reactor
        .find_attribute("bytes_1/wo")
        .expect_bytes()
        .await
        .unwrap();

    let start = Instant::now();

    let total = 1000;

    let megaB = 1;
    let kB = 1024;
    let bytes = 500;
    let size = bytes * kB * megaB;
    let mut data: Vec<u8> = vec![0; size];

    for i in 0..total {
        // println!("Iteration {:?}", i);
        benchmark_bytes.shoot(data.clone().into()).await;
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
    // Print the average time
    println!("Average time : {:.2?}", elapsed / total);
    // Print the efficiency
    let bytes_per_sec = size as f32 * total as f32 / elapsed.as_secs_f32();
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
