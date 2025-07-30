use super::std_obj::StdObjAttribute;
use super::CallbackId;
use crate::fbs::StructureBuffer;
use crate::AttributeMetadata;
use zenoh::Session;

#[derive(Clone, Debug)]
/// Objet pour gérer StructureAttribute
pub struct StructureAttribute {
    /// Internal implementation
    ///
    pub inner: StdObjAttribute<StructureBuffer>,
}

impl StructureAttribute {
    // ------------------------------------------------------------------------

    /// New instance
    ///
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        // Create inner implementation
        let inner = StdObjAttribute::<StructureBuffer>::new(session, metadata).await;

        // Return the new StructureAttribute instance
        Self { inner }
    }

    // ------------------------------------------------------------------------

    /// Attend une valeur spécifique de StructureBuffer (via un prédicat)
    #[inline]
    pub async fn wait_for_value<F>(
        &self,
        predicate: F,
        timeout: Option<std::time::Duration>,
    ) -> Result<(), String>
    where
        F: Fn(&StructureBuffer) -> bool + Send + Sync + 'static,
    {
        self.inner
            .wait_for_value(predicate, timeout)
            .await
            .map(|_| ())
    }

    // ------------------------------------------------------------------------

    /// Ajoute un callback déclenché à la réception de StructureBuffer
    #[inline]
    pub async fn add_callback<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(StructureBuffer) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
        C: Fn(&StructureBuffer) -> bool + Send + Sync + 'static,
    {
        self.inner.add_callback(callback, condition).await
    }

    // ------------------------------------------------------------------------

    /// Supprime un callback par son ID
    #[inline]
    pub async fn remove_callback(&self, callback_id: CallbackId) -> bool {
        self.inner.remove_callback(callback_id).await
    }

    // ------------------------------------------------------------------------

    /// Récupère les métadonnées de l'attribut
    #[inline]
    pub fn metadata(&self) -> &AttributeMetadata {
        self.inner.metadata()
    }

    // ------------------------------------------------------------------------
}
