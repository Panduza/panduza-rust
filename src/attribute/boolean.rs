// use crate::pubsub::Publisher;
use super::{CallbackEntry, CallbackId};
use crate::fbs::BooleanBuffer;
use crate::AttributeMetadata;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use zenoh::Session;

#[derive(Clone, Debug)]
/// Object to manage the BooleanAttribute
///
pub struct BooleanAttribute {
    /// Global Session
    ///
    session: Session,

    /// Metadata for the attribute
    ///
    metadata: AttributeMetadata,

    /// Callbacks storage
    ///
    callbacks: Arc<Mutex<HashMap<CallbackId, CallbackEntry<BooleanBuffer>>>>,

    /// Next callback ID
    ///
    next_callback_id: Arc<Mutex<CallbackId>>,

    /// Command topic
    ///
    cmd_topic: String,

    /// Last received value
    ///
    last_value: Arc<Mutex<Option<BooleanBuffer>>>,
}

impl BooleanAttribute {
    /// Create a new instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        // Initialize callbacks storage
        let callbacks = Arc::new(Mutex::new(HashMap::<
            CallbackId,
            CallbackEntry<BooleanBuffer>,
        >::new()));

        // Trigger the callback mecanism on message reception
        let att_topic = format!("{}/att", &metadata.topic);
        let subscriber = session.declare_subscriber(att_topic).await.unwrap();
        let last_value = Arc::new(Mutex::new(None));
        tokio::spawn({
            let callbacks = callbacks.clone();
            let last_value = last_value.clone();
            async move {
                while let Ok(sample) = subscriber.recv_async().await {
                    // Create BooleanBuffer from the received zbytes
                    let boolean_buffer = BooleanBuffer::from_zbytes(sample.payload().clone());

                    // Met à jour la dernière valeur reçue
                    {
                        let mut last = last_value.lock().unwrap();
                        *last = Some(boolean_buffer.clone());
                    }

                    // Trigger all callbacks
                    let callbacks_map = callbacks.lock().unwrap();
                    for (_id, callback_entry) in callbacks_map.iter() {
                        // Check condition if present
                        let should_trigger = if let Some(condition) = &callback_entry.condition {
                            condition(&boolean_buffer)
                        } else {
                            true
                        };

                        if should_trigger {
                            (callback_entry.callback)(&boolean_buffer);
                        }
                    }
                }
            }
        });

        // // Wait for the first message if mode is not readonly
        // if mode != AttributeMode::WriteOnly {
        //     let query = session.get(topic_att.clone()).await.unwrap();
        //     let result = query.recv_async().await.unwrap();
        //     let value: bool = result
        //         .result()
        //         .unwrap()
        //         .payload()
        //         .try_to_string()
        //         .unwrap()
        //         .parse()
        //         .unwrap();
        //     pack.lock().unwrap().push(value);
        // }        //

        // Create the command topic
        let cmd_topic = format!("{}/cmd", &metadata.topic);

        // Return attribute
        Self {
            session: session,
            metadata: metadata,
            callbacks: callbacks,
            next_callback_id: Arc::new(Mutex::new(0)),
            cmd_topic: cmd_topic,
            last_value: last_value,
        }
    }

    /// Send command and do not wait for validation
    ///
    pub async fn shoot(&mut self, value: bool) {
        // Create a BooleanBuffer with the value
        let boolean_buffer = BooleanBuffer::from(value);

        // Convert to bytes for sending
        let pyl = boolean_buffer.to_zbytes();

        // Send the command
        self.session.put(&self.cmd_topic, pyl).await.unwrap();
    }

    // ///
    // ///
    // pub async fn set(&mut self, value: bool) -> Result<(), String> {
    //     //
    //     self.shoot(value).await;

    //     if self.mode == AttributeMode::ReadWrite {
    //         let delay = Duration::from_secs(5);

    //         // Wait for change in the data pack
    //         timeout(delay, self.update_notifier.notified())
    //             .await
    //             .map_err(|e| e.to_string())?;

    //         while value != self.get().unwrap() {
    //             // append 3 retry before failling if update received but not good
    //             timeout(delay, self.update_notifier.notified())
    //                 .await
    //                 .map_err(|e| e.to_string())?;
    //         }
    //     }

    //     Ok(())
    // }

    /// Get the last received value
    ///
    pub fn get(&self) -> Option<BooleanBuffer> {
        self.last_value.lock().unwrap().clone()
    }

    /// Wait for a specific boolean value to be received
    ///
    pub async fn wait_for_value(&self, value: bool) -> Result<(), String> {
        use tokio::sync::oneshot;

        // Create a oneshot channel to signal when the value is received
        let (sender, receiver) = oneshot::channel();
        let sender_wrapped = Arc::new(Mutex::new(Some(sender)));

        // Clone the sender for use in the callback closure
        let sender_for_callback = sender_wrapped.clone();

        // Add a callback with condition to wait for the specific value
        let callback_id = self.add_callback_with_condition(
            move |_buffer| {
                // Send signal when the expected value is received
                if let Some(sender) = sender_for_callback.lock().unwrap().take() {
                    let _ = sender.send(());
                }
            },
            move |buffer| {
                // Condition: check if the received value matches the expected value
                buffer.value() == value
            },
        );

        // Wait for the signal with a timeout
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(10), // 10 seconds timeout
            receiver,
        )
        .await;

        // Remove the callback
        self.remove_callback(callback_id);

        match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(_)) => Err("Channel closed unexpectedly".to_string()),
            Err(_) => Err(format!("Timeout waiting for boolean value: {}", value)),
        }
    }

    /// Add a callback that will be triggered when receiving BooleanBuffer messages
    ///
    pub fn add_callback<F>(&self, callback: F) -> CallbackId
    where
        F: Fn(&BooleanBuffer) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        let mut next_id = self.next_callback_id.lock().unwrap();

        let callback_id = *next_id;
        *next_id += 1;

        let callback_entry = CallbackEntry {
            callback: Box::new(callback),
            condition: None,
        };

        callbacks.insert(callback_id, callback_entry);
        callback_id
    }

    /// Add a callback with a condition that will be triggered when receiving BooleanBuffer messages
    ///
    pub fn add_callback_with_condition<F, C>(&self, callback: F, condition: C) -> CallbackId
    where
        F: Fn(&BooleanBuffer) + Send + Sync + 'static,
        C: Fn(&BooleanBuffer) -> bool + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        let mut next_id = self.next_callback_id.lock().unwrap();

        let callback_id = *next_id;
        *next_id += 1;

        let callback_entry = CallbackEntry {
            callback: Box::new(callback),
            condition: Some(Box::new(condition)),
        };

        callbacks.insert(callback_id, callback_entry);
        callback_id
    }

    /// Remove a callback by its ID
    ///
    pub fn remove_callback(&self, callback_id: CallbackId) -> bool {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.remove(&callback_id).is_some()
    }

    /// Clear all callbacks
    ///
    pub fn clear_callbacks(&self) {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.clear();
    }

    /// Get the number of registered callbacks
    ///
    pub fn callback_count(&self) -> usize {
        let callbacks = self.callbacks.lock().unwrap();
        callbacks.len()
    }
}
