use crate::attribute::AttributeError;
use crate::Reactor;

/// Structure publique Executor
pub struct Executor {
    pub reactor: Reactor,
}

impl Executor {
    /// Crée un nouvel Executor à partir d'un Reactor
    pub fn new(reactor: Reactor) -> Self {
        Self { reactor }
    }

    //--------------------------------------------------------------------------

    /// Retourne la structure des attributs du banc de test au format JSON
    pub async fn structure_get(&self) -> Result<String, AttributeError> {
        let structure = self.reactor.get_structure_attribute().await;
        match structure.get_as_json_string().await {
            Some(json_string) => Ok(json_string),
            None => Err(AttributeError::NotFound(
                "Structure data not available".to_string(),
            )),
        }
    }

    //--------------------------------------------------------------------------

    /// Return the value of the given boolean attribute topic
    pub async fn attribute_boolean_get(&self, topic: &String) -> Result<bool, AttributeError> {
        let attribute_builder = self.reactor.find_attribute(topic).await;
        let boolean_attribute = attribute_builder.try_into_boolean().await?;

        match boolean_attribute.get().await {
            Some(buffer) => match buffer.value() {
                Some(value) => Ok(value),
                None => Err(AttributeError::NotFound(format!(
                    "No value in buffer for boolean attribute: {}",
                    topic
                ))),
            },
            None => Err(AttributeError::NotFound(format!(
                "No buffer available for boolean attribute: {}",
                topic
            ))),
        }
    }

    //--------------------------------------------------------------------------

    /// Set the value of the given boolean attribute topic
    pub async fn attribute_boolean_set(
        &self,
        topic: &String,
        value: bool,
    ) -> Result<(), AttributeError> {
        let attribute_builder = self.reactor.find_attribute(topic).await;
        let mut boolean_attribute = attribute_builder.try_into_boolean().await?;

        match boolean_attribute.set(value).await {
            Ok(()) => Ok(()),
            Err(error_msg) => Err(AttributeError::NotFound(format!(
                "Failed to set boolean attribute {}: {}",
                topic, error_msg
            ))),
        }
    }
}
