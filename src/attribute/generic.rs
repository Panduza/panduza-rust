use super::{CallbackEntry, CallbackId};
use crate::fbs::GenericBuffer;
use crate::AttributeMetadata;
use crate::AttributeMode;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use zenoh::Session;

/// Generic attribute implementation that can work with any buffer type that implements GenericBuffer
#[derive(Clone, Debug)]
pub struct GenericAttribute<B: GenericBuffer> {
    /// Global Session
    session: Session,

    /// Metadata for the attribute
    metadata: AttributeMetadata,

    /// Callbacks storage
    callbacks: Arc<Mutex<HashMap<CallbackId, CallbackEntry<B>>>>,

    /// Next callback ID
    next_callback_id: Arc<Mutex<CallbackId>>,

    /// Command topic
    cmd_topic: String,

    /// Last received value
    last_value: Arc<Mutex<Option<B>>>,
}

impl<B: GenericBuffer> GenericAttribute<B> {
    /// Create a new instance
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        // Initialize callbacks storage
        let callbacks = Arc::new(Mutex::new(HashMap::<CallbackId, CallbackEntry<B>>::new()));

        // Trigger the callback mechanism on message reception
        let att_topic = format!("{}/att", &metadata.topic);
        let subscriber = session.declare_subscriber(&att_topic).await.unwrap();
        let last_value = Arc::new(Mutex::new(None));

        tokio::spawn({
            let callbacks = callbacks.clone();
            let last_value = last_value.clone();
            async move {
                while let Ok(sample) = subscriber.recv_async().await {
                    // Create Buffer from the received zbytes
                    let buffer = B::from_zbytes(sample.payload().clone());

                    // Update the last received value
                    {
                        let mut last = last_value.lock().unwrap();
                        *last = Some(buffer.clone());
                    }

                    // Trigger all callbacks
                    let callbacks_map = callbacks.lock().unwrap();
                    for (_id, callback_entry) in callbacks_map.iter() {
                        // Check condition if present
                        let should_trigger = if let Some(condition) = &callback_entry.condition {
                            condition(&buffer)
                        } else {
                            true
                        };

                        if should_trigger {
                            (callback_entry.callback)(&buffer);
                        }
                    }
                }
            }
        });

        // Wait for the first message if mode is not WriteOnly
        if metadata.mode != AttributeMode::WriteOnly {
            let query = session.get(&att_topic).await.unwrap();
            let result = query.recv_async().await.unwrap();
            let buffer = B::from_zbytes(result.result().unwrap().payload().clone());
            let mut last = last_value.lock().unwrap();
            *last = Some(buffer);
        }

        // Create the command topic
        let cmd_topic = format!("{}/cmd", &metadata.topic);

        // Return attribute
        Self {
            session,
            metadata,
            callbacks,
            next_callback_id: Arc::new(Mutex::new(0)),
            cmd_topic,
            last_value,
        }
    }

    /// Send command and do not wait for validation
    pub async fn shoot<T>(&mut self, value: T)
    where
        T: Into<B>,
    {
        // Create a Buffer with the value
        let buffer = T::into(value);

        // Convert to bytes for sending
        let payload = buffer.to_zbytes();

        // Send the command
        self.session.put(&self.cmd_topic, payload).await.unwrap();
    }

    /// Set a value and wait for validation if in ReadWrite mode
    pub async fn set<T>(&mut self, value: T) -> Result<(), String>
    where
        T: Into<B>,
        B: PartialEq,
    {
        let buffer = T::into(value);
        self.shoot_buffer(buffer.clone()).await;

        if self.metadata.mode == AttributeMode::ReadWrite {
            self.wait_for_buffer(buffer, Some(std::time::Duration::from_secs(5)))
                .await
                .map_err(|e| format!("Failed to set value: {}", e))?;
        }

        Ok(())
    }

    /// Send a buffer directly and do not wait for validation
    pub async fn shoot_buffer(&mut self, buffer: B) {
        // Convert to bytes for sending
        let payload = buffer.to_zbytes();

        // Send the command
        self.session.put(&self.cmd_topic, payload).await.unwrap();
    }

    /// Get the last received value
    pub fn get(&self) -> Option<B> {
        self.last_value.lock().unwrap().clone()
    }

    /// Wait for a specific buffer value to be received
    pub async fn wait_for_buffer(
        &self,
        expected_buffer: B,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String>
    where
        B: PartialEq,
    {
        use tokio::sync::oneshot;

        // Create a oneshot channel to signal when the value is received
        let (sender, receiver) = oneshot::channel();
        let sender_wrapped = Arc::new(Mutex::new(Some(sender)));

        // Clone the sender for use in the callback closure
        let sender_for_callback = sender_wrapped.clone();

        // Add a callback with condition to wait for the specific value
        let callback_id = self.add_callback(
            move |_buffer| {
                // Send signal when the expected value is received
                if let Some(sender) = sender_for_callback.lock().unwrap().take() {
                    let _ = sender.send(());
                }
            },
            Some(move |buffer: &B| {
                // Condition: check if the received value matches the expected value
                *buffer == expected_buffer
            }),
        );

        // Wait for the signal with an optional timeout
        let result = if let Some(duration) = timeout {
            tokio::time::timeout(duration, receiver).await
        } else {
            // No timeout: wait indefinitely
            Ok(receiver.await)
        };

        // Remove the callback
        self.remove_callback(callback_id);

        match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(_)) => Err("Channel closed unexpectedly".to_string()),
            Err(_) => Err("Timeout waiting for value".to_string()),
        }
    }

    /// Add a callback that will be triggered when receiving buffer messages
    /// Optionally, a condition can be provided to filter when the callback is triggered
    pub fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(&B) + Send + Sync + 'static,
        C: Fn(&B) -> bool + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        let mut next_id = self.next_callback_id.lock().unwrap();

        let callback_id = *next_id;
        *next_id += 1;

        let callback_entry = CallbackEntry {
            callback: Box::new(callback),
            condition: condition.map(|c| Box::new(c) as Box<dyn Fn(&B) -> bool + Send + Sync>),
        };

        callbacks.insert(callback_id, callback_entry);
        callback_id
    }

    /// Remove a callback by its ID
    pub fn remove_callback(&self, callback_id: CallbackId) -> bool {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.remove(&callback_id).is_some()
    }

    /// Clear all callbacks
    pub fn clear_callbacks(&self) {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.clear();
    }

    /// Get the number of registered callbacks
    pub fn callback_count(&self) -> usize {
        let callbacks = self.callbacks.lock().unwrap();
        callbacks.len()
    }

    /// Get attribute metadata
    pub fn metadata(&self) -> &AttributeMetadata {
        &self.metadata
    }

    /// Get the command topic
    pub fn cmd_topic(&self) -> &str {
        &self.cmd_topic
    }
}

// Type aliases for convenience
pub type BooleanGenericAttribute = GenericAttribute<crate::fbs::BooleanBuffer>;

#[cfg(test)]
mod tests {
    use zenoh::bytes::ZBytes;

    use super::*;

    // Example showing how to implement GenericBuffer for a custom type
    #[derive(Clone, Debug, PartialEq)]
    struct MockBuffer {
        value: String,
    }

    impl GenericBuffer for MockBuffer {
        fn from_zbytes(zbytes: ZBytes) -> Self {
            let value = String::from_utf8(zbytes.to_bytes().to_vec()).unwrap_or_default();
            Self { value }
        }

        fn to_zbytes(&self) -> ZBytes {
            ZBytes::from(self.value.as_bytes().to_vec())
        }

        fn from_value<T>(value: T) -> Self
        where
            T: Into<Self>,
        {
            value.into()
        }
    }

    impl From<String> for MockBuffer {
        fn from(value: String) -> Self {
            Self { value }
        }
    }

    impl From<&str> for MockBuffer {
        fn from(value: &str) -> Self {
            Self {
                value: value.to_string(),
            }
        }
    }
}
