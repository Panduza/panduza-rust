use super::std_obj::StdObjAttribute;
use super::CallbackId;
use crate::fbs::status_buffer::StatusBuffer;
use crate::AttributeMetadata;
use zenoh::Session;

#[derive(Clone, Debug)]
/// Objet pour gérer StatusAttribute
pub struct StatusAttribute {
    /// Internal implementation
    ///
    pub inner: StdObjAttribute<StatusBuffer>,
}

impl StatusAttribute {
    /// New instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        // Create inner implementation
        let inner = StdObjAttribute::<StatusBuffer>::new(session, metadata).await;

        // Return the new StatusAttribute instance
        Self { inner }
    }

    /// Attend une valeur spécifique de StatusBuffer (via un prédicat)
    #[inline]
    pub async fn wait_for_value<F>(
        &self,
        predicate: F,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String>
    where
        F: Fn(&StatusBuffer) -> bool + Send + Sync + 'static,
    {
        self.inner
            .wait_for_value(predicate, timeout)
            .await
            .map(|_| ())
    }

    /// Ajoute un callback déclenché à la réception de StatusBuffer
    #[inline]
    pub async fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(StatusBuffer) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
        C: Fn(&StatusBuffer) -> bool + Send + Sync + 'static,
    {
        self.inner.add_callback(callback, condition).await
    }

    /// Supprime un callback par son ID
    #[inline]
    pub async fn remove_callback(&self, callback_id: CallbackId) -> bool {
        self.inner.remove_callback(callback_id).await
    }

    /// Récupère les métadonnées de l'attribut
    #[inline]
    pub fn metadata(&self) -> &AttributeMetadata {
        self.inner.metadata()
    }

    // ------------------------------------------------------------------------

    /// Waits until all instances of StatusBuffer are in the "running" state
    ///
    pub async fn wait_for_all_instances_to_be_running(
        &self,
        timeout: std::time::Duration,
    ) -> Result<(), String> {
        self.inner
            .wait_for_value(
                |status_buffer| status_buffer.all_instances_are_running(),
                Some(timeout),
            )
            .await
            .map(|_| ())
    }

    // ------------------------------------------------------------------------

    /// Waits until at least one instance of StatusBuffer is not in the "running" state
    pub async fn wait_for_at_least_one_instance_to_be_not_running(
        &self,
        timeout: std::time::Duration,
    ) -> Result<(), String> {
        self.inner
            .wait_for_value(
                |status_buffer| !status_buffer.all_instances_are_running(),
                Some(timeout),
            )
            .await
            .map(|_| ())
    }

    // ------------------------------------------------------------------------
}
