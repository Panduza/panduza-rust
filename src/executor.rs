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

    /// Retourne la structure des attributs du banc de test au format JSON
    pub async fn structure_get(&self) -> Option<String> {
        let structure = self.reactor.get_structure_attribute().await;
        structure.get_as_json_string().await
    }
}
