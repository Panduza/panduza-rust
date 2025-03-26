use panduza::{benchmark_config::BenchmarkConfig, reactor::ReactorOptions};
use std::fs::read_to_string;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    //
    // Read JSON file
    let config_str = read_to_string("benchmark_config.json").expect("Failed to read the file");
    let config: BenchmarkConfig = serde_json::from_str(&config_str).expect("Failed parsing JSON");

    let options = ReactorOptions::new(config.ip, config.port, config.ca_certificate);
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    let mut benchmark_bytes = reactor
        .find_attribute("bytes_1/wo")
        .expect_bytes()
        .await
        .unwrap();

    let mut counter_read = reactor
        .find_attribute("bytes_1/counter_ro")
        .expect_number()
        .await
        .unwrap();

    let mut counter_reset = reactor
        .find_attribute("bytes_1/counter_reset")
        .expect_boolean()
        .await
        .unwrap();

    let size = config.tests.get_total_size();
    let mut data: Vec<u8> = vec![0; size];

    // start sending timer
    let start = Instant::now();

    // reset the reception timer
    counter_reset.set(false).await;

    for _ in 0..config.tests.total_messages {
        benchmark_bytes.shoot(data.clone().into()).await;
    }

    let elapsed = start.elapsed();

    // get reception time
    let mut prev_timer = 0;
    loop {
        if let Some(timer) = counter_read.get() {
            if timer != prev_timer {
                println!("Reception time: {} ms", timer);
                prev_timer = timer;
                break;
            }
        }
        sleep(Duration::from_millis(100)).await;
    }

    // Print the number of messages sent
    println!("Number of messages : {:?}", config.tests.total_messages);
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
    println!(
        "Average time : {:.2?}",
        elapsed / config.tests.total_messages as u32
    );
    // Print the efficiency
    let bytes_per_sec = size as f32 * config.tests.total_messages as f32 / elapsed.as_secs_f32();
    let (efficiency, unit) = if bytes_per_sec >= 1_048_576.0 {
        (bytes_per_sec / 1_048_576.0, "MB/s")
    } else if bytes_per_sec >= 1_024.0 {
        (bytes_per_sec / 1_024.0, "kB/s")
    } else {
        (bytes_per_sec, "B/s")
    };
    println!("Bitrate : {:.2} {}", efficiency, unit);

    sleep(Duration::from_secs(1)).await;
}
