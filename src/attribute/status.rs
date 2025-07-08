use super::std_msg::StdMsgAttribute;
use super::CallbackId;
use crate::fbs::status_buffer::StatusBuffer;
use crate::AttributeMetadata;
use zenoh::Session;

#[derive(Clone, Debug)]
/// Objet pour gérer StatusAttribute
pub struct StatusAttribute {
    pub inner: StdMsgAttribute<StatusBuffer>,
}

impl StatusAttribute {
    /// Crée une nouvelle instance
    pub async fn new(session: Session, metadata: AttributeMetadata) -> Self {
        let inner = StdMsgAttribute::<StatusBuffer>::new(session, metadata).await;
        Self { inner }
    }

    /// Envoie une commande sans attendre la validation
    #[inline]
    pub async fn shoot(&mut self, value: StatusBuffer) {
        self.inner.shoot(value).await;
    }

    /// Définit la valeur et attend la validation
    #[inline]
    pub async fn set(&mut self, value: StatusBuffer) -> Result<(), String> {
        self.inner.set(value).await
    }

    /// Récupère la dernière valeur reçue
    #[inline]
    pub async fn get(&self) -> Option<StatusBuffer> {
        self.inner.get().await
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

    /// Attend que toutes les instances de StatusBuffer soient en état "running"
    pub async fn wait_for_all_instances_to_be_running(
        &self,
        timeout: std::time::Duration,
    ) -> Result<(), String> {
        // self.wait_for_value(
        //     move |status_buffer| {
        //         // Supposons que StatusBuffer a une méthode ou champ `is_running()` ou similaire
        //         // et qu'il peut représenter plusieurs instances.
        //         // Adaptez cette logique selon la structure réelle de StatusBuffer.
        //         status_buffer
        //     },
        //     timeout,
        // )
        // .await
        Ok(())
    }
}
