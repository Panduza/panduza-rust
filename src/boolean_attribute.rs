use std::sync::{Arc, Mutex};

use bytes::Bytes;
use tokio::sync::Notify;

use crate::{pubsub::Publisher, reactor::DataReceiver};

#[derive(Clone, Debug)]
/// Object to manage the BooleanAttribute
///
pub struct BooleanAttribute<P: Publisher> {
    // cmd_topic: String,
    ///
    ///
    cmd_publisher: P,

    /// initial data
    ///
    value: Arc<Mutex<bool>>,

    update: Arc<Notify>,
}

impl<P: Publisher> BooleanAttribute<P> {
    ///
    ///
    pub fn new(cmd_publisher: P, mut att_receiver: DataReceiver) -> Self {
        let b_value = Arc::new(Mutex::new(false));

        let update = Arc::new(Notify::new());

        let update2 = update.clone();
        let json_value_2 = b_value.clone();
        tokio::spawn(async move {
            loop {
                let message = att_receiver.recv().await;
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
            // cmd_topic: format!("{}/cmd", topic),
            // operator: operator,
            cmd_publisher: cmd_publisher,
            value: b_value,
            update: update,
        }
    }

    ///
    ///
    pub async fn set(&mut self, value: bool) {
        let pyl = Bytes::from(serde_json::to_string(&value).unwrap());

        self.cmd_publisher.publish(pyl).await.unwrap();

        self.update.notified().await;
    }
}
