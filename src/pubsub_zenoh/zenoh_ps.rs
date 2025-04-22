use zenoh::{open, Config, Session};

pub async fn new_connection() -> Session {
    let config = Config::from_file("config_zenoh.json5").unwrap();

    println!("{:?}", config);
    let session = open(config).await.unwrap();

    session
}
