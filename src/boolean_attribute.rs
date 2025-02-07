use std::sync::{Arc, Mutex};

use bytes::Bytes;
use serde_json::Value as JsonValue;

use crate::asyncv::reactor::DataReceiver;

#[derive(Clone, Debug)]
/// Object to manage the BooleanAttribute
///
pub struct BooleanAttribute {
    /// initial data
    ///
    value: Arc<Mutex<bool>>,
}

impl BooleanAttribute {
    pub fn new(mut data_receiver: DataReceiver) -> Self {
        let b_value = Arc::new(Mutex::new(false));

        let json_value_2 = b_value.clone();
        tokio::spawn(async move {
            loop {
                let message = data_receiver.recv().await;
                println!("!!!!!!!!!!! ssss ttt Notification = {:?}", message);

                if let Some(message) = message {
                    match json_value_2.lock() {
                        Ok(mut b_value) => {
                            *b_value = serde_json::from_slice(&message).unwrap();
                        }
                        Err(e) => {
                            println!("Error = {:?}", e);
                        }
                    }
                }
            }
        });

        BooleanAttribute { value: b_value }
    }

    // pub fn update(&mut self, b_value: JsonValue) {
    //     self.b_value = b_value;
    // }

    // pub async fn run(&mut self) {

    // }
}
