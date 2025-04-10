use futures::future::join_all;
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

    let options = ReactorOptions::new(config.ip, config.port);
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    // Create multiple benchmark byte attributes
    let mut benchmark_bytes_list = vec![];

    let mut counter_read_list = vec![];

    let mut counter_reset_list = vec![];

    // change mount number in plugin-vi/tester/device.rs
    for i in 1..=config.number_attributes {
        let bytes = reactor
            .find_attribute(&format!("bytes_{}/wo", i))
            .expect_bytes()
            .await
            .unwrap();
        benchmark_bytes_list.push(bytes);

        let mut counter_read = reactor
            .find_attribute(&format!("bytes_{}/counter_ro", i))
            .expect_number()
            .await
            .unwrap();
        counter_read_list.push(counter_read);

        let mut counter_reset = reactor
            .find_attribute(&format!("bytes_{}/counter_reset", i))
            .expect_boolean()
            .await
            .unwrap();
        counter_reset_list.push(counter_reset);
    }

    let total = config.tests.total_messages;

    let size = config.tests.get_total_size();
    let mut data: Vec<u8> = vec![0; size];

    let start = Instant::now();

    let futures: Vec<_> = counter_reset_list
        .iter_mut()
        .map(|counter| counter.set(false))
        .collect();
    join_all(futures).await;

    for i in 0..total {
        // println!("Iteration {:?}", i);
        // Send data to all bytes attributes concurrently
        let futures: Vec<_> = benchmark_bytes_list
            .iter_mut()
            .map(|bytes| bytes.shoot(data.clone().into()))
            .collect();
        join_all(futures).await;
    }

    let elapsed = start.elapsed();

    let mut prev_timers = vec![0; counter_read_list.len()];

    loop {
        for (index, counter_read) in counter_read_list.iter_mut().enumerate() {
            if let Some(timer) = counter_read.get() {
                if timer != prev_timers[index] {
                    println!("Reception time for bytes_{}: {} ms", index + 1, timer);
                    prev_timers[index] = timer;
                }
            }
        }
        if prev_timers.iter().all(|&timer| timer > 0) {
            break;
        }
        sleep(Duration::from_millis(100)).await;
    }

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

    // sleep(Duration::from_secs(60)).await;
}
