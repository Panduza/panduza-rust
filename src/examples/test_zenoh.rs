use panduza::pubsub::new_connection;
use panduza::reactor::new_reactor;
use panduza::reactor::ReactorOptions;
use panduza::Reactor;
use panduza::{benchmark_config::BenchmarkConfig, pubsub::Options};
use std::fs::read_to_string;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let config_str = read_to_string("benchmark_config.json").expect("Failed to read the file");
    let config: BenchmarkConfig = serde_json::from_str(&config_str).expect("Failed parsing JSON");

    // let options = Options::new(config.ip, config.port, config.ca_certificate);
    // let session = new_connection(options).await.unwrap();

    let options = ReactorOptions::new(config.ip, config.port, config.ca_certificate);

    println!("Options du reactor : {:?}", options);
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    // for i in 0..100 {
    //     println!("POK {:?}", i);
    //     reactor
    //         .session
    //         .put("pza/Attribute Tester/string/rw", "rrrr");
    // }

    let mut benchmark_string = reactor
        .find_attribute("string/rw")
        .expect_string()
        .await
        .unwrap();

    let total = 1000;
    let mut string = "Amet sunt cillum incididunt irure incididunt adipisicing. Dolore sint velit ipsum esse ea pariatur proident nisi qui proident adipisicing aliqua consectetur dolor. Quis veniam eu duis fugiat veniam dolor laborum ex ipsum. Sunt nostrud deserunt qui cillum cupidatat veniam sunt. Eu occaecat aliqua esse dolore nisi eu ea ad minim commodo irure sint anim. Nisi magna qui velit in anim sunt eu consectetur amet non. Duis incididunt reprehenderit ipsum ipsum.";
    for i in 0..total {
        println!("POK {:?}", i);
        benchmark_string.set(string.to_string()).await;
        // reactor.session.put("string/rw", string.to_string()).await;
    }
}
