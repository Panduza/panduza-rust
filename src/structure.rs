use std::sync::{Arc, Mutex};

use bytes::Bytes;
use serde_json::Value as JsonValue;

use crate::asyncv::reactor::DataReceiver;

///
///
pub struct Structure {
    /// initial data
    ///
    json_value: Arc<Mutex<JsonValue>>,
}

impl Structure {
    pub fn new(mut data_receiver: DataReceiver) -> Self {
        let json_value = Arc::new(Mutex::new(JsonValue::Null));

        let json_value_2 = json_value.clone();
        tokio::spawn(async move {
            loop {
                let message = data_receiver.recv().await;
                println!("Notification = {:?}", message);

                if let Some(message) = message {
                    match json_value_2.lock() {
                        Ok(mut json_value) => {
                            *json_value = serde_json::from_slice(&message).unwrap();
                        }
                        Err(e) => {
                            println!("Error = {:?}", e);
                        }
                    }
                }
            }
        });

        Structure {
            json_value: json_value,
        }
    }

    // pub fn update(&mut self, json_value: JsonValue) {
    //     self.json_value = json_value;
    // }

    // pub async fn run(&mut self) {

    // }
}
