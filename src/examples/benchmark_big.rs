use panduza::{reactor::ReactorOptions, task_monitor::TaskMonitor};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let options = ReactorOptions::new();
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    let mut benchmark_bytes = reactor
        .find_attribute("bytes_1/wo")
        .expect_bytes_publisher()
        .await
        .unwrap();

    let start = Instant::now();

    let total = 1000;

    let kB = 1024;
    let bytes = 1;
    let size = bytes * kB;
    let mut data = vec![0; size];

    for i in 0..total {
        // println!("Iteration {:?}", i);
        benchmark_bytes.shoot(data.clone().into()).await;
    }

    let elapsed = start.elapsed();

    // Print the average time
    println!("Average speed : {:?}", elapsed / total);
    // Print the efficiency
    let bytes_per_sec = size as f32 * total as f32 / elapsed.as_secs_f32();
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
