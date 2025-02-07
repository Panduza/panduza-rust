use std::sync::{Arc, Mutex};

use bytes::Bytes;
use serde_json::Value as JsonValue;
use tokio::sync::Notify;

use crate::asyncv::{reactor::DataReceiver, MessageClient};

#[derive(Clone, Debug)]
/// Object to manage the BooleanAttribute
///
pub struct BooleanAttribute {
    cmd_topic: String,
    ///
    message_client: MessageClient,

    /// initial data
    ///
    value: Arc<Mutex<bool>>,

    update: Arc<Notify>,
}

impl BooleanAttribute {
    pub fn new(
        topic: String,
        message_client: MessageClient,
        mut data_receiver: DataReceiver,
    ) -> Self {
        let b_value = Arc::new(Mutex::new(false));

        let update = Arc::new(Notify::new());

        let update2 = update.clone();
        let json_value_2 = b_value.clone();
        tokio::spawn(async move {
            loop {
                let message = data_receiver.recv().await;
                // println!("!!!!!!!!!!! BOOLEAN ttt Notification = {:?}", message);

                if let Some(message) = message {
                    match json_value_2.lock() {
                        Ok(mut b_value) => {
                            *b_value = serde_json::from_slice(&message).unwrap();
                            update2.notify_waiters();
                        }
                        Err(e) => {
                            println!("Error = {:?}", e);
                        }
                    }
                }
            }
        });

        BooleanAttribute {
            cmd_topic: format!("{}/cmd", topic),
            message_client: message_client,
            value: b_value,
            update: update,
        }
    }

    ///
    ///
    pub async fn set(&mut self, value: bool) {
        let pyl = Bytes::from(serde_json::to_string(&value).unwrap());

        self.message_client
            .publish_bytes(&self.cmd_topic, rumqttc::QoS::AtMostOnce, false, pyl)
            .await
            .unwrap();

        self.update.notified().await;
    }
}
