use super::{CallbackEntry, CallbackId};
use crate::fbs::PanduzaBuffer;
use crate::AttributeMetadata;
use crate::AttributeMode;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use zenoh::Session;

/// Generic attribute implementation that can work with any buffer type that implements GenericBuffer
#[derive(Clone, Debug)]
pub struct GenericAttribute<B: PanduzaBuffer> {
    /// Global Session
    session: Session,

    /// Metadata for the attribute
    metadata: AttributeMetadata,

    /// Async callbacks storage
    callbacks: Arc<Mutex<HashMap<CallbackId, CallbackEntry<B>>>>,

    /// Next callback ID
    next_callback_id: Arc<Mutex<CallbackId>>,

    /// Command topic
    cmd_topic: String,

    /// Last received value
    last_value: Arc<Mutex<Option<B>>>,
}

impl<B: PanduzaBuffer> GenericAttribute<B> {
    /// Create a new instance
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        // Initialize async callbacks storage
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
                    let buffer = B::build_from_zbytes(sample.payload().clone());

                    // Update the last received value
                    {
                        let mut last = last_value.lock().await;
                        *last = Some(buffer.clone());
                    }

                    // Trigger all async callbacks
                    let callbacks_map = callbacks.lock().await;
                    let mut futures = Vec::new();

                    for (_id, callback_entry) in callbacks_map.iter() {
                        // Check condition if present
                        let should_trigger = if let Some(condition) = &callback_entry.condition {
                            condition(&buffer)
                        } else {
                            true
                        };

                        if should_trigger {
                            futures.push((callback_entry.callback)(buffer.clone()));
                        }
                    }

                    // Drop the lock before awaiting futures
                    drop(callbacks_map);

                    // Execute all callbacks concurrently
                    futures::future::join_all(futures).await;
                }
            }
        });

        // Wait for the first message if mode is not WriteOnly
        if metadata.mode != AttributeMode::WriteOnly {
            let query = session.get(&att_topic).await.unwrap();
            let result = query.recv_async().await.unwrap();
            let buffer = B::build_from_zbytes(result.result().unwrap().payload().clone());
            let mut last = last_value.lock().await;
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
        let buffer: B = value.into();
        let publisher = self
            .session
            .declare_publisher(&self.cmd_topic)
            .await
            .unwrap();

        publisher.put(buffer.to_zbytes()).await.unwrap();
    }

    /// Send command and wait for validation
    pub async fn set<T>(&mut self, value: T) -> Result<(), String>
    where
        T: Into<B> + Clone,
        B: PartialEq,
    {
        let buffer: B = value.into();
        let expected_buffer = buffer.clone();


        let build = buffer.with_source(0).with_random_sequence().build().unwrap();
        self.shoot(build).await;

        // Wait for the value to be confirmed
        self.wait_for_value(
            move |received_buffer| *received_buffer == expected_buffer,
            Some(std::time::Duration::from_secs(5)),
        )
        .await?;

        Ok(())
    }

    /// Get last received value
    pub async fn get(&self) -> Option<B> {
        let last = self.last_value.lock().await;
        last.clone()
    }

    /// Wait for a specific value with optional timeout
    pub async fn wait_for_value<F>(
        &self,
        condition: F,
        timeout: Option<std::time::Duration>,
    ) -> Result<B, String>
    where
        F: Fn(&B) -> bool + Send + Sync + 'static,
    {
        // Use a broadcast channel to avoid the move issue
        let (tx, mut rx) = tokio::sync::broadcast::channel(1);

        // Add temporary callback
        let callback_id = self
            .add_callback(
                move |buffer: B| {
                    let buffer_clone = buffer.clone();
                    let tx_clone = tx.clone();
                    Box::pin(async move {
                        let _ = tx_clone.send(buffer_clone);
                    })
                },
                Some(condition),
            )
            .await;

        let result = if let Some(duration) = timeout {
            tokio::time::timeout(duration, rx.recv()).await
        } else {
            // No timeout: wait indefinitely
            Ok(rx.recv().await)
        };

        // Remove the callback
        self.remove_callback(callback_id).await;

        match result {
            Ok(Ok(buffer)) => Ok(buffer),
            Ok(Err(_)) => Err("Channel closed unexpectedly".to_string()),
            Err(_) => Err("Timeout waiting for value".to_string()),
        }
    }

    /// Add an async callback that will be triggered when receiving buffer messages
    /// Optionally, a condition can be provided to filter when the callback is triggered
    pub async fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(B) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
        C: Fn(&B) -> bool + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().await;
        let mut next_id = self.next_callback_id.lock().await;

        let callback_id = *next_id;
        *next_id += 1;

        let callback_entry = CallbackEntry {
            callback: Box::new(callback),
            condition: condition.map(|c| Box::new(c) as Box<dyn Fn(&B) -> bool + Send + Sync>),
        };

        callbacks.insert(callback_id, callback_entry);
        callback_id
    }

    /// Remove an async callback by its ID
    pub async fn remove_callback(&self, callback_id: CallbackId) -> bool {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.remove(&callback_id).is_some()
    }

    /// Clear all async callbacks
    pub async fn clear_callbacks(&self) {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.clear();
    }

    /// Get the number of registered async callbacks
    pub async fn callback_count(&self) -> usize {
        let callbacks = self.callbacks.lock().await;
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
    use super::*;
    use crate::fbs::BooleanBuffer;
    use std::time::Duration;
    use tokio::time::sleep;
    use zenoh::config::Config;
    #[tokio::test]
    async fn test_callback_registration() {
        let config = Config::default();
        let session = zenoh::open(config).await.unwrap();

        let metadata = crate::AttributeMetadata {
            topic: "test/boolean".to_string(),
            r#type: "boolean".to_string(),
            info: None,
            mode: crate::AttributeMode::ReadWrite,
        };

        let attribute: GenericAttribute<BooleanBuffer> =
            GenericAttribute::new(session, metadata).await;

        // Test adding async callback
        let callback_id = attribute
            .add_callback(
                |buffer| {
                    Box::pin(async move {
                        // Simulate async work
                        sleep(Duration::from_millis(10)).await;
                        println!("Async callback triggered with buffer: {:?}", buffer);
                    })
                },
                None::<fn(&BooleanBuffer) -> bool>,
            )
            .await;

        assert_eq!(attribute.callback_count().await, 1);

        // Test removing callback
        let removed = attribute.remove_callback(callback_id).await;
        assert!(removed);
        assert_eq!(attribute.callback_count().await, 0);
    }
    #[tokio::test]
    async fn test_callback_with_condition() {
        let config = Config::default();
        let session = zenoh::open(config).await.unwrap();

        let metadata = crate::AttributeMetadata {
            topic: "test/boolean".to_string(),
            r#type: "boolean".to_string(),
            info: None,
            mode: crate::AttributeMode::ReadWrite,
        };
        let attribute: GenericAttribute<BooleanBuffer> =
            GenericAttribute::new(session, metadata).await;

        // Test adding async callback with condition
        let condition: fn(&BooleanBuffer) -> bool = |_buffer: &BooleanBuffer| true;
        let _callback_id = attribute
            .add_callback(
                |buffer| {
                    Box::pin(async move {
                        sleep(Duration::from_millis(10)).await;
                        println!("Conditional async callback triggered: {:?}", buffer);
                    })
                },
                Some(condition), // Always trigger for this test
            )
            .await;

        assert_eq!(attribute.callback_count().await, 1);
    }
}
