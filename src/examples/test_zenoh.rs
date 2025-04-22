use panduza::{pubsub_zenoh::new_connection, pubsub_zenoh::Options};

#[tokio::main]
async fn main() {
    let session = new_connection().await;

    let _ = session.put("key/expression", "allo").await.unwrap();
    println!("Published data");
}
