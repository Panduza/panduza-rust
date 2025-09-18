use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use zenoh::Session;

use super::std_obj::StdObjAttribute;
use super::CallbackId;
use crate::fbs::StructureBuffer;
use crate::AttributeMetadata;

/// Flat structure module for HashMap representation
pub mod flat;
pub use flat::FlatStructure;

/// High-level wrapper for managing structure attributes with tree-like data representation
///
#[derive(Clone, Debug)]
pub struct StructureAttribute {
    /// Flat version of the structure to ease find algorithms
    pub flat: Arc<Mutex<FlatStructure>>,

    /// Internal generic implementation based on an already design manager
    pub inner: StdObjAttribute<StructureBuffer>,
}

impl StructureAttribute {
    // ------------------------------------------------------------------------

    /// Return the last structure value received
    pub async fn get(&self) -> Option<StructureBuffer> {
        self.inner.get().await
    }

    // ------------------------------------------------------------------------

    /// Return the last structure value received as json string
    pub async fn get_as_json_string(&self) -> Option<String> {
        if let Some(buffer) = self.inner.get().await {
            let json_value = buffer.as_json();
            Some(json_value.to_string())
        } else {
            None
        }
    }

    // ------------------------------------------------------------------------
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        let inner = StdObjAttribute::<StructureBuffer>::new(session, metadata).await;
        let flat = Arc::new(Mutex::new(FlatStructure::new()));

        let instance = Self {
            flat: flat.clone(),
            inner,
        };

        // Add callback to update flat whenever a new StructureBuffer is received
        instance
            .inner
            .add_callback(
                {
                    let flat_ref = flat.clone();
                    move |buffer: StructureBuffer| {
                        let flat_clone = flat_ref.clone();
                        Box::pin(async move {
                            let mut flat = flat_clone.lock().await;
                            flat.update_from_buffer(&buffer);
                        })
                    }
                },
                None::<fn(&StructureBuffer) -> bool>,
            )
            .await;

        // Initialize flat from current buffer if available
        if let Some(buffer) = instance.inner.get().await {
            let mut flat_guard = instance.flat.lock().await;
            flat_guard.update_from_buffer(&buffer);
        }

        instance
    }

    // ------------------------------------------------------------------------

    /// Waits for a specific StructureBuffer value matching predicate
    ///
    pub async fn wait_for_value<F>(
        &self,
        predicate: F,
        timeout: Option<Duration>,
    ) -> Result<(), String>
    where
        F: Fn(&StructureBuffer) -> bool + Send + Sync + 'static,
    {
        // Use the inner implementation and discard the actual value
        self.inner.wait_for_value(predicate, timeout).await?;
        Ok(())
    }

    // ------------------------------------------------------------------------

    /// Registers a callback triggered on StructureBuffer reception
    ///
    pub async fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(StructureBuffer) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static,
        C: Fn(&StructureBuffer) -> bool + Send + Sync + 'static,
    {
        self.inner.add_callback(callback, condition).await
    }

    // ------------------------------------------------------------------------

    /// Removes a callback by its ID
    ///
    pub async fn remove_callback(&self, callback_id: CallbackId) -> bool {
        self.inner.remove_callback(callback_id).await
    }

    // ------------------------------------------------------------------------

    /// Provides read-only access to attribute metadata
    ///
    pub fn metadata(&self) -> &AttributeMetadata {
        self.inner.metadata()
    }

    // ------------------------------------------------------------------------

    /// Use the flat field to find attributes matching the pattern
    ///
    pub async fn find_attribute<A: Into<String>>(&self, pattern: A) -> Option<AttributeMetadata> {
        let pattern_str = pattern.into();
        let flat_guard = self.flat.lock().await;

        // Use FlatStructure's find_attributes method and return the first match
        let matches = flat_guard.find_attributes(&pattern_str);
        matches.first().map(|metadata| (*metadata).clone())
    }

    // ------------------------------------------------------------------------

    /// Get a specific attribute by its exact topic path
    ///
    pub async fn get_attribute_by_topic(&self, topic: &str) -> Option<AttributeMetadata> {
        let flat_guard = self.flat.lock().await;
        flat_guard.get(topic).cloned()
    }

    // ------------------------------------------------------------------------

    /// Get all available attribute topics
    ///
    pub async fn get_all_topics(&self) -> Vec<String> {
        let flat_guard = self.flat.lock().await;
        flat_guard.get_topics().into_iter().cloned().collect()
    }

    // ------------------------------------------------------------------------

    /// Get the number of flattened attributes
    ///
    pub async fn flat_structure_len(&self) -> usize {
        let flat_guard = self.flat.lock().await;
        flat_guard.len()
    }

    // ------------------------------------------------------------------------
}
