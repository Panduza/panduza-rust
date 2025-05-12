use panduza::reactor::ReactorOptions;

#[tokio::main]
async fn main() {
    let options = ReactorOptions::new();
    let mut reactor = panduza::new_reactor(options).await.unwrap();

    let mut serial_tx = reactor
        .find_attribute("serial-stream/TX")
        .expect_bytes()
        .await
        .unwrap();

    let mut serial_rx = reactor
        .find_attribute("serial-stream/RX")
        .expect_bytes()
        .await
        .unwrap();
}

